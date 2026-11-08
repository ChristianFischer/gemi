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

use crate::core::cpu::interrupts::Interrupt;
use crate::core::debug::DebugListener;

use crate::debug::{
    breakpoint::BreakpointCondition,
    breakpoint_table::BreakpointTable,
    breakpoints::{AddressBreakpoint, Breakpoint, InterruptBreakpoint, PpuFrameBreakpoint, PpuLineBreakpoint},
};


/// A debugger observing the emulator device.
///
/// The debugger watches for particular events and compares them with
/// conditions stored in its [BreakpointTable].
/// On match, the breakpoint will be stored in an event table. Any client has
/// to check for pending events to suspend the emulator execution.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Debugger {
    breakpoints: BreakpointTable,

    events: Vec<DebugEvent>,
}


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DebugEvent {
    /// The breakpoint that caused the event.
    pub breakpoint: Breakpoint,

    /// An optional parameter to the debug event.
    pub param: u32,
}


impl Debugger {
    pub fn new() -> Self {
        Self {
            breakpoints: BreakpointTable::new(),
            events: Vec::new(),
        }
    }


    /// Get the current [BreakpointTable] stored in this debugger.
    pub fn get_breakpoints_table(&self) -> &BreakpointTable {
        &self.breakpoints
    }


    /// Assigns a new [BreakpointTable] to this debugger.
    pub fn set_breakpoints_table(&mut self, breakpoints: BreakpointTable) {
        self.breakpoints = breakpoints;
    }


    /// Checks if the debugger is currently halted by a specific breakpoint.
    pub fn is_halted_by(&self, breakpoint: Breakpoint) -> bool {
        self.events.iter().any(|event| event.breakpoint == breakpoint)
    }


    /// Get any pending events from this debugger.
    /// This access is readonly and does not remove events from the debugger.
    pub fn get_events(&self) -> &[DebugEvent] {
        &self.events
    }


    /// Takes any pending events from the debugger.
    /// This call removes all events from the emulator table and provides them to the caller.
    pub fn take_events(&mut self) -> Vec<DebugEvent> {
        std::mem::take(&mut self.events)
    }


    /// Takes the next pending event from the debugger.
    /// This call removes the first event from the list of events and provides it to the caller.
    pub fn take_event(&mut self) -> Option<DebugEvent> {
        self.events.drain(0..1).next()
    }


    /// Removes any remaining events from the debugger.
    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}


impl DebugListener for Debugger {
    fn on_instruction_pointer_changed(&mut self, address: u16) {
        let breakpoint = AddressBreakpoint(address);
        if let Some(attr) = self.breakpoints.instruction_breakpoints.get_breakpoint_attr(&breakpoint) {
            if attr.enabled {
                self.events.push(DebugEvent {
                    breakpoint: Breakpoint::InstructionHit(breakpoint),
                    param: 0
                });
            }
        }
    }


    fn on_mem_write(&mut self, addr: u16, value: u8) -> Option<u8> {
        let breakpoint = AddressBreakpoint(addr);
        if let Some(attr) = self.breakpoints.memory_access_breakpoints.get_breakpoint_attr(&breakpoint) {
            if let Some(cond) = self.breakpoints.memory_access_breakpoints.get_breakpoint_condition(&breakpoint) {
                if attr.enabled && cond.is_match(&value) {
                    self.events.push(DebugEvent {
                        breakpoint: Breakpoint::MemoryWritten(breakpoint),
                        param: value as u32
                    });
                }
            }
        }

        None
    }


    fn on_interrupt_triggered(&mut self, interrupt: Interrupt) {
        let breakpoint = InterruptBreakpoint(interrupt);
        if let Some(attr) = self.breakpoints.interrupt_breakpoints.get_breakpoint_attr(&breakpoint) {
            if attr.enabled {
                self.events.push(DebugEvent {
                    breakpoint: Breakpoint::InterruptTriggered(breakpoint),
                    param: 0
                });
            }
        }
    }


    fn on_ppu_line_completed(&mut self, line: u8) {
        let breakpoint = PpuLineBreakpoint(line);
        if let Some(attr) = self.breakpoints.ppu_line_breakpoints.get_breakpoint_attr(&breakpoint) {
            if attr.enabled {
                self.events.push(DebugEvent {
                    breakpoint: Breakpoint::PpuLineFinished(breakpoint),
                    param: line as u32
                });
            }
        }
    }


    fn on_ppu_frame_completed(&mut self) {
        let breakpoint = PpuFrameBreakpoint();
        if let Some(attr) = self.breakpoints.ppu_frame_breakpoints.get_breakpoint_attr(&breakpoint) {
            if attr.enabled {
                self.events.push(DebugEvent {
                    breakpoint: Breakpoint::PpuFrameFinished(breakpoint),
                    param: 0
                });
            }
        }
    }


    fn is_halted(&self) -> bool {
        !self.events.is_empty()
    }
}
