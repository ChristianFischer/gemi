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
use crate::memory::{MemoryRead, MemoryReadWriteHandle, MemoryWrite};
use crate::opcodes::{OPCODE_TABLE, OPCODE_TABLE_EXTENDED};
use crate::utils::{to_u16, to_u8};

/// Definition for each supported 8 bit Register.
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub enum RegisterR16 {
    AF,
    BC,
    DE,
    HL,
}

/// A list of CPU flags.
pub enum CpuFlag {
    Zero,
    Negative,
    HalfCarry,
    Carry,
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
    mem: MemoryReadWriteHandle,

    /// All CPU registers as 8 bit value each.
    /// To access 16 bit registers there is a set of functions.
    registers: [u8; 8],

    /// Offset where to read the next instruction.
    instruction_pointer: u16,

    /// Offset where to read the next value from the stack.
    stack_pointer: u16,

    /// Sets interrupts to be enabled.
    interrupts_enabled: bool,

    /// Currently active CPU flags.
    flags: CpuFlags,
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
    pub fn new(mem: MemoryReadWriteHandle) -> Cpu {
        Cpu {
            mem,

            registers: [0; 8],

            instruction_pointer: 0x0100,
            stack_pointer: 0x0000,

            interrupts_enabled: false,

            flags: CpuFlags {
                z: false,
                n: false,
                h: false,
                c: false,
            },
        }
    }

    /// Enables interrupts.
    pub fn enable_interrupts(&mut self) {
        self.interrupts_enabled = true;
    }

    /// Disables interrupts.
    pub fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false;
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
        let opcode_byte    = self.get_next_byte() as u16;
        let opcode_id      = if opcode_byte == 0xCB { self.get_next_u16() } else { opcode_byte };
        let opcode         = self.fetch_next_opcode();
        let param_address  = self.instruction_pointer;
        let memory         = self.mem.clone_readonly();

        Instruction {
            opcode,
            opcode_id,
            opcode_address,
            param_address,
            memory
        }
    }

    /// Get the next byte on the current location of the instruction pointer, without moving it.
    pub fn get_next_byte(&self) -> u8 {
        self.mem.read_byte(self.instruction_pointer)
    }

    /// Get the next byte relative to the current location of the instruction pointer, without moving it.
    pub fn get_next_byte_at(&self, offset: u16) -> u8 {
        self.mem.read_byte(self.instruction_pointer + offset)
    }

    /// Get the next i8 value on the current location of the instruction pointer, without moving it.
    pub fn get_next_i8(&self) -> i8 {
        self.get_next_byte() as i8
    }

    /// Get the next u8 value on the current location of the instruction pointer, without moving it.
    pub fn get_next_u8(&self) -> u8 {
        self.get_next_byte()
    }

    /// Get the next u16 value on the current location of the instruction pointer, without moving it.
    pub fn get_next_u16(&self) -> u16 {
        let low  = self.get_next_byte_at(0);
        let high = self.get_next_byte_at(1);
        to_u16(high, low)
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
        let low  = self.fetch_u8();
        let high = self.fetch_u8();
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

    /// Pushes a 8bit value on the stack, moving the stack pointer.
    pub fn push_u8(&mut self, value: u8) {
        self.stack_pointer -= 1;
        self.mem.write_u8(self.stack_pointer, value);
    }

    /// Pushes a 16bit value on the stack, moving the stack pointer.
    pub fn push_u16(&mut self, value: u16) {
        let (high, low) = to_u8(value);
        self.push_u8(high);
        self.push_u8(low);
    }

    /// Pops a 8bit value from the stack, moving the stack pointer.
    pub fn pop_u8(&mut self) -> u8 {
        let value = self.mem.read_u8(self.stack_pointer);
        self.stack_pointer += 1;
        value
    }

    /// Pops a 8bit value from the stack, moving the stack pointer.
    pub fn pop_u16(&mut self) -> u16 {
        let low  = self.pop_u8();
        let high = self.pop_u8();
        to_u16(high, low)
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

    /// Checks whether a specific CPU flag is set.
    pub fn is_flag_set(&self, flag: CpuFlag) -> bool {
        match flag {
            CpuFlag::Zero      => self.flags.z,
            CpuFlag::Negative  => self.flags.n,
            CpuFlag::HalfCarry => self.flags.h,
            CpuFlag::Carry     => self.flags.c,
        }
    }

    /// Set the value of a CPU flag.
    pub fn set_flag(&mut self, flag: CpuFlag, value: bool) {
        match flag {
            CpuFlag::Zero      => self.flags.z = value,
            CpuFlag::Negative  => self.flags.n = value,
            CpuFlag::HalfCarry => self.flags.h = value,
            CpuFlag::Carry     => self.flags.c = value,
        }
    }

    /// Clear all CPU flags.
    pub fn clear_flags(&mut self) {
        self.flags.z = false;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
    }

    /// Set CPU flags based on a calculation result.
    pub fn set_flags_by_result(&mut self, old_value: u32, new_value: u32) {
        let carry_bits = old_value ^ new_value;
        self.set_flag(CpuFlag::Negative,  false);
        self.set_flag(CpuFlag::Zero,      new_value == 0);
        self.set_flag(CpuFlag::Carry,     (carry_bits & 0x0100) != 0);
        self.set_flag(CpuFlag::HalfCarry, (carry_bits & 0x0010) != 0);
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
    pub fn jump_to(&mut self, address: u16) {
        self.set_instruction_pointer(address);
    }

    /// Get the current address of the instruction pointer.
    pub fn get_instruction_pointer(&self) -> u16 {
        self.instruction_pointer
    }

    /// Set the current address of the instruction pointer.
    pub fn set_instruction_pointer(&mut self, address: u16) {
        self.instruction_pointer = address;
    }

    /// Get the current address of the stack pointer.
    pub fn get_stack_pointer(&self) -> u16 {
        self.stack_pointer
    }

    /// Set the current address of the stack pointer.
    pub fn set_stack_pointer(&mut self, address: u16) {
        self.stack_pointer = address;
    }
}
