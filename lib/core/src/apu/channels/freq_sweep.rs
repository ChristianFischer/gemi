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
use crate::gameboy::Clock;
use crate::utils::get_bit;


const NRX0_NON_READABLE_BITS : u8       = 0b_1000_0000;


/// The maximum possible frequency, which can be stored in 11 bits being used in NRx3 and NRx4.
/// Used to check for a possible overflow.
const MAX_FREQUENCY : Clock = 0b_0000_0111_1111_1111;


/// Whether to increment or decrement the frequency
pub enum Direction {
    Addition,
    Subtraction,
}


/// The result of a sweep update.
pub enum FrequencySweepResult {
    /// Nothing to be done.
    None,

    /// An overflow occurred and the channel has to be disabled.
    DisableChannel,

    /// The value of the frequency was changed and need to be applied.
    FrequencyChanged(Clock),
}


pub struct FrequencySweep {
    /// Flag to store whether the frequency sweep is enabled or not.
    enabled: bool,

    /// The number of bits, the previous frequency has to be shifted to get the value
    /// the frequency has to be increased or decreased.
    shift: u8,

    /// The length of each period in ticks by the frame sequencer.
    period_length: u8,

    /// The current value of the period timer, which is decreased on each tick.
    period_timer: u8,

    /// Whether to increase or decrease the frequency.
    direction: Direction,
}


impl Direction {
    /// Get the direction based on the value written into the NR10 register.
    pub fn from_register_value(value: u8) -> Self {
        match get_bit(value, 3) {
            false => Direction::Addition,
            true  => Direction::Subtraction,
        }
    }


    /// Get the value, which should be written into the NR10 register.
    pub fn to_register_value(&self) -> u8 {
        match self {
            Direction::Addition    => 0b_0000_0000,
            Direction::Subtraction => 0b_0000_1000,
        }
    }
}


impl FrequencySweep {
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


    /// Computes the new frequency based on the last one and
    fn compute_frequency(&self, wave_length: Clock) -> Clock {
        let slope_amount = wave_length >> self.shift;

        match self.direction {
            Direction::Addition    => wave_length + slope_amount,
            Direction::Subtraction => wave_length - slope_amount,
        }
    }


    /// Updates the current frequency and checks for an overflow.
    pub fn update_frequency(&mut self, frequency: Clock) -> FrequencySweepResult {
        let new_frequency = self.compute_frequency(frequency);

        // if the wavelength would overflow, the channel is turned off instead
        if new_frequency >= MAX_FREQUENCY {
            return FrequencySweepResult::DisableChannel;
        }
        else {
            // otherwise, if the slope shift is non-zero, the new wavelength should be applied
            if self.shift != 0 {
                return FrequencySweepResult::FrequencyChanged(new_frequency);
            }
        }

        return FrequencySweepResult::None;
    }


    /// Receives the periodic call from the frame sequencer.
    /// When the timer elapsed, update the frequency and check for an overflow,
    /// deliver the result to the caller.
    pub fn tick(&mut self, frequency: Clock) -> FrequencySweepResult {
        self.period_timer = self.period_timer.saturating_sub(1);

        // when the timer elapses
        if self.period_timer == 0 {
            // reload the timer
            self.reload_timer();

            if self.enabled && self.period_length != 0 {
                // update the frequency and return the new value if changed
                return self.update_frequency(frequency);
            }
        }

        FrequencySweepResult::None
    }
}


impl ChannelComponent for FrequencySweep {
    fn on_read_register(&self, number: u16) -> u8 {
        match number {
            0 => {
                    NRX0_NON_READABLE_BITS
                |   self.direction.to_register_value()
                |   ((self.shift         & 0x07) << 0)
                |   ((self.period_length & 0x07) << 4)
            },

            _ => default_on_read_register(number)
        }
    }


    fn on_write_register(&mut self, number: u16, value: u8, apu_state: &ApuState) -> TriggerAction {
        match number {
            0 => {
                let period  = (value >> 4) & 0x07;
                let shift   = (value >> 0) & 0x07;
                let enabled = period != 0 || shift != 0;

                self.enabled        = enabled;
                self.shift          = shift;
                self.period_length  = period;
                self.direction      = Direction::from_register_value(value);
            }

            _ => { }
        }

        default_on_write_register(number, value, apu_state)
    }


    fn on_trigger_event(&mut self, apu_state: &ApuState) -> TriggerAction {
        self.reload_timer();

        default_on_trigger_event(apu_state)
    }


    fn on_reset(&mut self) {
        *self = Self::default();
    }
}


impl Default for FrequencySweep {
    fn default() -> Self {
        Self {
            enabled:        false,
            shift:          0,
            period_length:  0,
            period_timer:   0,
            direction:      Direction::Addition,
        }
    }
}

