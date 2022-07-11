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

use crate::opcode::{Instruction, OpCode};
use crate::memory::MemoryReadWrite;
use crate::opcode_table::{OPCODE_TABLE, OPCODE_TABLE_EXTENDED};

/// Definition for each supported 8 bit Register.
pub enum RegisterR8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

/// Definition for each supported 16 bit Register.
pub enum RegisterR16 {
    AF,
    BC,
    DE,
    HL,
}

/// Current configuration of CPU flags.
pub struct CpuFlags {
    /// zero flag
    z: bool,

    /// negative flag
    n: bool,

    /// half-carry flag
    h: bool,

    /// carry flag
    c: bool,
}

/// An object representing the gameboy's CPU
pub struct Cpu {
    /// Handle to the device memory.
    mem: MemoryReadWrite,

    /// All CPU registers as 8 bit value each.
    /// To access 16 bit registers there is a set of functions.
    registers: [u8; 8],

    /// Offset where to read the next instruction.
    instruction_pointer: u16,

    /// Offset where to read the next value from the stack.
    stack_pointer: u16,

    /// Currently active CPU flags.
    flags: CpuFlags,
}

/// Combines a high and low byte into a 16 bit value.
pub const fn to_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

/// Splits a 16 bit value into it's high and low bytes.
pub const fn to_u8(value: u16) -> (u8, u8) {
    (get_high(value), get_low(value))
}

/// Get the high byte of a 16 bit value.
pub const fn get_high(value: u16) -> u8 {
    ((value >> 8) & 0xff) as u8
}

/// Get the low byte of a 16 bit value.
pub const fn get_low(value: u16) -> u8 {
    (value & 0xff) as u8
}

impl RegisterR16 {
    /// Get the 8 bit registers which contains the high and low bytes of a 16 bit register.
    pub const fn to_r8(self) -> (RegisterR8, RegisterR8) {
        match self {
            RegisterR16::AF => (RegisterR8::A, RegisterR8::F),
            RegisterR16::BC => (RegisterR8::B, RegisterR8::C),
            RegisterR16::DE => (RegisterR8::D, RegisterR8::E),
            RegisterR16::HL => (RegisterR8::H, RegisterR8::L),
        }
    }

    /// Get the 8 bit register which contains the high byte of a 16 bit register.
    pub const fn get_high(self) -> RegisterR8 {
        match self {
            RegisterR16::AF => RegisterR8::A,
            RegisterR16::BC => RegisterR8::B,
            RegisterR16::DE => RegisterR8::D,
            RegisterR16::HL => RegisterR8::H,
        }
    }

    /// Get the 8 bit register which contains the low byte of a 16 bit register.
    pub const fn get_low(self) -> RegisterR8 {
        match self {
            RegisterR16::AF => RegisterR8::F,
            RegisterR16::BC => RegisterR8::C,
            RegisterR16::DE => RegisterR8::E,
            RegisterR16::HL => RegisterR8::L,
        }
    }
}

impl Cpu {
    /// Creates an empty CPU object.
    pub fn new(mem: MemoryReadWrite) -> Cpu {
        Cpu {
            mem,

            registers: [0; 8],

            instruction_pointer: 0x0100,
            stack_pointer: 0x0000,

            flags: CpuFlags {
                z: false,
                n: false,
                h: false,
                c: false,
            },
        }
    }

    /// Fetches the next opcode on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_next_opcode(&mut self) -> &'static OpCode {
        let opcode_byte = self.fetch_u8();
        if opcode_byte != 0xCB {
            &OPCODE_TABLE[opcode_byte as usize]
        }
        else {
            let opcode_byte_extended = self.fetch_u8();
            &OPCODE_TABLE_EXTENDED[opcode_byte_extended as usize]
        }
    }

    /// Fetches the next instruction on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_next_instruction(&mut self) -> Instruction {
        let opcode_address = self.instruction_pointer;
        let opcode_id = self.get_next_byte() as u16;
        let opcode = self.fetch_next_opcode();
        let param_address = self.instruction_pointer;

        Instruction {
            opcode,
            opcode_id,
            opcode_address,
            param_address
        }
    }

    /// Get the next byte on the current location of the instruction pointer, without moving it.
    pub fn get_next_byte(&self) -> u8 {
        self.mem.read(self.instruction_pointer)
    }

    /// Get the next i8 value on the current location of the instruction pointer, without moving it.
    pub fn get_next_i8(&self) -> i8 {
        self.get_next_byte() as i8
    }

    /// Get the next u8 value on the current location of the instruction pointer, without moving it.
    pub fn get_next_u8(&self) -> u8 {
        self.get_next_byte()
    }

    /// Fetches the next u8 value on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_u8(&mut self) -> u8 {
        let value = self.get_next_byte();
        self.instruction_pointer += 1;
        value
    }

    /// Fetches the next u16 value on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_u16(&mut self) -> u16 {
        let high = self.fetch_u8();
        let low  = self.fetch_u8();
        to_u16(high, low)
    }

    /// Fetches the next u8 value on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_i8(&mut self) -> i8 {
        self.fetch_u8() as i8
    }

    /// Fetches the next u8 value on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_i16(&mut self) -> i16 {
        self.fetch_u16() as i16
    }

    /// Get the value of a 8 bit register.
    pub fn get_r8(&self, register: RegisterR8) -> u8 {
        self.registers[register as usize]
    }

    /// Set the value of a 8 bit register.
    pub fn set_r8(&mut self, register: RegisterR8, value: u8) {
        self.registers[register as usize] = value;
    }

    /// Get the value of a 16 bit register.
    pub fn get_r16(&self, register: RegisterR16) -> u16 {
        let (high_r8, low_r8) = register.to_r8();
        let high = self.registers[high_r8 as usize];
        let low  = self.registers[low_r8 as usize];
        to_u16(high, low)
    }

    /// Set the value of a 16 bit register.
    pub fn set_r16(&mut self, register: RegisterR16, value: u16) {
        let (high_r8, low_r8) = register.to_r8();
        let (high, low) = to_u8(value);
        self.registers[high_r8 as usize] = high;
        self.registers[low_r8 as usize]  = low;
    }

    /// Moves the instruction pointer relative to it's current position.
    pub fn jump_relative(&mut self, offset: i16) {
        if offset < 0 {
            self.instruction_pointer -= (-offset) as u16;
        }
        else {
            self.instruction_pointer += offset as u16;
        }
    }

    /// Moves the instruction pointer to a fixed location.
    pub fn jump_to(&mut self, offset: u16) {
        self.instruction_pointer = offset;
    }

    /// set the current address of the stack pointer
    pub fn set_stack_pointer(&mut self, address: u16) {
        self.stack_pointer = address;
    }
}
