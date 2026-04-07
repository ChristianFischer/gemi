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

use crate::cartridge::image_data::{FixedSizeImageData, ImageData, ImageDataMut};
use crate::cartridge::CartridgeInfo;
use crate::device_type::DeviceConfig;


/// A trait representing the interface between the emulator and the client application.
///
/// While the emulator is providing CPU, PPU, memory, and other components, anything outside
/// the emulator components has to be provided by the client application.
/// This trait is used by the emulator to query for each of those components.
///
/// The client also has to provide information about the kind of device being emulated and
/// the properties of the cartridge. It is expected this information to be configured
/// on initialization and never changed during runtime.
///
/// In addition to this trait, a client also has to implement [EmulatorClientMut] for mutable
/// access to components which need to be modified, like RAM.
pub trait EmulatorClient {
    /// A fixed-size image data type specifically for the boot ROM image,
    /// constrained to have a size of 256 bytes.
    type BootRomImageData: FixedSizeImageData<256>;

    /// A generic image data type for representing cartridge ROM data.
    type RomImageData: ImageData;

    /// A generic image data type for representing cartridge RAM data.
    type RamImageData: ImageData;


    /// Retrieves the configuration of the device being emulated.
    ///
    /// This information is used to initialize memory banks and controls the availability of
    /// certain features like Color GameBoy support.
    /// Changing this information during runtime is not supported and may lead to undefined behaviour.
    fn get_device_config(&self) -> &DeviceConfig;

    /// Retrieves the cartridge information.
    ///
    /// This information is needed to initialize the emulated memory bank controller and to
    /// decide whether RAM emulation is needed.
    /// The [CartridgeInfo] can be read from the ROM image upon initialization and is not meant
    /// to be changed during runtime.
    fn get_cartridge_info(&self) -> &CartridgeInfo;

    /// Retrieves a reference to the Boot ROM image data, if available.
    ///
    /// This function provides access to the Boot ROM image for the emulator launch.
    /// The Boot ROM image is optional. If no Boot ROM image is provided,
    /// the emulator will initialize into a state which is the same as if the
    /// original Boot ROM was not used.
    fn get_boot_rom(&self) -> Option<&Self::BootRomImageData>;

    /// Retrieves a reference to the cartridge ROM.
    fn get_cartridge_rom(&self) -> &Self::RomImageData;

    /// Retrieves a reference to the cartridge RAM.
    ///
    /// For efficiency this function is expected to always return a valid reference,
    /// even if no RAM is supported or available.
    /// In case a cartridge is not supposed to provide RAM, this function is expected
    /// to provide an empty buffer.
    fn get_cartridge_ram(&self) -> &Self::RamImageData;
}


/// Part of the [EmulatorClient] trait, providing mutable access the client's components.
pub trait EmulatorClientMut: EmulatorClient
{
    type RamImageDataMut: ImageDataMut;

    fn get_cartridge_ram_mut(&mut self) -> &mut Self::RamImageDataMut;
}
