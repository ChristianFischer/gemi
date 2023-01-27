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

extern crate core;

mod sound_queue;
mod window;

use gbemu_core::cartridge::Cartridge;
use gbemu_core::cartridge::GameBoyColorSupport;
use gbemu_core::boot_rom::BootRom;
use gbemu_core::gameboy::{DeviceType, GameBoy};
use std::{env, time};
use std::time::Duration;
use gbemu_core::cpu::CPU_CLOCK_SPEED;
use crate::window::Window;

fn print_rom_info(filename: &String, cartridge: &Cartridge) {
    let mut features: Vec<&str> = vec![];

    if cartridge.has_ram() {
        features.push("RAM");
    }

    if cartridge.has_battery() {
        features.push("Battery");
    }

    if cartridge.has_timer() {
        features.push("Timer");
    }

    if cartridge.has_rumble() {
        features.push("Rumble");
    }

    let gbc = match cartridge.get_cgb_support() {
        GameBoyColorSupport::Supported => "Supported",
        GameBoyColorSupport::Required => "Required",
        GameBoyColorSupport::None => "-",
    };

    println!("ROM file: {}", filename);
    println!("Title:         {}",     cartridge.get_title());
    println!("Manufacturer:  {}",     cartridge.get_manufacturer_code());
    println!("Licensee:      {}",     cartridge.get_licensee_code());
    println!("MBC:           {}",     cartridge.get_mbc());
    println!("Features:      {}",     features.join(", "));
    println!("ROM size:      {} kiB", cartridge.get_rom_size() / 1024);
    println!("RAM size:      {} kiB", cartridge.get_ram_size() / 1024);
    println!("GameBoy Color: {}",     gbc);
    println!("SuperGameBoy:  {}",     cartridge.supports_sgb());
}


fn run(window: &mut Window, gb: &mut GameBoy) {
    let mut interval_begin  = time::Instant::now();
    let mut interval_cycles = 0;

    while window.is_opened() {
        let frame_cycles = gb.process_frame();
        interval_cycles += frame_cycles;

        // update window
        {
            let peripherals = gb.get_peripherals_mut();

            window.poll_events();
            window.apply_key_states(&mut peripherals.input);
            window.present(peripherals.ppu.get_lcd(), &peripherals.ppu);
            window.push_audio_samples(peripherals.apu.take_samples());
        }

        // handle frame times
        {
            let frame_end_time = time::Instant::now();

            let interval_duration_ns = (frame_end_time - interval_begin).as_nanos() as u64;
            let expected_time_ns     = (1_000_000_000u64 * interval_cycles) / CPU_CLOCK_SPEED;

            // when the interval time was shorter than expected,
            // let the CPU sleep for the time difference
            if interval_duration_ns < expected_time_ns {
                let time_remaining = expected_time_ns - interval_duration_ns;
                std::thread::sleep(Duration::from_nanos(time_remaining));
            }

            // reset the interval after counting more than the number of cycles per second,
            // so we get a bit more precision than just counting per frame
            if interval_cycles >= CPU_CLOCK_SPEED {
                interval_cycles -= CPU_CLOCK_SPEED;
                interval_begin = frame_end_time;
            }
        }
    }
}


fn make_gameboy_instance() -> Result<GameBoy, String> {
    let mut args = env::args().into_iter();
    let mut builder = GameBoy::build();

    // skip first argument, which is the executable name
    _ = args.next();

    while let Some(arg) = args.next() {
        println!("ARG: {}", arg);

        match arg.as_str() {
            "--boot" => {
                let filename = args.next()
                    .expect("'--boot' needs to be followed by the path to a valid boot rom");

                let boot_rom = BootRom::load_file(&filename).unwrap();
                builder.set_boot_rom(boot_rom);
            }

            "--dmg" => {
                builder.set_device_type(DeviceType::GameBoyDmg);
            }

            "--gbc" => {
                builder.set_device_type(DeviceType::GameBoyColor);
            }

            "--gba" => {
                builder.set_device_type(DeviceType::GameBoyAdvance);
            }

            "--sgb" => {
                builder.set_device_type(DeviceType::SuperGameBoy);
            }

            "--sgb2" => {
                builder.set_device_type(DeviceType::SuperGameBoy2);
            }
            
            "--print-opcodes" => {
                builder.set_print_opcodes(true);
            }

            _ => {
                let filename = arg;
                let cart     = Cartridge::load_file(&filename).unwrap();
                print_rom_info(&filename, &cart);

                builder.set_cartridge(cart);
            }
        }
    }

    builder.finish()
}


fn main() -> Result<(), String> {
    // create the gb instance using the current commandline arguments
    let mut gb = make_gameboy_instance()?;
    gb.initialize();

    // determine the title based on the cartridge available
    let title = match &*gb.get_peripherals().mem.get_cartridge() {
        Some(cartridge) => cartridge.get_title().to_string(),
        None => "GameBoy".to_string(),
    };

    // create window
    let mut window = Window::create(&title)?;

    // run the game
    run(&mut window, &mut gb);

    // after running the cartridge, save it's on-chip-RAM, if any
    gb.get_peripherals().mem.save_cartridge_ram_if_any()
        .map_err(|e| e.to_string())
        ?
    ;

    // everything went ok
    Ok(())
}


