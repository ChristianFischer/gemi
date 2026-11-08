/*
 * Copyright (C) 2026 by Christian Fischer
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

use crate::debug::breakpoint_set::BreakpointSet;
use crate::debug::breakpoints::*;


/// A set of instruction breakpoints to react on instruction pointer change.
pub type InstructionBreakpoints = BreakpointSet<AddressBreakpoint>;

/// A set of memory access breakpoints to react on read and write events on the device memory.
pub type MemoryAccessBreakpoints = BreakpointSet<AddressBreakpoint, ByteRangeCondition>;

/// A set of interrupt breakpoints to react on hardware interrupts of the emulated device.
pub type InterruptBreakpoints = BreakpointSet<InterruptBreakpoint>;

/// A set of line breakpoints to react when the actively drawn scanline switches.
pub type PpuLineBreakpoints = BreakpointSet<PpuLineBreakpoint>;

/// A set of breakpoints to react on frame completion.
pub type PpuFrameBreakpoints = BreakpointSet<PpuFrameBreakpoint>;


/// A table of all breakpoints configured for a debugger instance.
/// This table combines sets of various kinds of breakpoints.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BreakpointTable {
    pub instruction_breakpoints: InstructionBreakpoints,

    pub memory_access_breakpoints: MemoryAccessBreakpoints,

    pub interrupt_breakpoints: InterruptBreakpoints,

    pub ppu_line_breakpoints: PpuLineBreakpoints,

    pub ppu_frame_breakpoints: PpuFrameBreakpoints,
}


impl BreakpointTable {
    pub fn new() -> Self {
        Self {
            instruction_breakpoints:    InstructionBreakpoints::new(),
            memory_access_breakpoints:  MemoryAccessBreakpoints::new(),
            interrupt_breakpoints:      InterruptBreakpoints::new(),
            ppu_line_breakpoints:       PpuLineBreakpoints::new(),
            ppu_frame_breakpoints:      PpuFrameBreakpoints::new(),
        }
    }


    /// Checks whether this debugger contains a particular breakpoint.
    pub fn has_breakpoint(&self, breakpoint: &Breakpoint) -> bool {
        match breakpoint {
            Breakpoint::InstructionHit(b) => self.instruction_breakpoints.has_breakpoint(b),
            Breakpoint::MemoryWritten(b) => self.memory_access_breakpoints.has_breakpoint(b),
            Breakpoint::InterruptTriggered(b) => self.interrupt_breakpoints.has_breakpoint(b),
            Breakpoint::PpuLineFinished(b) => self.ppu_line_breakpoints.has_breakpoint(b),
            Breakpoint::PpuFrameFinished(b) => self.ppu_frame_breakpoints.has_breakpoint(b),
        }
    }
}


impl Default for BreakpointTable {
    fn default() -> Self {
        Self::new()
    }
}
