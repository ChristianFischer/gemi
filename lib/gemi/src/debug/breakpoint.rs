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

use crate::core::utils::SerdeSupport;
use std::hash::Hash;


/// A marker trait for types implementing a Breakpoint inside the debugger.
/// Each breakpoint needs to be unique in its area, e.g. representing an unique
/// address within the process memory.
pub trait BreakpointType: Eq + Ord + Hash + Clone + SerdeSupport { }


/// A marker trait for implementing additional conditions when a breakpoint is hit.
/// This can be, for example, the value being written on memory access.
pub trait BreakpointCondition: Default + Clone + SerdeSupport {
    /// The type of the value to be compared, e.g. a byte for memory values.
    type Type;

    /// Tests if a current value (`value`) matches the stored condition.
    fn is_match(&self, value: &Self::Type) -> bool;
}


/// A struct for storing shared breakpoint attributes, common for all kind
/// of breakpoints.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BreakpointAttributes {
    /// Tells whether a breakpoint is enabled or not.
    pub enabled: bool,
}


impl Default for BreakpointAttributes {
    fn default() -> Self {
        Self {
            enabled: true,
        }
    }
}
