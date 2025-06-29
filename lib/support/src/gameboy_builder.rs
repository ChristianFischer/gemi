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
use crate::context_data::GameBoyContextData;
use crate::GameBoy;
use gemi_core::boot_rom::BootRom;
use gemi_core::cartridge::{Cartridge, CartridgeInfo};
use gemi_core::device_type::{DeviceConfig, DeviceType, EmulationType};
use gemi_core::emulator_device::EmulatorDevice;
use gemi_core::utils::ioerr;
use std::fmt::{Display, Formatter};


/// A factory class to construct a GameBoy device object.
/// Usually created via GameBoy::build()
pub struct Builder {
    boot_rom:       Option<Box<BootRom>>,
    cartridge:      Option<Box<Cartridge>>,
    device_type:    Option<DeviceType>,
}


/// Error codes occurred during creating an emulator instance.
#[derive(Debug)]
pub enum BuilderErrorCode {
    IoError(ioerr::Error),
    
    GameBoyColorNotSupported,
}


impl Builder {
    /// Creates a new empty GameBoy builder
    pub fn new() -> Self {
        Self {
            boot_rom:       None,
            cartridge:      None,
            device_type:    None,
        }
    }


    /// Set the boot ROM, which will be executed before the actual ROM.
    pub fn set_boot_rom(&mut self, boot_rom: BootRom) {
        self.boot_rom = Some(boot_rom.into());
    }


    /// Set the cartridge, which ROM will be executed.
    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge.into());
    }


    /// Override the preferred device type.
    /// If not specified, the device type will be determined by the cartridge type.
    pub fn set_device_type(&mut self, device_type: DeviceType) {
        self.device_type = Some(device_type);
    }


    /// Get the preferred device type, which is either specified explicitly
    /// or selected by the cartridge properties.
    pub fn select_preferred_device_type(&self, cartridge_info: &CartridgeInfo) -> DeviceType {
        // explicit type will be preferred
        if let Some(device_type) = &self.device_type {
            return *device_type;
        }

        // determine the preferred device type by the cartridge properties
        if cartridge_info.supports_cgb() {
            return DeviceType::GameBoyColor;
        }

        // default to classic GameBoy
        DeviceType::GameBoyDmg
    }


    /// Check the emulation type based on the selected device and GameBoyColor
    /// support of the selected cartridge.
    pub fn select_emulation_type(&self, cartridge_info: &CartridgeInfo, device_type: &DeviceType) -> EmulationType {
        match device_type {
            DeviceType::GameBoyDmg => {}
            _ => {
                if cartridge_info.supports_cgb() {
                    return EmulationType::GBC;
                }
            }
        }

        EmulationType::DMG
    }


    /// Build the GameBoy device emulator based on the properties specified with this builder.
    pub fn finish(mut self) -> Result<GameBoy, BuilderErrorCode> {
        // take the Boot ROM object from the builder or create default images
        let boot_rom = self.boot_rom.take();

        // get the cartridge object or create an empty one
        let cartridge = self.cartridge.take()
                .unwrap_or_else(|| Box::new(Cartridge::new_empty()));
        
        // get cartridge info
        let cartridge_info = cartridge.get_cartridge_info();

        // select the preferred device type based on the current config and cartridge
        let device_type    = self.select_preferred_device_type(cartridge_info);
        let emulation_type = self.select_emulation_type(cartridge_info, &device_type);

        // setup device config based on the current configuration
        let device_config = DeviceConfig {
            device: device_type,
            emulation: emulation_type,
        };

        // setup the emulator context
        let context_data = GameBoyContextData {
            device_config,
            boot_rom,
            cartridge,
        };

        // construct the GameBoy object
        let context  = context_data.make_context();
        let emulator = Box::new(
            EmulatorDevice::new(&context)
        );

        Ok(GameBoy {
            context_data,
            emulator,
        })
    }
}


impl Display for BuilderErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to build emulator")
    }
}
