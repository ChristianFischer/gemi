/*
 * Copyright (C) 2022-2024 by Christian Fischer
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

use crate::apu::sample::{Sample, SampleResult, SampleType};

/// The DAC is the component to covert the sound generators output signal
/// into an audio wave signal.
/// The input signal is the digital value between 0x00 and 0x07 generated
/// by the sound processor. The output will be an audio sample in the range
/// between `i16::MIN / 4` and `i16::MAX / 4`, so combining all four channels
/// wont exceed the value range of i16.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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


    /// Checks whether DAC is currently enabled or not.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }


    pub fn on_reset(&mut self) {
        self.enabled = false;
    }


    /// Converts a digital input value into an audio sample value.
    pub fn convert(&self, value: u8) -> SampleResult<Sample> {
        if self.enabled {
            let safe_value   = value & 0x0f;
            let sample_value = 1.0 - ((safe_value as SampleType) / 7.5);

            SampleResult::Audio(
                Sample::new(
                    sample_value
                )
            )
        }
        else {
            SampleResult::Silence
        }
    }
}
