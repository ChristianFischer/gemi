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

pub mod boot_rom;
pub mod cartridge;
mod client_data;
mod display;
mod gameboy;
mod gameboy_builder;
pub mod audio;
pub mod snapshots;

// re-export the core library as "gemi::core"
pub use gemi_core as core;

pub use display::*;
pub use gameboy::*;
pub use gameboy_builder::*;
