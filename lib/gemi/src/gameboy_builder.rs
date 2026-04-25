/*
 * Copyright (C) 2022-2026 by Christian Fischer
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

use crate::audio::GameBoyAudio;
use crate::boot_rom::BootRom;
use crate::cartridge::{Cartridge, CartridgeInfo};
use crate::client_data::GameBoyClientData;
use crate::display::GameBoyDisplay;
use crate::GameBoy;

use crate::core::apu::audio_output::{AudioOutputSpec, AudioOutputSpecError};
use crate::core::device_type::{DeviceConfig, DeviceType, EmulationType};
use crate::core::emulator_device::EmulatorDevice;
use crate::core::ppu::graphic_data::DmgDisplayPalette;
use crate::core::utils::ioerr;
use std::fmt::{Display, Formatter};


/// A factory class to construct a GameBoy device object.
/// Usually created via GameBoy::build()
pub struct Builder {
    boot_rom:               Option<Box<BootRom>>,
    cartridge:              Option<Box<Cartridge>>,
    device_type:            Option<DeviceType>,
    audio_output_spec:      Option<AudioOutputSpec>,
    dmg_display_palette:    Option<DmgDisplayPalette>,
}


/// Error codes occurred during creating an emulator instance.
#[derive(Debug)]
pub enum BuilderErrorCode {
    IoError(ioerr::Error),

    AudioSpecError(AudioOutputSpecError),

    GameBoyColorNotSupported,
}


impl Builder {
    /// Creates a new empty GameBoy builder
    pub fn new() -> Self {
        Self {
            boot_rom:               None,
            cartridge:              None,
            device_type:            None,
            audio_output_spec:      None,
            dmg_display_palette:    None,
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


    /// Configure the audio output.
    pub fn set_audio_output_spec(&mut self, spec: AudioOutputSpec) {
        self.audio_output_spec = Some(spec);
    }


    /// Configure the palette to be used to translate DMG colors to the actual display colors.
    pub fn set_dmg_display_palette(&mut self, palette: DmgDisplayPalette) {
        self.dmg_display_palette = Some(palette);
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

        // create the audio interface
        let audio = Box::new(GameBoyAudio::new());

        // create the display based on the selected device config
        let mut display = Box::new(GameBoyDisplay::for_device(device_config));

        // apply the DMG display palette, if specified
        if let Some(dmg_display_palette) = self.dmg_display_palette {
            display.set_dmg_display_palette(dmg_display_palette);
        }

        // setup the emulator context
        let context_data = Box::new(
            GameBoyClientData {
                device_config,
                boot_rom,
                cartridge,
                audio,
                display,
            }
        );

        // construct the GameBoy object
        let mut emulator = Box::new(
            EmulatorDevice::new(context_data.as_ref())
        );

        // configure audio output, if specified
        if let Some(audio_output_spec) = self.audio_output_spec {
            emulator
                .get_peripherals_mut().apu
                .get_audio_output()
                .set_audio_spec(audio_output_spec)
                .map_err(BuilderErrorCode::AudioSpecError)?
            ;
        }

        Ok(GameBoy {
            client_data: context_data,
            emulator,
        })
    }
}


impl Display for BuilderErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to build emulator")
    }
}
