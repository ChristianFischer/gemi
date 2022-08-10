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
use crate::ppu::{FrameState, Ppu, SCREEN_H, SCREEN_W};
use crate::window::Window;

/// The GameBoy object providing access to all it's emulated components.
pub struct GameBoy {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub mem: Memory,
    pub window: Window,
}


impl GameBoy {
    /// Create a new GameBoy device.
    pub fn new() -> Result<GameBoy,String> {
        let mem = Memory::new();
        let window = Window::create("GameBoy", SCREEN_W, SCREEN_H)?;

        Ok(
            GameBoy {
                cpu: Cpu::new(mem.create_read_write_handle()),
                ppu: Ppu::new(mem.create_read_write_handle()),
                mem,
                window,
            }
        )
    }

    /// Inserts a cartridge into the device and load ROM data into memory.
    pub fn insert_cart(&mut self, cartridge: &Cartridge) {
        self.mem.load_rom_data(cartridge);
    }

    /// Runs the program located on a cartridge, starting on the
    /// current location of the instruction pointer.
    pub fn run(&mut self) {
        loop {
            let instruction = self.cpu.fetch_next_instruction();

            println!(
                "/* {:04x} [{:02x}] */ {}",
                instruction.opcode_address,
                instruction.opcode_id,
                instruction
            );

            (instruction.opcode.proc)(self);

            // take the number of cycles consumed by the last operation
            let cycles = instruction.opcode.cycles;

            // let the PPU run for the same amount of cycles
            let ppu_state = self.ppu.update(cycles);

            // When a frame completed, it should be presented
            if let FrameState::FrameCompleted = ppu_state {
                self.window.present(self.ppu.get_lcd());
                self.window.poll_events();

                if !self.window.is_opened() {
                    return;
                }
            }
        }
    }
}
