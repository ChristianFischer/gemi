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

use crate::boot_rom::BootRom;
use crate::cartridge::{Cartridge, CartridgeInfo, DataBuffer};
use crate::core::device_type::DeviceConfig;
use crate::core::emulator_client::{EmulatorClient, EmulatorClientMut};


/// A struct holding the internal data used by the emulator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct GameBoyClientData {
    pub(crate) device_config: DeviceConfig,
    pub(crate) boot_rom:  Option<Box<BootRom>>,
    pub(crate) cartridge: Box<Cartridge>,
}


impl EmulatorClient for GameBoyClientData {
    type BootRomImageData = BootRom;
    type RomImageData = DataBuffer;
    type RamImageData = DataBuffer;


    fn get_device_config(&self) -> &DeviceConfig {
        &self.device_config
    }

    fn get_cartridge_info(&self) -> &CartridgeInfo {
        self.cartridge.get_cartridge_info()
    }

    fn get_boot_rom(&self) -> Option<&Self::BootRomImageData> {
        self.boot_rom.as_ref().map(|b| b.as_ref())
    }

    fn get_cartridge_rom(&self) -> &Self::RomImageData {
        self.cartridge.get_rom()
    }

    fn get_cartridge_ram(&self) -> &Self::RamImageData {
        self.cartridge.get_ram()
    }
}


impl EmulatorClientMut for GameBoyClientData {
    type RamImageDataMut = DataBuffer;

    fn get_cartridge_ram_mut(&mut self) -> &mut Self::RamImageDataMut {
        self.cartridge.get_ram_mut()
    }
}


