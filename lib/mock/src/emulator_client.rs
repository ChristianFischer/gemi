/*
 * Copyright (C) 2026 by Christian Fischer
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

use crate::{MockBootRom, MockImageData, ZeroImageData};
use gemi_core::cartridge::image_data::{FixedSizeImageData, ImageData, ImageDataMut};
use gemi_core::cartridge::CartridgeInfo;
use gemi_core::device_type::DeviceConfig;
use gemi_core::emulator_client::{EmulatorClient, EmulatorClientMut};
use gemi_core::utils::ioerr;


/// A mock implementation of the `EmulatorClient` trait.
///
/// This implementation allows to easily create simple instances of the emulator client
/// for testing purposes.
/// By default, this client is using other mock implementations for ROM, RAM, and boot ROM, but
/// also allows selecting different implementations via the template arguments of this type.
pub struct MockEmulatorClient<Rom, Ram, BootRom>
where Rom: ImageData,
      Ram: ImageDataMut,
      BootRom: FixedSizeImageData<256>,
{
    pub device_config:  DeviceConfig,
    pub cartridge_info: CartridgeInfo,
    pub cartridge_rom:  Rom,
    pub cartridge_ram:  Ram,
    pub boot_rom:       Option<BootRom>,
}



impl<Rom, Ram, BootRom> MockEmulatorClient<Rom, Ram, BootRom>
where Rom: ImageData,
      Ram: ImageDataMut,
      BootRom: FixedSizeImageData<256>,
{
    /// Creates a new instance of the mock emulator client with custom RAM and ROM images.
    pub fn new(device_config: DeviceConfig, rom_data: Rom, ram_data: Ram) -> ioerr::Result<Self> {
        let cartridge_info = CartridgeInfo::create_from(&rom_data)?;

        Ok(
            Self {
                device_config,
                cartridge_info,
                cartridge_rom: rom_data,
                cartridge_ram: ram_data,
                boot_rom: None,
            }
        )
    }
}


impl MockEmulatorClient<MockImageData, ZeroImageData, MockBootRom> {
    /// Creates a "default" instance.
    ///
    /// This instance will consist of a simple 32k ROM image with no meaningful
    /// data, an empty RAM image, and no Boot ROM image.
    pub fn new_default(device_config: DeviceConfig) -> Self {
        Self {
            device_config,
            cartridge_info: CartridgeInfo::default(),
            cartridge_rom: MockImageData::new(),
            cartridge_ram: ZeroImageData::new(),
            boot_rom: None,
        }
    }
}


impl MockEmulatorClient<ZeroImageData, ZeroImageData, MockBootRom> {
    /// Creates an empty instance.
    ///
    /// This instance will consist of empty RAM and ROM images.
    /// Running an emulator with this client will likely run into
    /// undefined behaviour as reading from the cartridge will not
    /// be possible.
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


impl<Rom, Ram, BootRom> EmulatorClient for MockEmulatorClient<Rom, Ram, BootRom>
where Rom: ImageData,
      Ram: ImageDataMut,
      BootRom: FixedSizeImageData<256>
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


impl<Rom, Ram, BootRom> EmulatorClientMut for MockEmulatorClient<Rom, Ram, BootRom>
where Rom: ImageData,
      Ram: ImageDataMut,
      BootRom: FixedSizeImageData<256>
{
    type RamImageDataMut = Ram;

    fn get_cartridge_ram_mut(&mut self) -> &mut Self::RamImageDataMut {
        &mut self.cartridge_ram
    }
}
