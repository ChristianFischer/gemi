/*
 * Copyright (C) 2022-2023 by Christian Fischer
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

use egui::Ui;
use crate::state::EmulatorState;
use crate::views::cartridge_info::CartridgeInfoView;
use crate::views::display::EmulatorDisplayView;
use crate::views::memory::MemoryView;
use crate::views::placeholder::PlaceholderView;

pub mod cartridge_info;
pub mod display;
pub mod memory;
pub mod placeholder;


/// A trait to be implemented by view objects of which each of them display
/// their unique aspect of information about the currently running emulator.
pub trait View: serde::Serialize + serde::de::DeserializeOwned {
    /// Get the title of the view.
    fn title(&self, state: &mut EmulatorState) -> &str;

    /// Render the view UI.
    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui);

    /// Invoked when a new instance of the emulator was created.
    fn on_emulator_loaded(&mut self, state: &mut EmulatorState) {
        _ = state;
    }
}


/// An enum to store the different view classes.
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ViewClass {
    Display(EmulatorDisplayView),
    CartridgeInfo(CartridgeInfoView),
    Cpu(PlaceholderView),
    Memory(MemoryView),
    Disassembly(PlaceholderView),
    TileMap(PlaceholderView),
    Sprites(PlaceholderView),
}


impl ViewClass {
    /// Creates a new [`EmulatorDisplayView`] object.
    pub fn new_display_view() -> ViewClass {
        ViewClass::Display(EmulatorDisplayView { })
    }


    /// Creates a new [`CartridgeInfoView`] object.
    pub fn new_cartridge_info() -> ViewClass {
        ViewClass::CartridgeInfo(CartridgeInfoView::new())
    }


    /// Placeholder for the CPU view.
    pub fn new_cpu() -> ViewClass {
        ViewClass::Cpu(PlaceholderView::new("CPU"))
    }


    /// Placeholder for the memory view.
    pub fn new_memory() -> ViewClass {
        ViewClass::Memory(MemoryView::new())
    }


    /// Placeholder for the disassembly view.
    pub fn new_disassembly() -> ViewClass {
        ViewClass::Disassembly(PlaceholderView::new("Disassembly"))
    }


    /// Placeholder for the tile map view.
    pub fn new_tile_map() -> ViewClass {
        ViewClass::TileMap(PlaceholderView::new("Tile Map"))
    }


    /// Placeholder for the sprites view.
    pub fn new_sprites() -> ViewClass {
        ViewClass::Sprites(PlaceholderView::new("Sprites"))
    }
}


impl View for ViewClass {
    fn title(&self, state: &mut EmulatorState) -> &str {
        match self {
            ViewClass::Display(v)       => v.title(state),
            ViewClass::CartridgeInfo(v) => v.title(state),
            ViewClass::Cpu(v)           => v.title(state),
            ViewClass::Memory(v)        => v.title(state),
            ViewClass::Disassembly(v)   => v.title(state),
            ViewClass::TileMap(v)       => v.title(state),
            ViewClass::Sprites(v)       => v.title(state),
        }
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        match self {
            ViewClass::Display(v)       => v.ui(state, ui),
            ViewClass::CartridgeInfo(v) => v.ui(state, ui),
            ViewClass::Cpu(v)           => v.ui(state, ui),
            ViewClass::Memory(v)        => v.ui(state, ui),
            ViewClass::Disassembly(v)   => v.ui(state, ui),
            ViewClass::TileMap(v)       => v.ui(state, ui),
            ViewClass::Sprites(v)       => v.ui(state, ui),
        }
    }


    fn on_emulator_loaded(&mut self, state: &mut EmulatorState) {
        match self {
            ViewClass::Display(v)       => v.on_emulator_loaded(state),
            ViewClass::CartridgeInfo(v) => v.on_emulator_loaded(state),
            ViewClass::Cpu(v)           => v.on_emulator_loaded(state),
            ViewClass::Memory(v)        => v.on_emulator_loaded(state),
            ViewClass::Disassembly(v)   => v.on_emulator_loaded(state),
            ViewClass::TileMap(v)       => v.on_emulator_loaded(state),
            ViewClass::Sprites(v)       => v.on_emulator_loaded(state),
        }
    }
}
