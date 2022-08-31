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

use crate::boot_rom::BootRom;
use crate::cartridge::Cartridge;
use crate::cpu::{Cpu, RegisterR8};
use crate::input::Input;
use crate::memory::Memory;
use crate::ppu::{FrameState, Ppu, SCREEN_H, SCREEN_W};
use crate::timer::Timer;
use crate::window::Window;

/// The GameBoy object providing access to all it's emulated components.
pub struct GameBoy {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub mem: Memory,
    pub timer: Timer,
    pub input: Input,
    pub window: Window,
}


impl GameBoy {
    /// Create a new GameBoy device.
    pub fn new() -> Result<GameBoy,String> {
        let mem = Memory::new();
        let window = Window::create("GameBoy")?;

        Ok(
            GameBoy {
                cpu: Cpu::new(mem.create_read_write_handle()),
                ppu: Ppu::new(mem.create_read_write_handle()),
                timer: Timer::new(mem.create_read_write_handle()),
                input: Input::new(mem.create_read_write_handle()),
                mem,
                window,
            }
        )
    }

    /// Assign a boot rom to be executed on startup.
    pub fn set_boot_rom(&mut self, boot_rom: BootRom) {
        self.mem.set_boot_rom(boot_rom);
    }

    /// Inserts a cartridge into the device and load ROM data into memory.
    pub fn insert_cart(&mut self, cartridge: Cartridge) {
        self.mem.set_cartridge(cartridge);
    }

    /// Boot the device, initializing the Boot ROM program.
    pub fn initialize(&mut self) {
        if self.mem.has_boot_rom() {
            self.cpu.set_instruction_pointer(0x0000);
        }
        else {
            self.setup_dmg();
        }
    }

    /// setup values like expected after the boot rom was executed on the original GameBoy.
    fn setup_dmg(&mut self) {
        self.cpu.set_r8(RegisterR8::A, 0x01);
        self.cpu.set_r8(RegisterR8::F, 0xB0);
        self.cpu.set_r8(RegisterR8::B, 0x00);
        self.cpu.set_r8(RegisterR8::C, 0x13);
        self.cpu.set_r8(RegisterR8::D, 0x00);
        self.cpu.set_r8(RegisterR8::E, 0xd8);
        self.cpu.set_r8(RegisterR8::H, 0x01);
        self.cpu.set_r8(RegisterR8::L, 0x4d);
        self.cpu.set_instruction_pointer(0x0100);
        self.cpu.set_stack_pointer(0xfffe);
    }

    /// Runs the program located on a cartridge, starting on the
    /// current location of the instruction pointer.
    pub fn run(&mut self) {
        loop {
            let instruction = self.cpu.fetch_next_instruction();

            (instruction.opcode.proc)(self);

            println!(
                "/* {:04x} [{:02x}]{} */ {:<16}    ; {}",
                instruction.opcode_address,
                instruction.opcode_id,
                if instruction.opcode_id <= 0xff { "  " } else { "" },
                instruction.to_string(),
                self.cpu
            );

            // take the number of cycles consumed by the last operation
            let cycles = instruction.opcode.cycles;

            // let other components handle their state
            self.mem.update(cycles);
            self.cpu.update(cycles);
            self.timer.update(cycles);
            self.input.update();

            // let the PPU run for the same amount of cycles
            let ppu_state = self.ppu.update(cycles);

            // When a frame completed, it should be presented
            if let FrameState::FrameCompleted = ppu_state {
                self.window.poll_events();
                self.window.apply_key_states(&mut self.input);
                self.window.present(self.ppu.get_lcd(), &self.ppu);

                if !self.window.is_opened() {
                    return;
                }
            }
        }
    }
}
