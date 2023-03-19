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
use crate::apu::channels::frequency::Frequency;
use crate::utils::{as_bit_flag, get_bit};


const NRX0_NON_READABLE_BITS : u8 = 0b_1000_0000;


/// The result of a sweep update.
pub enum FrequencySweepResult {
    /// Nothing to be done.
    None,

    /// An overflow occurred and the channel has to be disabled.
    DisableChannel,

    /// The value of the frequency was changed and need to be applied.
    FrequencyChanged(Frequency),
}


pub struct FrequencySweep {
    /// Flag to store whether the frequency sweep is enabled or not.
    enabled: bool,

    /// The shadow frequency, which is a copy of the channel frequency at the moment
    /// of the trigger event and used for frequency calculation. Even if the channel's
    /// frequency gets updated via registers, the shadow register wont get updated
    /// until the next trigger event.
    shadow_frequency: Frequency,

    /// The number of bits, the previous frequency has to be shifted to get the value
    /// the frequency has to be increased or decreased.
    shift: u8,

    /// The length of each period in ticks by the frame sequencer.
    period_length: u8,

    /// The current value of the period timer, which is decreased on each tick.
    period_timer: u8,

    /// Whether to increase or decrease the frequency.
    subtract_mode: bool,

    /// Flag to store whether a subtraction was applied.
    subtraction_flag: bool,
}


impl FrequencySweep {
    /// Initializes the shadow frequency with the channels original frequency
    /// at the beginning of a trigger event.
    pub fn init_shadow_frequency(&mut self, frequency: Frequency) {
        self.shadow_frequency = frequency;
    }


    /// Reloads the timer once it reached zero.
    fn reload_timer(&mut self) {
        // if period length is zero, the value 8 is used instead
        self.period_timer = if self.period_length != 0 {
            self.period_length
        }
        else {
            8
        };
    }


    /// Computes the new frequency based on the last one and checks if the calculation
    /// did produce an overflow.
    fn compute_frequency(&mut self) -> Option<Frequency> {
        let slope_amount = self.shadow_frequency.get_value() >> self.shift;

        let new_value = if self.subtract_mode {
            self.subtraction_flag = true;
            self.shadow_frequency.get_value() - slope_amount
        }
        else {
            self.shadow_frequency.get_value() + slope_amount
        };

        Frequency::new(new_value)
    }


    /// Performs the frequency calculation and checks if the value would overflow.
    fn would_overflow(&mut self) -> bool {
        match self.compute_frequency() {
            Some(_) => false,
            None    => true,
        }
    }


    /// Updates the current frequency and checks for an overflow.
    fn update_frequency(&mut self) -> FrequencySweepResult {
        match self.compute_frequency() {
            None => {
                // if the wavelength would overflow, the channel is turned off instead
                FrequencySweepResult::DisableChannel
            }

            Some(new_frequency) => {
                // otherwise, if the slope shift is non-zero, the new frequency should be applied
                if self.shift != 0 {
                    self.shadow_frequency = new_frequency;

                    // disable channel, if frequency would overflow next calculation
                    if self.would_overflow() {
                        FrequencySweepResult::DisableChannel
                    }
                    else {
                        FrequencySweepResult::FrequencyChanged(new_frequency)
                    }
                }
                else {
                    return FrequencySweepResult::None
                }
            }
        }
    }


    /// Receives the periodic call from the frame sequencer.
    /// When the timer elapsed, update the frequency and check for an overflow,
    /// deliver the result to the caller.
    pub fn tick(&mut self) -> FrequencySweepResult {
        self.period_timer = self.period_timer.saturating_sub(1);

        // when the timer elapses
        if self.period_timer == 0 {
            // reload the timer
            self.reload_timer();

            if self.enabled && self.period_length != 0 {
                // update the frequency and return the new value if changed
                return self.update_frequency();
            }
        }

        FrequencySweepResult::None
    }
}


impl ChannelComponent for FrequencySweep {
    fn on_read_register(&self, number: u16, apu_state: &ApuState) -> u8 {
        match number {
            0 => {
                    NRX0_NON_READABLE_BITS
                |   as_bit_flag(self.subtract_mode, 3)
                |   ((self.shift         & 0x07) << 0)
                |   ((self.period_length & 0x07) << 4)
            },

            _ => default_on_read_register(number, apu_state)
        }
    }


    fn on_write_register(&mut self, number: u16, value: u8, apu_state: &ApuState) -> TriggerAction {
        match number {
            0 => {
                self.shift          = (value >> 0) & 0x07;
                self.period_length  = (value >> 4) & 0x07;
                self.subtract_mode  = get_bit(value, 3);

                // exit negate mode after calculation will disable the channel
                if self.subtraction_flag && !self.subtract_mode {
                    return TriggerAction::DisableChannel;
                }
            }

            _ => { }
        }

        default_on_write_register(number, value, apu_state)
    }


    fn on_trigger_event(&mut self, apu_state: &ApuState) -> TriggerAction {
        // reload the timer
        self.reload_timer();

        // reset subtraction flag
        self.subtraction_flag = false;

        // frequency sweep gets enabled or disabled if either length or shift is non-zero
        self.enabled = self.period_length != 0 || self.shift != 0;

        // if shift is non-zero, the overflow check is applied immediately
        if self.shift != 0 {
            if self.would_overflow() {
                return TriggerAction::DisableChannel;
            }
        }

        default_on_trigger_event(apu_state)
    }


    fn on_reset(&mut self, _apu_state: &ApuState) {
        *self = Self::default();
    }
}


impl Default for FrequencySweep {
    fn default() -> Self {
        Self {
            enabled:            false,
            shadow_frequency:   Frequency::default(),
            shift:              0,
            period_length:      0,
            period_timer:       0,
            subtract_mode:      false,
            subtraction_flag:   false,
        }
    }
}

