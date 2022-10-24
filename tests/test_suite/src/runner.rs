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
use gbemu_core::gameboy::GameBoy;
use gbemu_core::memory::MemoryRead;
use tests_shared::test_config::{CheckResultConfig, EmulatorTestConfig, RunConfig, SetUpConfig};
use crate::checks::check_display::compare_display_with_image;
use crate::util::get_test_file;

/// The maximum number of frames allowed per emulator run,
/// before it's considered as an error.
const MAX_FRAMES: u32 = 10_000;


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

    // enable serial data output queue
    if setup.enable_serial_output {
        gb.serial.enable_output_queue(true);
    }

    // set the color palette for DMG emulation
    if let Some(palette) = setup.dmg_display_palette {
        gb.ppu.set_dmg_display_palette(palette);
    }

    gb
}


/// Run the emulator until any stop condition is met, which is defined in the RunConfig.
pub fn run_to_stop_conditions(gb: &mut GameBoy, config: &RunConfig) -> u32 {
    let mut stop_next_frame   = false;
    let mut frames_to_process = config.run_frames;
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
        if let Some(frames) = &mut frames_to_process {
            *frames = frames.saturating_sub(1);

            if *frames == 0 {
                break;
            }
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
            let current_address = gb.cpu.get_instruction_pointer();
            let opcode          = gb.mem.read_u8(current_address + 0);
            let param_i8         = gb.mem.read_i8(current_address + 1);
            let param_u16        = gb.mem.read_u16(current_address + 1);

            // check for JR -2
            if opcode == 0x18 && param_i8 == -2 {
                stop_next_frame = true;
            }

            // check for jump to self
            if opcode == 0xC3 && param_u16 == current_address {
                stop_next_frame = true;
            }
        }

        // if no stop condition was met, stop when 10k frames were computed
        assert!(
            frames < MAX_FRAMES,
            "Emulator did not properly stop before reaching {} frames",
            MAX_FRAMES
        )
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
