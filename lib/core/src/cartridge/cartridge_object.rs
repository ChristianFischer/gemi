/*
 * Copyright (C) 2025 by Christian Fischer
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
use crate::cartridge::image_data::{DataBuffer, ImageData, ImageDataMut};
use crate::cartridge::Cartridge;
use crate::utils::ioerr;
use std::io;
use std::path::Path;


// todo: rename cartridge? move to support lib?
// todo: doc
#[cfg(feature = "dyn_alloc")]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CartridgeObject {
    // todo: private?
    pub rom: Box<DataBuffer>,
    pub ram: Box<DataBuffer>, // todo: make optional?
}


#[cfg(feature = "dyn_alloc")]
impl CartridgeObject {
    /// Loads cartridge ROM and RAM images.
    pub fn with_images(rom: Box<DataBuffer>, ram: Option<Box<DataBuffer>>) -> Self {
        Self {
            rom,
            ram: ram.unwrap_or_else(|| Box::new(DataBuffer::new_empty())),
        }
    }


    /// Creates an empty cartridge with no data.
    pub fn new_empty() -> Self {
        Self {
            rom: Box::new(DataBuffer::new_empty()),
            ram: Box::new(DataBuffer::new_empty()),
        }
    }


    /// Loads a cartridge and optionally its RAM from a byte buffer.
    pub fn load_from_bytes(rom: Vec<u8>, ram: Option<Vec<u8>>) -> Self {
        Self {
            rom: Box::new(DataBuffer::new(rom)),
            ram: Box::new(
                    ram
                    .map(DataBuffer::new)
                    .unwrap_or_else(DataBuffer::new_empty)
            )
        }
    }


    /// Reads the cartridge info from the current cartridge files.
    /// This may fail when the cartridge has no data or is corrupted.
    pub fn read_cartridge_info(&self) -> ioerr::Result<Cartridge> {
        Cartridge::create_from(self.rom.as_ref())
    }


    /// Get the plain data of this cartridge.
    pub fn get_rom(&self) -> &dyn ImageData {
        self.rom.as_ref()
    }


    /// Get the RAM banks of this cartridge.
    pub fn get_ram(&self) -> &dyn ImageDataMut {
        self.ram.as_ref()
    }


    /// Get the mutable RAM banks of this cartridge.
    pub fn get_ram_mut(&mut self) -> &mut dyn ImageDataMut {
        self.ram.as_mut()
    }


    /// If the Cartridge has a battery-powered RAM, flush it's current content
    /// into any assigned source.
    /// The result value is `true`, when the RAM image was successfully written,
    /// `false` if the cartridge does not have battery-powered RAM or an
    /// `ioerr::Error` on error.
    pub fn flush_ram_if_any(&self) -> ioerr::Result<bool> {
        let cartridge_info = self.read_cartridge_info()?;

        if cartridge_info.has_ram() && cartridge_info.has_battery() {
            self
                    .get_ram()
                    .flush()
                    .map_err(|e| ioerr::Error {
                        error_code:  e.error_code,
                        source_file: e.source_file,
                        source:      Some(ioerr::Source::RamImage),
                    })
        }
        else {
            Ok(false)
        }
    }
}


#[cfg(feature = "dyn_alloc")]
#[cfg(feature = "file_io")]
impl CartridgeObject {
    pub const FILE_EXT_GB:  &'static str = "gb";
    pub const FILE_EXT_GBC: &'static str = "gbc";
    pub const FILE_EXT_RAM: &'static str = "sav";


    /// Load a cartridge from a ROM file.
    /// If a RAM file with the same name exists, it tries to load it as well.
    /// Failing to load the RAM file will cause an error, but if no RAM file
    /// exists, the cartridge will be loaded with uninitialized RAM.
    pub fn load_files_with_default_ram(rom_file: &Path) -> io::Result<Self> {
        let ram_file = rom_file.with_extension(Self::FILE_EXT_RAM);

        Self::load_files(
            rom_file,

            // only try to load the RAM file if it exists
            if ram_file.exists() {
                Some(&ram_file)
            }
            else {
                None
            }
        )
    }


    /// Loads a cartridge from a ROM file.
    /// Keeps the RAM uninitialized.
    pub fn load_file(rom_file: &Path) -> io::Result<Self> {
        Self::load_files(rom_file, None)
    }


    /// Loads a cartridge and it's RAM image from files.
    pub fn load_files(rom_file: &Path, ram_file: Option<&Path>) -> io::Result<Self> {
        // load the cartridge from the ROM file
        let rom_data = DataBuffer::load_from_file(rom_file)?;

        let ram_data = match ram_file {
            Some(path) => Some(DataBuffer::load_from_file(path)?),
            None => None,
        };

        Ok(Self {
            rom: Box::new(rom_data),
            ram: Box::new(ram_data.unwrap_or_else(DataBuffer::new_empty)),
        })
    }
}
