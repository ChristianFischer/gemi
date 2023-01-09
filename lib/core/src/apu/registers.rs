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

/// All registers used by the APU.
/// Contains registers used by the sound channels and the global audio control registers.
#[derive(Default)]
#[repr(packed(1))]
pub struct ApuRegisters {
    /// Registers to control each of the four audio channels and sound generators.
    pub channels: [ApuChannelRegisters; 4],

    /// Controls the master volume for left and right output channels.
    /// * Bit 0-2 Right output volume
    /// * Bit 4-6 Left output volume
    pub nr50: u8,

    /// Controls for each channel whether to mix it into left or right output channel.
    /// * Bit 0-3: Output on left channel for each audio channel
    /// * Bit 4-7: Output on right channel for each audio channel
    pub nr51: u8,

    /// * Bit 7 enables or disables the APU.
    /// * Bit 0-3 reports which audio channel sound generator is enabled or not (read only)
    pub nr52: u8,

    _unused_0x27: [u8; 9],

    pub wave_ram: WaveRam,
}


/// An alias to a 16 byte array containing the wave RAM for channel 3.
#[derive(Default)]
pub struct WaveRam(pub [u8; 16]);


/// Contains the registers for a single APU audio channel.
/// There are five registers for each channel, but channel 2 and 4 have no channel
/// specific feature register, so NR20 and NR40 do not exist.
#[derive(Default)]
#[repr(packed(1))]
pub struct ApuChannelRegisters {
    /// NRx0: channel specific features
    pub nr0: u8,

    /// NRx1: length timer
    pub nr1: u8,

    /// NRx2: initial volume and envelope timer
    pub nr2: u8,

    /// NRx3: wavelength
    pub nr3: u8,

    /// NRx4: trigger bit and length timer enabled bit
    pub nr4: u8,
}
