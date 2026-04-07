/*
 * Copyright (C) 2025-2026 by Christian Fischer
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
use crate::cartridge::{CartridgeInfo, DataBuffer};
use crate::core::cartridge::image_data::ImageDataMut;
use crate::core::utils::ioerr;

#[cfg(feature = "file_io")]
use std::{io, path::Path};


/// Represents a game cartridge containing a RAM image and ROM image.
///
/// Even if RAM is optional for Cartridges, this struct is expected to always contain a RAM buffer
/// to simplify the handling of cartridge data and avoid unnecessary checks.
///
/// If no RAM is supported by a certain cartridge, it is expected to contain an empty RAM buffer.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cartridge {
    cartridge_info: CartridgeInfo,
    rom: Box<DataBuffer>,
    ram: Box<DataBuffer>,
}


impl Cartridge {
    /// Loads cartridge ROM and RAM images.
    pub fn with_images(rom: Box<DataBuffer>, ram: Option<Box<DataBuffer>>) -> ioerr::Result<Self> {
        let cartridge_info = CartridgeInfo::create_from(rom.as_ref())?;

        let ram = ram.unwrap_or_else(|| Box::new(
            // create an empty RAM buffer, if not already provided
            if cartridge_info.has_ram() {
                let mut buffer = DataBuffer::alloc(cartridge_info.get_ram_size());

                // if we know the file source of the ROM, set an according RAM file path
                if let Some(rom_file_path) = rom.get_file_path() {
                    let ram_file_path = rom_file_path.with_extension(Self::FILE_EXT_RAM);
                    buffer.set_file_path(ram_file_path);
                }

                buffer
            }
            else {
                DataBuffer::new_empty()
            }
        ));

        Ok(
            Self {
                cartridge_info,
                rom,
                ram,
            }
        )
    }


    /// Creates an empty cartridge with no data.
    pub fn new_empty() -> Self {
        Self {
            cartridge_info: CartridgeInfo::new_empty(),
            rom: Box::new(DataBuffer::new_empty()),
            ram: Box::new(DataBuffer::new_empty()),
        }
    }


    /// Loads a cartridge and optionally its RAM from a byte buffer.
    pub fn load_from_bytes(rom: Vec<u8>, ram: Option<Vec<u8>>) -> ioerr::Result<Self> {
        Self::with_images(
            Box::new(DataBuffer::new(rom)),
            ram.map(|v| Box::new(DataBuffer::new(v)))
        )
    }


    /// Get the [CartridgeInfo] of this cartridge object.
    pub fn get_cartridge_info(&self) -> &CartridgeInfo {
        &self.cartridge_info
    }


    /// Get the plain data of this cartridge.
    pub fn get_rom(&self) -> &DataBuffer {
        self.rom.as_ref()
    }


    /// Get the RAM banks of this cartridge.
    pub fn get_ram(&self) -> &DataBuffer {
        self.ram.as_ref()
    }


    /// Get the mutable RAM banks of this cartridge.
    pub fn get_ram_mut(&mut self) -> &mut DataBuffer {
        self.ram.as_mut()
    }


    /// If the Cartridge has a battery-powered RAM, flush it's current content
    /// into any assigned source.
    /// The result value is `true`, when the RAM image was successfully written,
    /// `false` if the cartridge does not have battery-powered RAM or an
    /// `ioerr::Error` on error.
    pub fn flush_ram_if_any(&self) -> ioerr::Result<bool> {
        if self.cartridge_info.has_ram() && self.cartridge_info.has_battery() {
            self
                    .get_ram()
                    .flush()
                    .map_err(|e| ioerr::Error {
                        error_code:  e.error_code,
                        #[cfg(feature = "file_io")]
                        source_file: e.source_file,
                        source:      Some(ioerr::Source::RamImage),
                    })
        }
        else {
            Ok(false)
        }
    }
}


#[cfg(feature = "file_io")]
impl Cartridge {
    pub const FILE_EXT_GB:  &'static str = "gb";
    pub const FILE_EXT_GBC: &'static str = "gbc";
    pub const FILE_EXT_RAM: &'static str = "ram";


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
        let rom_data = Box::new(DataBuffer::load_from_file(rom_file)?);

        let ram_data = match ram_file {
            Some(path) => Some(Box::new(DataBuffer::load_from_file(path)?)),
            None => None,
        };

        Self::with_images(rom_data, ram_data)
                .map_err(|e| io::Error::other(Box::new(e)))
    }
}
