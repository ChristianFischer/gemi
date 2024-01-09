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

use eframe::emath::{Align, Vec2};
use egui::{ComboBox, Direction, Image, Label, Layout, TextStyle, Ui, Widget};
use egui_extras::{Column, TableBuilder, TableRow};
use gemi_core::ppu::ppu::Ppu;
use crate::event::UiEvent;
use crate::state::EmulatorState;
use crate::ui::sprite_cache;
use crate::ui::style::GemiStyle;
use crate::view_response::ViewResponse;
use crate::views::View;


const OAM_ENTRIES : usize       = 40;
const SPRITE_DISPLAY_SIZE : f32 = 16.0;



/// Name entries for Video banks
const BANK_NAMES : [&str; 2] = [
    "Bank #0",
    "Bank #1",
];


/// Name entries for palettes when in classic GameBoy emulation.
const DMG_PALETTE_NAMES : [&str; 2] = [
    "OBP0",
    "OBP1",
];


/// Name entries for palettes when in GameBoy Color emulation.
const GBC_PALETTE_NAMES : [&str; 8] = [
    "#0",
    "#1",
    "#2",
    "#3",
    "#4",
    "#5",
    "#6",
    "#7",
];


#[derive(serde::Serialize, serde::Deserialize)]
pub struct OamView {
    selected_entry: Option<usize>,
}


impl OamView {
    /// Creates a new [`crate::views::memory::MemoryView`] object.
    pub fn new() -> Self {
        Self {
            selected_entry: None,
        }
    }
}


impl View for OamView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "OAM"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) -> ViewResponse {
        let mut response = ViewResponse::none();
        let is_paused = state.is_paused();

        if let Some(emu) = state.get_emulator_mut() {
            let is_gbc      = emu.get_config().is_gbc_enabled();
            let text_height = ui.text_style_height(&TextStyle::Monospace);

            // take the highest value of either the text height or the sprite display size
            let item_height = if text_height > SPRITE_DISPLAY_SIZE {
                text_height
            }
            else {
                SPRITE_DISPLAY_SIZE
            };

            TableBuilder::new(ui)
                    .column(Column::exact(SPRITE_DISPLAY_SIZE)) // image
                    .column(Column::auto().resizable(true))     // tile index
                    .column(Column::auto().resizable(true))     // x position
                    .column(Column::auto().resizable(true))     // y position
                    .column(Column::auto().resizable(true))     // x flip
                    .column(Column::auto().resizable(true))     // y flip
                    .column(Column::auto().resizable(true))     // priority
                    .column(Column::auto().resizable(true))     // tile bank
                    .column(Column::auto().resizable(true))     // palette
                    .column(Column::remainder())                // empty remainder

                    .vscroll(true)
                    .striped(true)
                    .sense(egui::Sense::click())

                    .header(text_height, |mut header| {
                        header.col(|_|  {                           } );
                        header.col(|ui| { ui.heading("Tile");       } );
                        header.col(|ui| { ui.heading("X");          } );
                        header.col(|ui| { ui.heading("Y");          } );
                        header.col(|ui| { ui.heading("FlipX");      } );
                        header.col(|ui| { ui.heading("FlipY");      } );
                        header.col(|ui| { ui.heading("Prio");       } );
                        header.col(|ui| { ui.heading("Bank");       } );
                        header.col(|ui| { ui.heading("Palette");    } );
                        header.col(|_|  {                           } );
                    })

                    .body(|body| {
                        body.rows(
                                item_height,
                                OAM_ENTRIES,
                                |row| {
                                    let row_response = self.display_entry(
                                            row,
                                            &mut emu.get_peripherals_mut().ppu,
                                            item_height,
                                            is_paused,
                                            is_gbc
                                    );

                                    response.add(row_response);
                                }
                        )
                    })
            ;
        }

        response
    }


    fn handle_ui_event(&mut self, event: &UiEvent) {
        _ = event;
    }


    fn on_emulator_loaded(&mut self, state: &mut EmulatorState) {
        _ = state;
    }
}


impl OamView {
    /// Select a single entry in this list by it's index.
    pub fn select_entry(&mut self, oam_index: usize) -> ViewResponse {
        self.selected_entry = Some(oam_index);

        ViewResponse::event(UiEvent::OamEntrySelected(oam_index))
    }


