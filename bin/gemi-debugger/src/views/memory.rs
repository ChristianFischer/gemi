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
use egui_memory_editor::MemoryEditor;
use crate::state::EmulatorState;
use crate::views::View;


/// A view to display the emulator's memory.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MemoryView {
    #[serde(skip)]
    #[serde(default = "make_memory_editor_default")]
    memory_editor: MemoryEditor,
}


/// Creating the default instance of a memory editor with all relevant address ranges.
fn make_memory_editor_default() -> MemoryEditor {
    MemoryEditor::new()
        .with_address_range("ALL",              0x0000..0x01_0000)
        .with_address_range("ROM Bank #0",      0x0000..0x00_4000)
        .with_address_range("ROM Bank #1",      0x4000..0x00_8000)
        .with_address_range("VRAM",             0x8000..0x00_a000)
        .with_address_range("Cartridge RAM",    0xa000..0x00_c000)
        .with_address_range("WRAM Bank #0",     0xc000..0x00_d000)
        .with_address_range("WRAM Bank #1",     0xd000..0x00_e000)
        .with_address_range("OAM",              0xfe00..0x00_feA0)
        .with_address_range("IO",               0xff00..0x00_ff80)
        .with_address_range("HRAM",             0xff80..0x00_ffff)
}


impl MemoryView {
    /// Creates a new [`MemoryView`] object.
    pub fn new() -> Self {
        Self {
            memory_editor: make_memory_editor_default(),
        }
    }
}


impl View for MemoryView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Memory"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        self.memory_editor.draw_editor_contents(
            ui,
            state,

            // reading memory
            |state, address| {
                match (state.get_emulator(), address) {
                    // we can only read an address, if we have a running emulator and
                    // the address is in a valid 16 bit range
                    (Some(emu), 0x0000 ..= 0xffff) => {
                        let value = emu.get_mmu().read_u8(address as u16);
                        Some(value)
                    }

                    _ => None,
                }
            },

            // writing memory
            |state, address, value| {
                match state.get_emulator_mut() {
                    // we can only read an address, if we have a running emulator
                    Some(emu) => {
                        // only some address ranges are writable
                        match address {
                            /* VRAM",             */    0x8000..=0x9FFF
                            /* Cartridge RAM",    */  | 0xA000..=0xBFFF
                            /* WRAM Bank #0",     */  | 0xC000..=0xCFFF
                            /* WRAM Bank #1",     */  | 0xD000..=0xDFFF
                            /* OAM",              */  | 0xFE00..=0xFE9F
                            /* HRAM",             */  | 0xFF80..=0xFFFE => {
                                emu.get_mmu_mut().write_u8(address as u16, value);
                            }

                            _ => { }
                        }
                    }

                    _ => { },
                };
            }
        );
    }
}
