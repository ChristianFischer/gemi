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
use crate::context::GameBoyContext;
use crate::GameBoy;
use gemi_core::boot_rom::BootRom;
use gemi_core::cartridge::memory_image_data::MemoryImageData;
use gemi_core::cartridge::GameBoyColorSupport;
use gemi_core::device_type::{DeviceConfig, DeviceType, EmulationType};
use gemi_core::emulator_device::EmulatorDevice;
use std::fmt::{Display, Formatter};

#[cfg(feature = "file_io")]
use std::{
    io,
    path::Path
};

#[cfg(feature = "file_io")]
use gemi_core::cartridge::file_image_data::FileImageData;
use gemi_core::cartridge::image_data::ImageData;

/// A factory class to construct a GameBoy device object.
/// Usually created via GameBoy::build()
pub struct Builder {
    boot_rom:       Option<BootRom>,
    cartridge_ram:  Option<MemoryImageData>,
    cartridge_rom:  Option<MemoryImageData>,
    device_type:    Option<DeviceType>,
}


/// Error codes occurred during creating an emulator instance.
#[derive(Debug)]
pub enum BuilderErrorCode {
    GameBoyColorNotSupported,
}


#[cfg(feature = "file_io")]
impl Builder {
    pub const FILE_EXT_GB:  &'static str = "gb";
    pub const FILE_EXT_GBC: &'static str = "gbc";
    pub const FILE_EXT_RAM: &'static str = "sav";


    /// Load a cartridge from a ROM file.
    /// If a RAM file with the same name exists, it tries to load it as well.
    /// Failing to load the RAM file will cause an error, but if no RAM file
    /// exists, the cartridge will be loaded with uninitialized RAM.
    pub fn load_files_with_default_ram(rom_file: &Path) -> io::Result<Builder> {
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
    #[cfg(feature = "file_io")]
    pub fn load_file(rom_file: &Path) -> io::Result<Builder> {
        Self::load_files(rom_file, None)
    }


    /// Loads a cartridge and it's RAM image from files.
    #[cfg(feature = "file_io")]
    pub fn load_files(rom_file: &Path, ram_file: Option<&Path>) -> io::Result<Builder> {
        // load the cartridge from the ROM file
        let rom_data = FileImageData::load(rom_file)?;

        let ram_data = match ram_file {
            Some(path) => Some(FileImageData::load(path)?),
            None => None,
        };

        let mut builder = Self::new();


        Ok(Self {
            boot_rom:       None,
            cartridge_rom:  Some(MemoryImageData::new(rom_data.get_data().to_vec())),
            cartridge_ram:  ram_data.map(|d| MemoryImageData::new(d.get_data().to_vec())),
            device_type:    None,
        })
    }
}


impl Builder {
    /// Creates a new empty GameBoy builder
    pub fn new() -> Self {
        Self {
            boot_rom:       None,
            cartridge_rom:  None,
            cartridge_ram:  None,
            device_type:    None,
        }
    }


    /// Set the boot ROM, which will be executed before the actual ROM.
    pub fn set_boot_rom(&mut self, boot_rom: BootRom) {
        self.boot_rom = Some(boot_rom);
    }


    /// Set the cartridge ROM image to be used.
    pub fn set_cartridge_rom(&mut self, rom: MemoryImageData) {
        self.cartridge_rom = Some(rom);
    }


    /// Override the preferred device type.
    /// If not specified, the device type will be determined by the cartridge type.
    pub fn set_device_type(&mut self, device_type: DeviceType) {
        self.device_type = Some(device_type);
    }


    /// Get the preferred device type, which is either specified explicitly
    /// or selected by the cartridge properties.
    pub fn select_preferred_device_type(&self) -> DeviceType {
        // explicit type will be preferred
        if let Some(device_type) = &self.device_type {
            return *device_type;
        }

        // determine the preferred device type by the cartridge properties
        if let Some(cartridge) = &self.cartridge_rom {
            return match cartridge.get_cgb_support() {
                GameBoyColorSupport::None      => DeviceType::GameBoyDmg,
                GameBoyColorSupport::Supported => DeviceType::GameBoyColor,
                GameBoyColorSupport::Required  => DeviceType::GameBoyColor,
            };
        }

        // default to classic GameBoy
        DeviceType::GameBoyDmg
    }


    /// Check the emulation type based on the selected device and GameBoyColor
    /// support of the selected cartridge.
    pub fn select_emulation_type(&self, device_type: &DeviceType) -> EmulationType {
        match device_type {
            DeviceType::GameBoyDmg => {}
            _ => {
                if let Some(cartridge) = &self.cartridge {
                    if cartridge.supports_cgb() {
                        return EmulationType::GBC;
                    }
                }
            }
        }

        EmulationType::DMG
    }


    /// Build the GameBoy device emulator based on the properties specified with this builder.
    pub fn finish(mut self) -> Result<GameBoy, BuilderErrorCode> {
        // select the preferred device type based on the current config and cartridge
        let device_type    = self.select_preferred_device_type();
        let emulation_type = self.select_emulation_type(&device_type);

        // setup device config based on the current configuration
        let device_config = DeviceConfig {
            device: device_type,
            emulation: emulation_type,
        };
        
        // todo: replace with zero?
        // take the ROM objects from the builder, or create default images
        let rom_data = self.cartridge_rom.take().unwrap_or_else(|| MemoryImageData::alloc(0));
        let ram_data = self.cartridge_ram.take().unwrap_or_else(|| MemoryImageData::alloc(0));

        // setup the emulator context
        let mut context = GameBoyContext::new(
            device_config,
            rom_data,
            ram_data
        );

        // construct the GameBoy object
        let mut emulator = Box::new(
            EmulatorDevice::new(&mut context)
        );

        // set boot ROM, if any
        if let Some(boot_rom) = self.boot_rom.take() {
            emulator.get_peripherals_mut().mem.set_boot_rom(boot_rom);
        }

        Ok(GameBoy {
            context,
            emulator,
        })
    }
}


impl Display for BuilderErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to build emulator")
    }
}
