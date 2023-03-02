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
use crate::apu::channels::channel::ChannelComponent;
use crate::apu::channels::frequency::Frequency;
use crate::gameboy::Clock;


/// Trait for any sound generator object.
pub trait SoundGenerator : ChannelComponent {
    /// Called to create a new instance of this generator.
    fn create() -> Self;

    /// Get the generators current frequency.
    fn get_frequency(&self) -> Frequency;

    /// Changes the frequency for this generator.
    fn set_frequency(&mut self, frequency: Frequency);

    /// Called to update the internal values of the sound generator.
    /// This call wont happen periodically but when necessary on register changes
    /// or when a sound sample needs to be created.
    fn update(&mut self, cycles: Clock);

    /// Checks whether to enable or disable the channels DAC.
    fn is_dac_enabled(&self) -> bool {
        true
    }

    /// Get the current sample generated by this generator object.
    /// The generated sample is expected to be in the range of 0x00 to 0x0f.
    fn get_sample(&self, apu_state: &ApuState) -> u8;
}
