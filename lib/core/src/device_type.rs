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


pub mod abbreviations {
    pub const DMG:  &str = "dmg";
    pub const GBC:  &str = "gbc";
    pub const GBA:  &str = "gba";
    pub const SGB:  &str = "sgb";
    pub const SGB2: &str = "sgb2";
}


/// The type of GameBoy device to be emulated.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum DeviceType {
    /// The original GameBoy with monochrome 4 color display.
    /// DMG = Dot Matrix Game
    GameBoyDmg,

    /// GameBoy Color with slightly more RAM and color support.
    GameBoyColor,

    /// GameBoy Advance
    GameBoyAdvance,

    /// Super GameBoy
    SuperGameBoy,

    /// Super GameBoy 2
    SuperGameBoy2,
}


/// Depending on the device and ROM being emulated, the type of
/// emulation running. For example, the GameBoy Color hardware may
/// run in DMG compatibility mode when a ROM without GBC support
/// is played.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum EmulationType {
    /// Classic GameBoy or compatibility mode.
    DMG,

    /// GameBoy Color support enabled.
    GBC,
}


impl DeviceType {

    /// List of all valid devices.
    pub const ALL_DEVICES: [DeviceType; 5] = [
        DeviceType::GameBoyDmg,
        DeviceType::GameBoyColor,
        DeviceType::GameBoyAdvance,
        DeviceType::SuperGameBoy,
        DeviceType::SuperGameBoy2,
    ];

    /// A list of all devices with GameBoy Color compatibility.
    pub const GBC_DEVICES: [DeviceType; 2] = [
        DeviceType::GameBoyColor,
        DeviceType::GameBoyAdvance,
    ];


    /// Get the device type matching the given abbreviation.
    /// Returns `None` if the abbreviation is invalid.
    pub fn from_abbreviation(abbreviation: &str) -> Option<DeviceType> {
        match abbreviation {
            abbreviations::DMG  => Some(DeviceType::GameBoyDmg),
            abbreviations::GBC  => Some(DeviceType::GameBoyColor),
            abbreviations::GBA  => Some(DeviceType::GameBoyAdvance),
            abbreviations::SGB  => Some(DeviceType::SuperGameBoy),
            abbreviations::SGB2 => Some(DeviceType::SuperGameBoy2),
            _                   => None,
        }
    }


    /// Get an abbreviation for the device type.
    pub fn get_abbreviation(&self) -> &'static str {
        match self {
            DeviceType::GameBoyDmg     => abbreviations::DMG,
            DeviceType::GameBoyColor   => abbreviations::GBC,
            DeviceType::GameBoyAdvance => abbreviations::GBA,
            DeviceType::SuperGameBoy   => abbreviations::SGB,
            DeviceType::SuperGameBoy2  => abbreviations::SGB2,
        }
    }


    /// Checks whether this device type has GameBoy Color support.
    pub fn has_gbc_support(&self) -> bool {
        match self {
            DeviceType::GameBoyDmg     => false,
            DeviceType::GameBoyColor   => true,
            DeviceType::GameBoyAdvance => true,
            DeviceType::SuperGameBoy   => false,
            DeviceType::SuperGameBoy2  => false,
        }
    }


    /// Get a string representation of the device type.
    pub fn to_string(&self) -> &'static str {
        match self {
            DeviceType::GameBoyDmg     => "GameBoyDmg",
            DeviceType::GameBoyColor   => "GameBoyColor",
            DeviceType::GameBoyAdvance => "GameBoyAdvance",
            DeviceType::SuperGameBoy   => "SuperGameBoy",
            DeviceType::SuperGameBoy2  => "SuperGameBoy2",
        }
    }
}
