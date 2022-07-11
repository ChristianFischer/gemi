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

mod cartridge;
mod cpu;
mod gameboy;
mod memory;
mod opcode;
mod opcode_table;
mod opcodes_ld;
mod opcodes_jump;
mod opcodes;

use cartridge::Cartridge;
use cartridge::GameBoyColorSupport;
use cpu::Cpu;
use std::env;
use crate::gameboy::GameBoy;

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
    println!("Features:      {}",     features.join(", "));
    println!("ROM size:      {} kiB", cartridge.get_rom_size() / 1024);
    println!("RAM size:      {} kiB", cartridge.get_ram_size() / 1024);
    println!("GameBoy Color: {}",     gbc);
    println!("SuperGameBoy:  {}",     cartridge.supports_sgb());
}

fn run(cartridge: &Cartridge) {
    let mut gb = GameBoy::new();
    gb.insert_cart(cartridge);
    gb.run();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file = &args[1];

        let cartridge = Cartridge::load_file(file).expect("Unable to load ROM");

        print_rom_info(file, &cartridge);

        run(&cartridge);
    }
}
