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

use crate::utils::SerializableArray;
use std::fs::File;
use std::io;
use std::io::Read;

/// A data object containing a 256 byte boot ROM.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BootRom {
    rom: Box<SerializableArray<u8, 256>>,
}


impl BootRom {
    /// Load a boot rom from a file.
    pub fn load_file(filepath: &String) -> Result<BootRom, io::Error> {
        let mut file = File::open(filepath)?;
        let metadata  = file.metadata()?;
        let file_size = metadata.len();

        // fail when the boot rom image has an unexpected size
        if file_size != 256 {
            let msg = format!(
                "Unexpected Boot ROM size: {} is {} bytes, expected: 256 bytes",
                *filepath,
                file_size
            );

            return Err(io::Error::new(io::ErrorKind::Other, msg));
        }

        let mut buffer = [0u8; 256];
        file.read(&mut buffer)?;

        Ok(BootRom {
            rom: Box::new(buffer.into())
        })
    }

    /// Get data from the boot ROM.
    pub fn read(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }
}
