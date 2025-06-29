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

use gemi_core::boot_rom::BootRom;
use gemi_core::cartridge::image_data::{ImageData, ImageDataMut};
use gemi_core::cartridge::Cartridge;
use gemi_core::device_type::DeviceConfig;
use gemi_core::emulator_context::EmulatorContext;


/// A struct holding the internal data used by the emulator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct GameBoyContextData {
    pub(crate) device_config: DeviceConfig,
    pub(crate) boot_rom:  Option<Box<BootRom>>,
    pub(crate) cartridge: Box<Cartridge>,
}


impl GameBoyContextData {
    pub(crate) fn make_context(&self) -> EmulatorContext {
        EmulatorContext::new(
            self.device_config,
            &self.cartridge.cartridge_info,
            self.cartridge.rom.get_data(),
            self.cartridge.ram.get_data(),
            self.boot_rom.as_ref().map(|boot_rom| boot_rom.as_ref())
        )
    }


    // todo: unify make + make_mut?
    pub(crate) fn make_context_mut(&mut self) -> EmulatorContext {
        EmulatorContext::new_mut(
            self.device_config,
            &self.cartridge.cartridge_info,
            self.cartridge.rom.get_data(),
            self.cartridge.ram.get_data_mut(),
            self.boot_rom.as_ref().map(|boot_rom| boot_rom.as_ref())
        )
    }
}
