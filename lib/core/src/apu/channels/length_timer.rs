/*
 * Copyright (C) 2022-2023 by Christian Fischer
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::apu::apu::ApuState;
use crate::apu::channels::channel::{ChannelComponent, TriggerAction, default_on_trigger_event, default_on_write_register, default_on_read_register};
use crate::utils::{as_bit_flag, get_bit};


/// A counter to disable a sound generator after a specific time period.
/// The timer reads its value from the NRx1 register of each channel.
/// A generic parameter controls how much bits from NRx1 will be used as the length timer,
/// which should be 8 for CH3 and 6 for all other channels.
/// Each frame sequencer tick the timer is decreased. Once it reaches zero, the channel's
/// sound generator will be disabled. Initializing the timer with zero will NOT disable
/// the sound generator.
pub struct LengthTimer<const LENGTH_BITS: u8> {
    /// Flag whether the length timer is enabled or not.
    length_timer_enabled: bool,

    /// The current timer value.
    length_timer: u16,
}


impl<const LENGTH_BITS: u8> LengthTimer<LENGTH_BITS> {
    pub const LENGTH_MAX  : u16 = 1 << LENGTH_BITS;
    pub const LENGTH_MASK : u8  = (Self::LENGTH_MAX - 1) as u8;

    /// Receives the periodic call from the frame sequencer.
    /// Decrease the timer on each tick. When the timer becomes zero during this operation,
    /// the channels sound generator will be disabled.
    pub fn tick(&mut self) -> TriggerAction {
        if self.length_timer != 0 && self.length_timer_enabled {
            self.length_timer = self.length_timer.saturating_sub(1);

            if self.length_timer == 0 {
                return TriggerAction::DisableChannel;
            }
        }

        TriggerAction::None
    }
}


impl<const LENGTH_BITS: u8> ChannelComponent for LengthTimer<LENGTH_BITS> {
    fn on_read_register(&self, number: u16) -> u8 {
        match number {
            1 => {
                // length timer initial value is write-only,
                // therefor set all bits to '1' on reading
                Self::LENGTH_MASK
            },

            4 => {
                as_bit_flag(self.length_timer_enabled, 6)
            },

            _ => default_on_read_register(number)
        }
    }


    fn on_write_register(&mut self, number: u16, value: u8, apu_state: &ApuState) -> TriggerAction {
        match number {
            1 => {
                self.length_timer = Self::LENGTH_MAX - ((value & Self::LENGTH_MASK) as u16);
            }

            4 => {
                let was_enabled = self.length_timer_enabled;

                self.length_timer_enabled = get_bit(value, 6);

                // extra clock if the timer got enabled and the sound length component
                // will not be active in the next frame sequencer step.
                if
                        !was_enabled
                    &&  self.length_timer_enabled
                    &&  self.length_timer != 0
                    &&  !apu_state.fs.is_length_timer_active()
                {
                    // this may also disable the channel
                    return self.tick();
                }
            }

            _ => { }
        }

        default_on_write_register(number, value, apu_state)
    }


    fn on_trigger_event(&mut self, apu_state: &ApuState) -> TriggerAction {
        // length timer will be set to maximum, if zero when triggered
        // this prevents the timer to be stuck at zero
        if self.length_timer == 0 {
            self.length_timer = Self::LENGTH_MAX;

            // when the sound length timer will not be activated in the next step,
            // this will cause an extra tick
            if !apu_state.fs.is_length_timer_active() {
                self.tick();
            }
        }

        default_on_trigger_event(apu_state)
    }


    fn on_reset(&mut self) {
        *self = Self::default();
    }
}


impl<const LENGTH_BITS: u8> Default for LengthTimer<LENGTH_BITS> {
    fn default() -> Self {
        Self {
            length_timer_enabled: false,
            length_timer: 0,
        }
    }
}

