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

use crate::apu::registers::ApuChannelRegisters;
use crate::gameboy::Clock;
use crate::utils::get_bit;


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
    pub fn from_bit(bit: bool) -> Self {
        match bit {
            false => Direction::Addition,
            true  => Direction::Subtraction,
        }
    }
}


impl FrequencySweep {
    /// Initializes a new frequency sweep timer from a channels registers.
    pub fn from_registers(registers: &ApuChannelRegisters) -> Self {
        let period        = (registers.nr0 >> 4) & 0x07;
        let shift         = (registers.nr0 >> 0) & 0x07;
        let direction_bit = get_bit(registers.nr0, 3);
        let enabled       = period != 0 || shift != 0;

        Self {
            enabled,
            shift,
            period_length:  period,
            period_timer:   period,
            direction:      Direction::from_bit(direction_bit),
        }
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
    pub fn tick(&mut self, _registers: &ApuChannelRegisters, frequency: Clock) -> FrequencySweepResult {
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