    /// Deselect a specific entry if it was selected before.
    pub fn clear_selection(&mut self) -> ViewResponse {
        if let Some(oam_index) = self.selected_entry {
            self.selected_entry = None;

            ViewResponse::event(UiEvent::OamEntryDeselected(oam_index))
        }
        else {
            ViewResponse::none()
        }
    }
}


impl OamView {
    fn display_entry(
            &mut self,
            mut table_row: TableRow,
            ppu: &mut Ppu,
            row_height: f32,
            is_paused: bool,
            is_gbc: bool
    ) -> ViewResponse {
        let oam_index  = table_row.index();
        let tile_index = ppu.get_oam_mut()[oam_index].tile as usize;
        let bank       = ppu.get_oam()[oam_index].get_gbc_vram_bank();
        let sprite     = ppu.get_sprite_image(tile_index, bank);
        let entry      = &mut ppu.get_oam_mut()[oam_index];
        let style      = GemiStyle::VALUE_WRITABLE;

        let is_selected = self.selected_entry == Some(oam_index);

        // draw background if selected
        if is_selected {
            table_row.set_selected(true);
        }

        // Sprite image
        table_row.col(|ui| {
            let texture    = sprite_cache::get_texture_for(ui, &sprite);

            Image::new(&texture)
                    .fit_to_exact_size(Vec2::splat(row_height))
                    .ui(ui)
            ;
        });

        // Tile index
        table_row.col(|ui| {
            ui.with_layout(
                    Layout::right_to_left(Align::Max),
                    |ui| Label::new(style.rich_text(format!("{}", tile_index))).ui(ui)
            );
        });

        // X position
        table_row.col(|ui| {
            ui.with_layout(
                    Layout::right_to_left(Align::Max),
                    |ui| Label::new(style.rich_text(format!("{}", entry.pos_x))).ui(ui)
            );
        });

        // Y position
        table_row.col(|ui| {
            ui.with_layout(
                    Layout::right_to_left(Align::Max),
                    |ui| Label::new(style.rich_text(format!("{}", entry.pos_y))).ui(ui)
            );
        });

        // Flip X
        table_row.col(|ui| {
            ui.with_layout(
                    Layout::centered_and_justified(Direction::TopDown),
                    |ui| ui.checkbox(&mut entry.is_flip_x(), "")
            );
        });

        // Flip Y
        table_row.col(|ui| {
            ui.with_layout(
                    Layout::centered_and_justified(Direction::TopDown),
                    |ui| ui.checkbox(&mut entry.is_flip_y(), "")
            );
        });

        // Priority
        table_row.col(|ui| {
            ui.with_layout(
                    Layout::centered_and_justified(Direction::TopDown),
                    |ui| ui.checkbox(&mut entry.is_bg_priority(), "")
            );
        });

        // Bank
        table_row.col(|ui| {
            let mut new_bank = bank as usize;

            if !is_paused {
                ui.label(BANK_NAMES[bank as usize]);
            }
            else {
                ComboBox::from_id_source(format!("oam{oam_index}_bank"))
                        .width(ui.available_width())
                        .show_index(
                                ui,
                                &mut new_bank,
                                BANK_NAMES.len(),
                                |i| BANK_NAMES[i],
                        )
                ;
            }
        });

        // Palette
        table_row.col(|ui| {
            if is_gbc {
                // GBC palette
                let current_palette = entry.get_color_palette();
                let mut new_palette = current_palette as usize;

                ComboBox::from_id_source(format!("oam{oam_index}_gbc_palette"))
                        .width(ui.available_width())
                        .show_index(
                                ui,
                                &mut new_palette,
                                GBC_PALETTE_NAMES.len(),
                                |i| GBC_PALETTE_NAMES[i],
                        )
                ;
            }
            else {
                // DMG palette
                let current_palette = entry.get_dmg_palette();
                let mut new_palette = current_palette as usize;

                ComboBox::from_id_source(format!("oam{oam_index}_dmg_palette"))
                        .width(ui.available_width())
                        .show_index(
                                ui,
                                &mut new_palette,
                                DMG_PALETTE_NAMES.len(),
                                |i| DMG_PALETTE_NAMES[i],
                        )
                ;
            }
        });

        // Remainder
        table_row.col(|_| { });

        // handle click
        if table_row.response().clicked() {
            return if is_selected {
                self.clear_selection()
            }
            else {
                self.select_entry(oam_index)
            }
        }

        ViewResponse::none()
    }
}