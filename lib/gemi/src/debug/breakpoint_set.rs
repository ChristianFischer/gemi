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

use crate::debug::breakpoint::{BreakpointAttributes, BreakpointCondition, BreakpointType};
use crate::debug::breakpoints::NullCondition;
use std::collections::HashMap;


#[cfg(feature = "serde")]
use crate::core::utils::SerdeSupport;


/// A set of multiple breakpoints of one particular type.
///
/// Each breakpoint is unique within this set, e.g. there cannot be more than
/// one breakpoint for each single memory address.
///
/// `B` - Type for each breakpoint kind.
/// `C` - Type for an additional breakpoint condition. `NullCondition` per default.
#[derive(Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(bound(
        serialize = "B: SerdeSupport",
        deserialize = "B: SerdeSupport"
    ))
)]
pub struct BreakpointSet<B, C = NullCondition>
where B: BreakpointType,
      C: BreakpointCondition
{
    breakpoints: HashMap<B, BreakpointData<C>>,
}


/// Internal struct storing all attributes of a single breakpoint.
/// This includes the actual [BreakpointAttributes] and any additional
/// [BreakpointCondition].
#[derive(Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(bound(
        serialize = "C: SerdeSupport",
        deserialize = "C: SerdeSupport"
    ))
)]
struct BreakpointData<C>
where C: BreakpointCondition
{
    breakpoint_conditions: C,
    breakpoint_attributes: BreakpointAttributes,
}


impl<B, C> BreakpointSet<B, C>
where B: BreakpointType,
      C: BreakpointCondition
{
    pub fn new() -> Self {
        Self {
            breakpoints: HashMap::new()
        }
    }


    /// Adds a new breakpoint into this set.
    pub fn add_breakpoint(&mut self, breakpoint: B) {
        self.breakpoints.insert(breakpoint, Default::default());
    }


    /// Removes an existing breakpoint from this set.
    pub fn remove_breakpoint(&mut self, breakpoint: &B) {
        self.breakpoints.remove(breakpoint);
    }


    /// Moves an existing breakpoint from `src` to `dst`.
    ///
    /// Returns `true` if the breakpoint at `src` was successfully moved to `dst`.
    ///
    /// Returns `false` if:
    /// - a breakpoint already exists at `dst`
    /// - `src` and `dst` are equal
    /// - no breakpoint exists at `src`
    pub fn move_breakpoint(&mut self, src: &B, dst: B) -> bool {
        // cannot move if the destination already exists
        if self.has_breakpoint(&dst) {
            return false;
        }

        // cannot move if source and destination are equal
        if *src == dst {
            return false;
        }

        if let Some(entry) = self.breakpoints.remove(src) {
            self.breakpoints.insert(dst, entry);
            true
        }
        else {
            false
        }
    }


    /// Get a sequence of all breakpoints within this set.
    pub fn get_breakpoints(&self) -> impl Iterator<Item=&B> + '_ {
        self.breakpoints.keys()
    }


    /// Get a breakpoint's condition.
    pub fn get_breakpoint_condition(&self, breakpoint: &B) -> Option<&C> {
        self.breakpoints.get(breakpoint).map(|d| &d.breakpoint_conditions)
    }


    /// Get a breakpoint's condition.
    pub fn get_breakpoint_condition_mut(&mut self, breakpoint: &B) -> Option<&mut C> {
        self.breakpoints.get_mut(breakpoint).map(|d| &mut d.breakpoint_conditions)
    }


    /// Get a breakpoint's attributes.
    pub fn get_breakpoint_attr(&self, breakpoint: &B) -> Option<&BreakpointAttributes> {
        self.breakpoints.get(breakpoint).map(|d| &d.breakpoint_attributes)
    }


    /// Get a breakpoint's attributes.
    pub fn get_breakpoint_attr_mut(&mut self, breakpoint: &B) -> Option<&mut BreakpointAttributes> {
        self.breakpoints.get_mut(breakpoint).map(|d| &mut d.breakpoint_attributes)
    }


    /// Checks whether this set contains a particular breakpoint.
    pub fn has_breakpoint(&self, breakpoint: &B) -> bool {
        self.breakpoints.contains_key(breakpoint)
    }


    /// Clears all breakpoints from this set.
    pub fn clear(&mut self) {
        self.breakpoints.clear();
    }
}
