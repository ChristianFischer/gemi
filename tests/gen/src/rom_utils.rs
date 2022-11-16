/*
 * Copyright (C) 2022 by Christian Fischer
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

use std::path::PathBuf;
use gbemu_core::gameboy::DeviceType;


/// Checks whether a file contains a ROM and delivers the DeviceType
/// suitable to run the ROM.
pub fn file_is_rom(file: &PathBuf) -> Option<DeviceType> {
    match file.extension().map(|o| o.to_str().unwrap()) {
        Some("gb")  => Some(DeviceType::GameBoyDmg),
        Some("gbc") => Some(DeviceType::GameBoyColor),
        _ => None
    }
}