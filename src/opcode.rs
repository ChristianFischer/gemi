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

use crate::gameboy::GameBoy;
use crate::opcodes::nop;

type ProcessOpCode = fn(gb: &mut GameBoy);

/// Data types for opcode parameters.
#[derive(Copy, Clone)]
pub enum ParamType {
    None,
    U8,
    U16,
}

/// Data struct describing a single opcode.
#[derive(Copy, Clone)]
pub struct OpCode {
    pub name: &'static str,
    pub bytes: usize,
    pub cycles: usize,
    pub proc: ProcessOpCode,
    pub param1: ParamType,
}


/// Stores a single instruction from ROM data.
/// This contains the opcode and the address the opcode was read from.
pub struct Instruction {
    pub opcode: &'static OpCode,
    pub opcode_id: u16,
    pub opcode_address: u16,
    pub param_address: u16,
}


/// Represents an invalid opcode.
/// This is intended as a placeholder for opcodes not yet implemented.
pub static OPCODE_INVALID: OpCode = OpCode { name: "???", bytes: 1, cycles: 0, proc: nop, param1: ParamType::None };

