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

use std::fmt::{Display, Formatter};
use crate::gameboy::GameBoy;
use crate::memory::{MemoryRead, MemoryReadOnlyHandle};

type ProcessOpCode = fn(gb: &mut GameBoy);

/// Data struct describing a single opcode.
#[derive(Copy, Clone)]
pub struct OpCode {
    pub name: &'static str,
    pub bytes: u32,
    pub cycles: u32,
    pub proc: ProcessOpCode,
}


/// Stores a single instruction from ROM data.
/// This contains the opcode and the address the opcode was read from.
pub struct Instruction {
    pub opcode: &'static OpCode,
    pub opcode_id: u16,
    pub opcode_address: u16,
    pub param_address: u16,
    pub memory: MemoryReadOnlyHandle,
}

fn get_arg(arg: &str, instruction: &Instruction) -> String {
    match arg {
        "i8" => {
            let value = instruction.memory.read_i8(instruction.param_address);
            format!("{}", value)
        }

        "u8" => {
            let value = instruction.memory.read_u8(instruction.param_address);
            format!("{}", value)
        }

        "x8" => {
            let value = instruction.memory.read_u8(instruction.param_address);
            format!("{:02x}", value)
        }

        "i16" => {
            let value = instruction.memory.read_i16(instruction.param_address);
            format!("{}", value)
        }

        "u16" => {
            let value = instruction.memory.read_u16(instruction.param_address);
            format!("{}", value)
        }

        "x16" => {
            let value = instruction.memory.read_u16(instruction.param_address);
            format!("{:04x}", value)
        }

        _ => arg.to_string()
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut label = self.opcode.name.to_string();

        loop {
            let begin = label.find("{");
            match begin {
                Some(begin_index) => {
                    let end = label.find("}");
                    if let Some(end_index) = end {
                        let substring = &label[begin_index+1 .. end_index];
                        let formatted = get_arg(substring, &self);

                        label.replace_range(begin_index .. end_index+1, formatted.as_str());
                    }
                }

                None => {
                    break;
                }
            }
        }

        write!(f, "{}", label)
    }
}
