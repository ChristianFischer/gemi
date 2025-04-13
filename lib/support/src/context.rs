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
use gemi_core::cartridge::memory_image_data::MemoryImageData;
use gemi_core::device_type::DeviceConfig;
use gemi_core::emulator_context::EmulatorContext;
use gemi_core::ppu::graphic_data::Color;


// todo: doc
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameBoyContext {
    device_config: DeviceConfig,

    // todo: use ImageData / ImageDataMut traits
    cartridge_ram: MemoryImageData,
    cartridge_rom: MemoryImageData,
}


impl GameBoyContext {
    pub fn new(device_config: DeviceConfig, rom: MemoryImageData, ram: MemoryImageData) -> Self {
        Self {
            device_config,
            cartridge_ram: ram,
            cartridge_rom: rom,
        }
    }
}


impl EmulatorContext for GameBoyContext {
    type RomImageType = MemoryImageData;
    type RamImageType = MemoryImageData;


    fn get_device_config(&self) -> &DeviceConfig {
        &self.device_config
    }


    fn get_cartridge_rom(&self) -> &Self::RomImageType {
        &self.cartridge_rom
    }


    fn get_cartridge_ram(&mut self) -> &mut Self::RamImageType {
        &mut self.cartridge_ram
    }


    fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        _ = (x, y, color);
    }
}
