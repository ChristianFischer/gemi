/*
 * Copyright (C) 2022 by Christian Fischer
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

use gbemu_core::gameboy::DeviceType;
use gbemu_core::graphic_data::DmgDisplayPalette;


/// Configuration parameters on how to setup an emulator instance.
pub struct SetUpConfig {
    /// Optional: Device type which kind of device to be emulated.
    /// When omitted, the emulator tries to detect the correct
    /// emulation based on the cartridge to be inserted.
    pub device: Option<DeviceType>,

    /// Optional: Boot ROM to be run before the actual cartridge is run.
    pub boot_rom_path: Option<String>,

    /// Path to the ROM file to run.
    pub cartridge_path: String,

    /// If true, output to the serial port will be stored.
    pub enable_serial_output: bool,

    /// Optional: A palette to translate DMG color values into
    /// RGBA color values. Only used in DMG mode.
    pub dmg_display_palette: Option<DmgDisplayPalette>
}


/// Configuration on how to run the emulator.
/// Also contains stop conditions to stop the emulator after tests were finished.
/// Any stop condition are only checked after a frame was completed.
pub struct RunConfig {
    /// The number of frames to be processed.
    pub run_frames: Option<u32>,

    /// Stops the emulator when in HALT mode.
    pub stop_on_halt: bool,

    /// Stops the emulator when an infinite loop was detected,
    /// like when a 'JR -2' instruction is invoked.
    pub stop_on_infinite_loop: bool,
}


/// Configuration how to check whether a test ROM was successful or not.
pub struct CheckResultConfig {
    /// Compare the emulator display with a given image.
    /// If failed, the test will print a pattern to detect which areas
    /// of the screen were different to the comparison image.
    pub compare_lcd_with_image: Option<String>,
}


/// Configuration of a single test case.
pub struct EmulatorTestConfig {
    pub setup:      SetUpConfig,
    pub run_config: RunConfig,
    pub result:     CheckResultConfig,
}


impl SetUpConfig {
    /// Creates a configuration where a ROM file will be loaded.
    pub fn with_rom_file(cartridge_path: &str) -> Self {
        Self {
            device: None,
            cartridge_path: cartridge_path.to_string(),
            boot_rom_path: None,
            enable_serial_output: false,
            dmg_display_palette: None,
        }
    }
}


impl Default for RunConfig {
    fn default() -> Self {
        Self {
            run_frames: None,
            stop_on_halt: false,
            stop_on_infinite_loop: false,
        }
    }
}


impl Default for CheckResultConfig {
    fn default() -> Self {
        Self {
            compare_lcd_with_image: None,
        }
    }
}

