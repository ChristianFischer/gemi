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

use crate::cartridge::image_data::{ImageData, ImageDataMut, ZeroImageData};
use crate::device_type::DeviceConfig;
use crate::ppu::graphic_data::Color;


// todo: doc
pub trait EmulatorContext {
    type RomImageType: ImageData;
    type RamImageType: ImageDataMut;

    fn get_device_config(&self) -> &DeviceConfig;

    fn get_cartridge_rom(&self) -> &Self::RomImageType;
    fn get_cartridge_ram(&mut self) -> &mut Self::RamImageType;

    fn put_pixel(&mut self, x: u32, y: u32, color: Color);
}


// todo: doc
pub type ZeroEmulatorContext = DefaultEmulatorContext<ZeroImageData, ZeroImageData>;


// todo: doc
pub struct DefaultEmulatorContext<
    ROM: ImageData,
    RAM: ImageDataMut
> {
    device_config: DeviceConfig,
    cartridge_rom: ROM,
    cartridge_ram: RAM,
}


impl<ROM: ImageData + Default, RAM: ImageDataMut + Default> DefaultEmulatorContext<ROM, RAM>
{
    pub fn new(device_config: DeviceConfig) -> Self {
        Self {
            device_config,
            cartridge_rom: ROM::default(),
            cartridge_ram: RAM::default(),
        }
    }
}


impl<ROM: ImageData, RAM: ImageDataMut> EmulatorContext for DefaultEmulatorContext<ROM, RAM>
{
    type RomImageType = ROM;
    type RamImageType = RAM;

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
