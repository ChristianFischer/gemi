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

use crate::cartridge::image_data::{ImageData, ImageDataMut, MutableRefImageData, RefImageData, ZeroImageData};
use crate::device_type::DeviceConfig;


// todo: doc
pub struct EmulatorContext<'a> {
    device_config: DeviceConfig,
    cartridge_rom: RefImageData<'a>,
    cartridge_ram: MutableRefImageData<'a>,
}


// todo: what to do here????
pub struct EmulatorContextDataHolder<Rom, Ram>
    where Rom: ImageData,
          Ram: ImageDataMut
{
    pub device_config: DeviceConfig,
    pub cartridge_rom: Rom,
    pub cartridge_ram: Ram,
}


impl<'a> EmulatorContext<'a> {
    pub fn new(device_config: DeviceConfig, rom_data: &'a [u8], ram_data: &'a mut [u8]) -> Self {
        Self {
            device_config,
            cartridge_rom: RefImageData::new(rom_data),
            cartridge_ram: MutableRefImageData::new(ram_data),
        }
    }


    pub fn get_device_config(&self) -> &DeviceConfig {
        &self.device_config
    }


    pub fn get_cartridge_rom(&self) -> &RefImageData<'a> {
        &self.cartridge_rom
    }


    pub fn get_cartridge_ram(&mut self) -> &mut MutableRefImageData<'a> {
        &mut self.cartridge_ram
    }
}


impl<Rom, Ram> EmulatorContextDataHolder<Rom, Ram>
    where Rom: ImageData,
          Ram: ImageDataMut
{
    pub fn new(device_config: DeviceConfig, rom_data: Rom, ram_data: Ram) -> Self {
        Self {
            device_config,
            cartridge_rom: rom_data,
            cartridge_ram: ram_data,
        }
    }


    pub fn make_context(&mut self) -> EmulatorContext {
        EmulatorContext::new(
            self.device_config,
            self.cartridge_rom.get_data(),
            self.cartridge_ram.get_data_mut()
        )
    }
}


impl EmulatorContextDataHolder<ZeroImageData, ZeroImageData> {
    pub fn new_empty(device_config: DeviceConfig) -> Self {
        Self {
            device_config,
            cartridge_rom: ZeroImageData::new(),
            cartridge_ram: ZeroImageData::new(),
        }
    }
}

/*
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
*/


/*
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
*/
