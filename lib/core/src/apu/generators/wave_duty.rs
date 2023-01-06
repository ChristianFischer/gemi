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

use crate::utils::get_bit;

/// Patterns to be used for each selectable wave duty.
pub const WAVE_DUTY_PATTERN: [u8; 4] = [
    0b_0000_0001, // 12.5%
    0b_0000_0011, // 25%
    0b_0000_1111, // 50%
    0b_1111_1100, // 75%
];


/// A wave duty describes the ratio between peaks and valleys of a sound wave.
/// This creates a wave pattern of alternating valleys and peaks with their
/// duration depending on the ratio between valleys and peaks.
pub struct WaveDuty {
    /// The pattern created by the wave duty.
    wave_pattern: u8,
}


impl WaveDuty {
    /// Get the wave duty by it's index.
    /// * 0 -> 12.5%
    /// * 1 -> 25%
    /// * 2 -> 50%
    /// * 3 -> 75%
    pub fn by_index(index: u8) -> Self {
        Self {
            wave_pattern: WAVE_DUTY_PATTERN[(index & 0b11) as usize],
        }
    }


    /// Get the value of the wave on a specific position.
    /// The wave contains 8 bits, so it will repeat after position 7.
    pub fn get_wave_at(&self, position: u8) -> u8 {
        get_bit(self.wave_pattern, position & 0x07) as u8
    }
}


impl Default for WaveDuty {
    fn default() -> Self {
        Self {
            wave_pattern: 0b_0000_0000,
        }
    }
}