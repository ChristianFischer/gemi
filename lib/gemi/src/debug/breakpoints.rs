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

pub use breakpoint_impl::*;
pub use condition_impl::*;




/// A debugger event storing a breakpoint being hit.
#[derive(Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Breakpoint {
    /// The instruction pointer was set to the address of the breakpoint,
    /// but the instruction was not executed yet.
    InstructionHit(AddressBreakpoint),

    /// The program did write to the memory address of the breakpoint.
    /// The value was written and the execution of the recent instruction was completed.
    /// If the breakpoint interception has provided a replacement value, the replacement
    /// value was written instead of the original one.
    /// This event stores the original value that was intended to be written.
    MemoryWritten(AddressBreakpoint),

    /// An interrupt was triggered by the system.
    /// The interrupt itself was not processed yet. Depending on the state of
    /// IE and IF registers, the interrupt processing may be skipped, if either
    /// not enabled via IE flags or already be enqueued in the IF register.
    InterruptTriggered(InterruptBreakpoint),

    /// A particular line was finished rendering. This also includes the
    /// "invisible" lines outside the visible screen.
    PpuLineFinished(PpuLineBreakpoint),

    /// A frame was finished rendering.
    PpuFrameFinished(PpuFrameBreakpoint),
}


mod breakpoint_impl {
    use crate::debug::breakpoint::BreakpointType;
    use gemi_core::cpu::interrupts::Interrupt;


    /// A breakpoint type to match 16bit memory adresses.
    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct AddressBreakpoint(pub u16);


    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct InterruptBreakpoint(pub Interrupt);


    /// A breakpoint type to match individual scanlines of the emulator display.
    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct PpuLineBreakpoint(pub u8);


    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct PpuFrameBreakpoint();


    impl BreakpointType for AddressBreakpoint { }
    impl BreakpointType for InterruptBreakpoint { }
    impl BreakpointType for PpuLineBreakpoint { }
    impl BreakpointType for PpuFrameBreakpoint { }
}


mod condition_impl {
    use crate::debug::breakpoint::BreakpointCondition;
    use std::ops::Range;


    /// A `Null` condition that always matches.
    /// This can be used as a placeholder when no other condition is to be implemented.
    #[derive(Clone, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct NullCondition();


    /// A condition matching a range of byte values.
    #[derive(Clone, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct ByteRangeCondition(Option<Range<u8>>);


    impl BreakpointCondition for NullCondition {
        type Type = ();

        fn is_match(&self, _value: &Self::Type) -> bool {
            true
        }
    }


    impl BreakpointCondition for ByteRangeCondition {
        type Type = u8;

        fn is_match(&self, value: &Self::Type) -> bool {
            match &self.0 {
                Some(range) => range.contains(value),
                None => true,
            }
        }
    }
}
