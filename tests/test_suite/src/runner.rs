/*
 * Copyright (C) 2022-2023 by Christian Fischer
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

use std::env;
use gbemu_core::boot_rom::BootRom;
use gbemu_core::cartridge::Cartridge;
use gbemu_core::gameboy::{DeviceType, GameBoy};
use gbemu_core::utils::to_u8;
use tests_shared::test_config::{CheckResultConfig, EmulatorTestConfig, RunConfig, SetUpConfig};
use crate::checks::check_display::compare_display_with_image;
use crate::checks::gambatte_checks::check_gambatte_display_code;
use crate::util::get_test_file;

/// The maximum number of frames allowed per emulator run,
/// before it's considered as an error.
const MAX_FRAMES: u32 = 10_000;


/// Print the commandline to re-run this test ROM normally.
pub fn print_run_command(setup: &SetUpConfig) {
    let mut cmd = String::new();

    // command
    cmd.push_str("cargo run --package gbemu --bin gbemu --");

    // add argument for specific device type
    if let Some(device_type) = &setup.device {
        let device_type_arg = match device_type {
            DeviceType::GameBoyDmg     => "--dmg",
            DeviceType::GameBoyColor   => "--gbc",
            DeviceType::GameBoyAdvance => "--gba",
            DeviceType::SuperGameBoy   => "--sgb",
            DeviceType::SuperGameBoy2  => "--sgb2",
        };

        cmd.push_str(&format!(" {}", device_type_arg));
    }

    // get the current working dir
    let working_dir = if let Ok(wd) = env::current_dir() {
        format!("{}/", wd.to_str().unwrap().to_string().replace("\\", "/"))
    }
    else {
        String::new()
    };

    // add file reference to the ROM to be executed
    cmd.push_str(&format!(
        " {}{}",
        working_dir,
        get_test_file(&setup.cartridge_path)
    ));

    println!("#> {cmd}");
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

    // initialize
    gb.initialize();

    // enable serial data output queue
    if setup.enable_serial_output {
        gb.get_peripherals_mut().serial.enable_output_queue(true);
    }

    // set the color palette for DMG emulation
    if let Some(palette) = setup.dmg_display_palette {
        gb.get_peripherals_mut().ppu.set_dmg_display_palette(palette);
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
            let (addr_high, addr_low) = to_u8(current_address);

            // check for JR -2
            if check_for_opcode_sequence(gb, current_address, &[0x18, 0xfe]) {
                stop_next_frame = true;
            }

            // check for NOP, JR -3
            let seq_nop_jr = [0x00, 0x18, 0xfd];
            if
                    check_for_opcode_sequence(gb, current_address, &seq_nop_jr)
                ||  check_for_opcode_sequence(gb, current_address.saturating_sub(1), &seq_nop_jr)
            {
                stop_next_frame = true;
            }

            // check for jump to self
            if check_for_opcode_sequence(gb, current_address, &[0xc3, addr_low, addr_high]) {
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


/// Checks for a sequence of bytes in the memory at the specified address
fn check_for_opcode_sequence(gb: &GameBoy, address: u16, sequence: &[u8]) -> bool {
    for i in 0..sequence.len() {
        let i_addr = address + (i as u16);

        let byte_expected = sequence[i];
        let byte_read     = gb.get_mmu().read_u8(i_addr);

        if byte_read != byte_expected {
            return false;
        }
    }

    true
}


/// After the emulator has been finished, run result checks on the current state.
pub fn check_results(gb: &GameBoy, result: &CheckResultConfig) {
    if let Some(image_path) = &result.compare_lcd_with_image {
        compare_display_with_image(gb, &image_path, &result.color_mod);
    }
    
    if let Some(gambatte_display_result_code) = &result.gambatte_display_result_code {
        check_gambatte_display_code(gb, &gambatte_display_result_code);
    }
}


/// Helper function to run a whole test case
/// Constructs the emulator instance, runs the program and checks for results.
/// Each failure will lead into panic!
pub fn run_with_config(test_config: EmulatorTestConfig) -> GameBoy {
    let setup    = test_config.setup;
    let run_cfg = test_config.run_config;
    let result  = test_config.result;

    // print commandline arg to easily re-run the test
    print_run_command(&setup);

    // Construct
    let mut gb = create_device_with_config(setup);

    // Run
    run_to_stop_conditions(&mut gb, &run_cfg);

    // Check
    check_results(&gb, &result);

    gb
}
