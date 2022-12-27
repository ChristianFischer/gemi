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
use crate::gameboy::{Clock, GameBoy};
use crate::memory::{MemoryRead, MemoryReadOnlyHandle};

type ProcessOpCode = fn(gb: &mut GameBoy, ctx: &mut OpCodeContext) -> OpCodeResult;


/// A macro to generate an opcode implementation function.
macro_rules! opcode {
    ($(#[$meta:meta])? $name:ident, [$($bind_gb:ident)? $(, $bind_ctx:ident)?] $($body:tt)*) => {
        $(#[$meta])?
        pub fn $name(gb: &mut GameBoy, ctx: &mut OpCodeContext) -> crate::opcode::OpCodeResult {
            // silence 'unused' warning for gb and ctx
            { let _ = (&gb, &ctx); }

            // make gb and ctx visible to 'body', if requested
            $(let $bind_gb  = gb;)?
            $(let $bind_ctx = ctx;)?

            // paste 'body' statements
            let result = {
                $($body)*
            };

            // convert return value into OpCodeResult
            crate::opcode::OpCodeResult::from(result)
        }
    };
}

pub(crate) use opcode;


/// Data struct describing a single opcode.
#[derive(Copy, Clone)]
pub struct OpCode {
    /// The opcode label, may contain placeholders to be replaced by
    /// actual arguments read from the opcode data stream
    pub name: &'static str,

    /// Total length of the opcode, including arguments,
    /// but excluding the 0xcb prefix for the extended table.
    pub bytes: u32,

    /// Number of cycles to pass before the actual opcode execution.
    /// This may be relevant for opcodes which are writing or reading memory to ensure
    /// the actual read/write operation will happen at the same time as expected
    /// on the real device.
    /// The number of cycles ahead of the opcode are intended to be already included in
    /// the total number of cycles in ```cycles``` and not added additionally.
    pub cycles_ahead: Clock,

    /// Number of T-Cycles the opcode takes to execute.
    /// Does not include extra time when branches are taken.
    pub cycles: Clock,

    /// Function pointer to the actual opcode execution.
    pub proc: ProcessOpCode,
}


/// Context object to deliver additional information about the current context
/// to the opcode implementation, but also allow the opcode implementation to
/// deliver additional results to it's caller.
pub struct OpCodeContext {
    /// The currently executed opcode
    opcode: &'static OpCode,

    /// Stores the number of cycles the execution of this opcode consumed
    cycles: Clock,

    /// The stage an opcode is in, used for opcodes which will be processed in multiple steps
    stage: u8,
}


/// Result code of an opcode execution.
pub enum OpCodeResult {
    /// A single stage of the opcode was completed. The same opcode need to be executed
    /// at least one more time in order to be completed.
    /// The number passed represents the number of cycles consumed by this stage.
    StageDone(Clock),

    /// The opcode was fully completed.
    Done,
}


/// Stores a single instruction from ROM data.
/// This contains the opcode and the address the opcode was read from.
pub struct Instruction {
    /// The opcode being executed by this instruction.
    pub opcode: &'static OpCode,

    /// 16 Bit ID of the opcode being executed.
    /// This includes the 0xCB prefix for extended opcodes.
    pub opcode_id: u16,

    /// The address the opcode starts (or the location 0xCB prefix)
    pub opcode_address: u16,

    /// The address, where the parameters are beginning.
    pub param_address: u16,

    /// The memory where to read the opcode from.
    pub memory: MemoryReadOnlyHandle,
}


impl OpCodeContext {
    /// Creates a context object for an instruction being executed.
    pub fn for_instruction(instruction: &Instruction) -> OpCodeContext {
        OpCodeContext {
            opcode: instruction.opcode,
            cycles: instruction.opcode.cycles,
            stage:  0,
        }
    }

    /// Get the opcode being executed within the current context.
    pub fn get_opcode(&self) -> &'static OpCode {
        self.opcode
    }

    /// Increase the stage index to be executed when invoking the opcode implementation.
    pub fn enter_next_stage(&mut self) {
        self.stage = self.stage + 1;
    }

    /// Get the opcodes current stage.
    pub fn get_stage(&self) -> u8 {
        self.stage
    }

    /// Adds a number of cycles consumed by the current instruction.
    pub fn add_cycles(&mut self, cycles: Clock) {
        self.cycles += cycles;
    }

    /// Get the total amount of cycles consumed for this instruction.
    /// This includes the fetch and execution time of the opcode itself,
    /// as well as the extra time consumed on branching.
    pub fn get_cycles_consumed(&self) -> Clock {
        self.cycles
    }
}


/// Implements the conversion from () to OpCodeResult in order to generate
/// a default value for opcode implementations without specific result code.
impl From<()> for OpCodeResult {
    fn from(_: ()) -> Self {
        OpCodeResult::Done
    }
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
