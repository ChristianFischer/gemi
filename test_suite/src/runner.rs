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

use gbemu_core::boot_rom::BootRom;
use gbemu_core::cartridge::Cartridge;
use gbemu_core::gameboy::{DeviceType, GameBoy};
use gbemu_core::graphic_data::DmgDisplayPalette;
use crate::checks::check_display::compare_display_with_image;
use crate::util::get_test_file;


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

    /// Optional: A palette to translate DMG color values into
    /// RGBA color values. Only used in DMG mode.
    pub dmg_display_palette: Option<DmgDisplayPalette>
}


/// Configuration on how to run the emulator.
/// Also contains stop conditions to stop the emulator after tests were finished.
/// Any stop condition are only checked after a frame was completed.
pub struct RunConfig {
    /// The maximum number of frames to be processed.
    /// Cannot be disabled, so the emulator should not run forever.
    pub max_frames: u32,

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
            dmg_display_palette: None,
        }
    }
}


impl Default for RunConfig {
    fn default() -> Self {
        Self {
            max_frames: 100,
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


/// Creates the device emulator based on a setup configuration.
pub fn create_device_with_config(setup: SetUpConfig) -> GameBoy {
    let mut builder = GameBoy::build();

    // set the device type to be emulated
    if let Some(device_type) = setup.device {
        builder.set_device_type(device_type);
    }

    // load the cartridge file
    let cartridge_path = get_test_file(&setup.cartridge_path);
    let cartridge = Cartridge::load_file(&cartridge_path).unwrap();
    builder.set_cartridge(cartridge);

    // load the boot ROM if any
    if let Some(boot_rom_path) = setup.boot_rom_path {
        let boot_rom = BootRom::load_file(&boot_rom_path).unwrap();
        builder.set_boot_rom(boot_rom);
    }

    // create the device emulator
    let mut gb = builder.finish().unwrap();

    // set the color palette for DMG emulation
    if let Some(palette) = setup.dmg_display_palette {
        gb.ppu.set_dmg_display_palette(palette);
    }

    gb
}


/// Run the emulator until any stop condition is met, which is defined in the RunConfig.
pub fn run_to_stop_conditions(gb: &mut GameBoy, config: &RunConfig) -> u32 {
    let mut stop_next_frame   = false;
    let mut frames_to_process = config.max_frames;
    let mut frames = 0;

    loop {
        gb.process_frame();
        frames += 1;

        // stop when the flag was set
        // this will be used to ensure the following frame will be completed,
        // after the actual program has been finished.
        if stop_next_frame {
            break;
        }

        // stop running after 'n' frames were processed
        frames_to_process = frames_to_process.saturating_sub(1);
        if frames_to_process == 0 {
            break;
        }

        // stop running when in HALT state
        if config.stop_on_halt {
            if !gb.cpu.is_running() {
                stop_next_frame = true;
            }
        }

        // stop if the emulator is stuck in an infinite loop
        // like JR -2
        if config.stop_on_infinite_loop {
        }
    }

    frames
}


/// After the emulator has been finished, run result checks on the current state.
pub fn check_results(gb: &GameBoy, result: &CheckResultConfig) {
    if let Some(image_path) = &result.compare_lcd_with_image {
        compare_display_with_image(gb, &image_path);
    }
}


/// Helper function to run a whole test case
/// Constructs the emulator instance, runs the program and checks for results.
/// Each failure will lead into panic!
pub fn run_with_config(test_config: EmulatorTestConfig) -> GameBoy {
    let setup    = test_config.setup;
    let run_cfg = test_config.run_config;
    let result  = test_config.result;

    // Construct
    let mut gb = create_device_with_config(setup);

    // Run
    run_to_stop_conditions(&mut gb, &run_cfg);

    // Check
    check_results(&gb, &result);

    gb
}
