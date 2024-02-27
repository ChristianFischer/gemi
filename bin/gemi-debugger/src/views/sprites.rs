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

use std::cmp::max;
use egui::{Grid, Image, Label, ScrollArea, Ui, Vec2, Widget};
use gemi_core::ppu::ppu::Ppu;
use crate::selection::Selected;
use crate::state::{EmulatorState, UiStates};
use crate::ui::sprite_cache;
use crate::ui::style::GemiStyle;
use crate::views::View;


const TOTAL_SPRITES : usize     = 384;
const SPRITE_DISPLAY_SIZE : f32 = 64.0;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpritesView {
    selected_sprite: Option<usize>,
}


impl SpritesView {
    pub fn new() -> Self {
        Self {
            selected_sprite: None,
        }
    }
}


impl View for SpritesView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Sprites"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        if let Some(emu) = state.emu.get_emulator() {
            let ui_states = &mut state.ui;

            let scroll_area = ScrollArea::vertical()
                    .id_source("sprites_scroll_area")
                    .auto_shrink([false, false])
            ;

            let item_spacing    = ui.spacing().item_spacing.y;
            let item_width      = SPRITE_DISPLAY_SIZE + item_spacing;
            let item_height     = SPRITE_DISPLAY_SIZE + item_spacing;
            let cells_per_row   = (ui.available_size().x / item_width).floor() as usize;
            let items_per_row   = max(cells_per_row, 2) - 1;
            let number_of_rows  = (TOTAL_SPRITES + items_per_row - 1) / items_per_row;

            scroll_area.show_rows(
                    ui,
                    item_height,
                    number_of_rows,
                    |ui, display_rows| {
                        let ppu          = &emu.get_peripherals().ppu;

                        Grid::new("sprites_grid")
                                .num_columns(items_per_row)
                                .spacing([item_spacing, item_spacing])
                                .show(ui, |ui| {
                                    for row in display_rows {
                                        let first_sprite_in_row = row * items_per_row;

                                        // display the address of the first sprite in the row
                                        ui.vertical(|ui| {
                                            let address = 0x8000 + first_sprite_in_row * 16;
                                            let address_str = format!("{:04x}", address);
                                            let address_text = GemiStyle::ADDRESS.rich_text(address_str);

                                            ui.add_sized(
                                                    [SPRITE_DISPLAY_SIZE, 0.0],
                                                    Label::new(address_text)
                                            );
                                        });

                                        // display the sprites in the remaining cells
                                        for item_in_row in 0..items_per_row {
                                            let sprite_index = first_sprite_in_row + item_in_row;

                                            if sprite_index < TOTAL_SPRITES {
                                                self.display_sprite(
                                                        ui,
                                                        ui_states,
                                                        ppu,
                                                        sprite_index
                                                );
                                            }
                                        }

                                        ui.end_row();
                                    }
                                })
                    });
        }
    }
}


impl SpritesView {
    /// Select a single sprite in this list by it's index.
    pub fn select_sprite(&mut self, ui_states: &mut UiStates, sprite_index: usize) {
        ui_states.selection.select(Selected::Sprite(sprite_index));
        self.selected_sprite = Some(sprite_index);
    }


    /// Deselect a specific sprite if it was selected before.
    pub fn clear_selection(&mut self, ui_states: &mut UiStates) {
        if let Some(sprite_index) = self.selected_sprite {
            ui_states.selection.clear(Selected::Sprite(sprite_index));
            self.selected_sprite = None;
        }
    }
}


impl SpritesView {
    /// Display a single sprite within the grid.
    fn display_sprite(&mut self, ui: &mut Ui, ui_states: &mut UiStates, ppu: &Ppu, sprite_index: usize) {
        let sprite  = ppu.get_sprite_image(sprite_index, 0);
        let texture = sprite_cache::get_texture_for(ui, &sprite);

        let is_selected = self.selected_sprite == Some(sprite_index);

        // draw background if selected
        if is_selected {
            let sprite_bounds = egui::Rect::from_min_size(
                ui.cursor().left_top(),
                Vec2::splat(SPRITE_DISPLAY_SIZE)
            ).expand(3.0);

            ui.painter().rect_filled(
                sprite_bounds,
                3.0,
                GemiStyle::BACKGROUND_HIGHLIGHT,
            );
        }

        let response = Image::new(&texture)
                .fit_to_exact_size(Vec2::splat(SPRITE_DISPLAY_SIZE))
                .sense(egui::Sense::click())
                .ui(ui)
        ;

        if response.clicked() {
            return if is_selected {
                self.clear_selection(ui_states)
            }
            else {
                self.select_sprite(ui_states, sprite_index)
            }
        }
    }
}
