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

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::memory::Memory;

/// The GameBoy object providing access to all it's emulated components.
pub struct GameBoy {
    pub cpu: Cpu,
    pub mem: Memory,
}


impl GameBoy {
    /// Create a new GameBoy device.
    pub fn new() -> GameBoy {
        let mem = Memory::new();

        GameBoy {
            cpu: Cpu::new(mem.create_read_write_handle()),
            mem,
        }
    }

    /// Inserts a cartridge into the device and load ROM data into memory.
    pub fn insert_cart(&mut self, cartridge: &Cartridge) {
        self.mem.load_rom_data(cartridge);
    }

    /// Runs the program located on a cartridge, starting on the
    /// current location of the instruction pointer.
    pub fn run(&mut self) {
        while true {
            let instruction = self.cpu.fetch_next_instruction();

            println!(
                "/* {:04x} [{:02x}] */ {}",
                instruction.opcode_address,
                instruction.opcode_id,
                instruction
            );

            if instruction.opcode.name == "???" {
                println!("Invalid OpCode @ ${:04x}", instruction.opcode_address);
                return;
            }

            (instruction.opcode.proc)(self);
        }
    }
}
