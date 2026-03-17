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

use gemi_core::boot_rom::BootRom;
use gemi_core::cartridge::image_data::DataBuffer;
use gemi_core::cartridge::{Cartridge, CartridgeInfo};
use gemi_core::device_type::DeviceConfig;
use gemi_core::emulator_context::{EmulatorClient_PPU, EmulatorContext, EmulatorContextMut};
use gemi_core::ppu::graphic_data::Color;

/// A struct holding the internal data used by the emulator.
// todo: rename? move?
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct GameBoyContextData {
    pub(crate) device_config: DeviceConfig,
    pub(crate) boot_rom:  Option<Box<BootRom>>,
    pub(crate) cartridge: Box<Cartridge>,
}


impl EmulatorContext for GameBoyContextData {
    type BootRomImageData = BootRom;
    type RomImageData = DataBuffer;
    type RamImageData = DataBuffer;


    fn get_device_config(&self) -> &DeviceConfig {
        &self.device_config
    }

    fn get_cartridge_info(&self) -> &CartridgeInfo {
        &self.cartridge.cartridge_info
    }

    fn get_boot_rom(&self) -> Option<&Self::BootRomImageData> {
        self.boot_rom.as_ref().map(|b| b.as_ref())
    }

    fn get_cartridge_rom(&self) -> &Self::RomImageData {
        &self.cartridge.rom
    }

    fn get_cartridge_ram(&self) -> &Self::RamImageData {
        &self.cartridge.ram
    }
}


impl EmulatorClient_PPU for GameBoyContextData {
    fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        _ = (x, y, color); // todo: implement me
    }
}


impl EmulatorContextMut for GameBoyContextData {
    type RamImageDataMut = DataBuffer;

    fn get_cartridge_ram_mut(&mut self) -> &mut Self::RamImageDataMut {
        &mut self.cartridge.ram
    }
}


