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

use crate::cartridge::image_data::{ImageData, ImageDataMut, ZeroImageData};
use crate::cartridge::CartridgeInfo;
use crate::device_type::DeviceConfig;
use crate::ppu::graphic_data::Color;

// todo: doc
// todo: rename into EmulatorClient
pub trait EmulatorContext {
    type BootRomImageData: ImageData;
    type RomImageData: ImageData;
    type RamImageData: ImageData;


    fn get_device_config(&self) -> &DeviceConfig;

    fn get_cartridge_info(&self) -> &CartridgeInfo;

    fn get_boot_rom(&self) -> Option<&Self::BootRomImageData>;

    fn get_cartridge_rom(&self) -> &Self::RomImageData;

    fn get_cartridge_ram(&self) -> &Self::RamImageData;
}


// todo: doc
// todo: find better name; move into PPU module
pub trait EmulatorClient_PPU {
    fn put_pixel(&mut self, x: u32, y: u32, color: Color);
}


// todo: doc
pub trait EmulatorContextMut : EmulatorContext + EmulatorClient_PPU
{
    type RamImageDataMut: ImageDataMut;

    fn get_cartridge_ram_mut(&mut self) -> &mut Self::RamImageDataMut;
}



// todo: what to do here???? rename into MockEmulatorContext?
pub struct EmulatorContextDataHolder<Rom, Ram, BootRom>
    where Rom: ImageData,
          Ram: ImageDataMut,
          BootRom: ImageData,
{
    // todo: all optional? (except device_config)
    pub device_config:  DeviceConfig,
    pub cartridge_info: CartridgeInfo,
    pub cartridge_rom:  Rom,
    pub cartridge_ram:  Ram,
    pub boot_rom:       Option<BootRom>,
}



impl<Rom, Ram, BootRom> EmulatorContextDataHolder<Rom, Ram, BootRom>
    where Rom: ImageData,
          Ram: ImageDataMut,
          BootRom: ImageData,
{
    pub fn new(device_config: DeviceConfig, rom_data: Rom, ram_data: Ram) -> Self {
        Self {
            device_config,
            cartridge_info: CartridgeInfo::create_from(&rom_data).unwrap(), // todo: remove unwrap
            cartridge_rom: rom_data,
            cartridge_ram: ram_data,
            boot_rom: None,
        }
    }
}


impl EmulatorContextDataHolder<ZeroImageData, ZeroImageData, ZeroImageData> {
    pub fn new_empty(device_config: DeviceConfig) -> Self {
        Self {
            device_config,
            cartridge_info: CartridgeInfo::default(),
            cartridge_rom: ZeroImageData::new(),
            cartridge_ram: ZeroImageData::new(),
            boot_rom: None,
        }
    }
}


impl<Rom, Ram, BootRom> EmulatorContext for EmulatorContextDataHolder<Rom, Ram, BootRom>
where Rom: ImageData,
      Ram: ImageDataMut,
      BootRom: ImageData
{
    type BootRomImageData = BootRom;
    type RomImageData = Rom;
    type RamImageData = Ram;

    fn get_device_config(&self) -> &DeviceConfig {
        &self.device_config
    }

    fn get_cartridge_info(&self) -> &CartridgeInfo {
        &self.cartridge_info
    }

    fn get_boot_rom(&self) -> Option<&Self::BootRomImageData> {
        self.boot_rom.as_ref().map(|x| x as &Self::BootRomImageData)
    }

    fn get_cartridge_rom(&self) -> &Self::RomImageData {
        &self.cartridge_rom
    }

    fn get_cartridge_ram(&self) -> &Self::RamImageData {
        &self.cartridge_ram
    }
}


impl<Rom, Ram, BootRom> EmulatorClient_PPU for EmulatorContextDataHolder<Rom, Ram, BootRom>
where Rom: ImageData,
      Ram: ImageDataMut,
      BootRom: ImageData
{
    fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        _ = (x, y, color);
    }
}


impl<Rom, Ram, BootRom> EmulatorContextMut for EmulatorContextDataHolder<Rom, Ram, BootRom>
where Rom: ImageData,
      Ram: ImageDataMut,
      BootRom: ImageData
{
    type RamImageDataMut = Ram;

    fn get_cartridge_ram_mut(&mut self) -> &mut Self::RamImageDataMut {
        &mut self.cartridge_ram
    }
}
