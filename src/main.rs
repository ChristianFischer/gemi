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

extern crate core;

mod boot_rom;
mod cartridge;
mod cpu;
mod gameboy;
mod input;
mod mbc;
mod memory;
mod memory_data;
mod opcode;
mod opcodes;
mod ppu;
mod timer;
mod utils;
mod window;

use cartridge::Cartridge;
use cartridge::GameBoyColorSupport;
use std::env;
use crate::boot_rom::BootRom;
use crate::gameboy::{DeviceType, GameBoy};

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


fn main() {
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

    match builder.finish() {
        Ok(mut gb) => {
            gb.initialize();
            gb.run();
        }

        Err(e) => {
            println!("Failed: {}", e);
        }
    }
}
