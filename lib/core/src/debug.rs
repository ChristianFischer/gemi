/*
 * Copyright (C) 2022-2024 by Christian Fischer
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

use flagset::{flags, FlagSet};

flags! {
    /// An enumeration events that may occur during updating the emulator.
    /// This does not represent actual signals sent from a GameBoy's
    /// original hardware, but is an additional signal to provide feedback
    /// from the emulator's process to its frontend.
    pub enum DebugEvent : u8 {
        /// The PPU completed rendering a line.
        PpuLineCompleted    = 0b_0000_0001,

        /// The PPU completed rendering a frame.
        PpuFrameCompleted   = 0b_0000_0010,
    }
}


/// A set of events occurred during updating the emulator.
pub type DebugEvents = FlagSet<DebugEvent>;
