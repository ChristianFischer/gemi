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

use crate::apu::channels::channel::ChannelComponent;
use crate::apu::registers::ApuChannelRegisters;
use crate::utils::get_bit;


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
    /// Receives the periodic call from the frame sequencer.
    /// Decrease the timer on each tick. When the timer becomes zero during this operation,
    /// the channels sound generator will be disabled.
    pub fn tick(&mut self) -> bool {
        if self.length_timer != 0 && self.length_timer_enabled {
            self.length_timer = self.length_timer.saturating_sub(1);

            if self.length_timer == 0 {
                return true;
            }
        }

        false
    }
}


impl<const LENGTH_BITS: u8> ChannelComponent for LengthTimer<LENGTH_BITS> {
    fn on_register_changed(&mut self, number: u16, registers: &ApuChannelRegisters) {
        match number {
            1 => {
                self.length_timer = match LENGTH_BITS {
                    6 =>  64 - ((registers.nr1 & 0x3f) as u16),
                    8 => 256 - ((registers.nr1 & 0xff) as u16),
                    _ => unreachable!()
                };
            }

            4 => {
                self.length_timer_enabled = get_bit(registers.nr4, 6);
            }

            _ => { }
        }
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

