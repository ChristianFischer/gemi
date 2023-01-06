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

/// The DAC is the component to covert the sound generators output signal
/// into an audio wave signal.
/// The input signal is the digital value between 0x00 and 0x07 generated
/// by the sound processor. The output will be an audio sample in the range
/// between `i16::MIN / 4` and `i16::MAX / 4`, so combining all four channels
/// wont exceed the value range of i16.
pub struct DigitalAudioConverter {
    enabled: bool,
}


impl DigitalAudioConverter {
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }


    /// Set whether the DAC is enabled or not.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }


    /// Converts a digital input value into an audio sample value.
    pub fn convert(&self, value: u8) -> i16 {
        if self.enabled {
            let safe_value  = value & 0x0f;
            let value_max   = (i16::MAX / 4) as i32;
            let value_range = (i16::MAX / 2) as i32;

            (value_max - ((safe_value as i32) * value_range / 15)) as i16
        }
        else {
            0
        }
    }
}
