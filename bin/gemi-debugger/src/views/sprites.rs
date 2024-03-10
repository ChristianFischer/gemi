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

use std::cmp::max;

use egui::{Color32, Grid, Image, Label, ScrollArea, Ui, Vec2, Widget};

use gemi_core::gameboy::GameBoy;

use crate::event::UiEvent;
use crate::highlight::test_selection;
use crate::selection::{Kind, Selected};
use crate::state::{EmulatorState, UiStates};
use crate::ui::sprite_cache;
use crate::ui::style::GemiStyle;
use crate::views::View;


const TOTAL_SPRITES : usize     = 384;
const SPRITE_DISPLAY_SIZE : f32 = 64.0;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpritesView {
    sprite_selected: Option<usize>,
}


impl SpritesView {
    pub fn new() -> Self {
        Self {
            sprite_selected: None,
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
                                                        emu,
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


    fn get_current_selection(&self) -> Option<Selected> {
        self.sprite_selected.map(|index| Selected::Sprite(index))
    }


    fn handle_ui_event(&mut self, event: &UiEvent) {
        match event {
            UiEvent::SelectionChanged(Kind::Focus, Some(Selected::Sprite(sprite_index))) => {
                self.sprite_selected = Some(*sprite_index);
            },

            _ => { }
        }
    }
}


impl SpritesView {
    /// Display a single sprite within the grid.
    fn display_sprite(&mut self, ui: &mut Ui, ui_states: &mut UiStates, emu: &GameBoy, sprite_index: usize) {
        let ppu     = &emu.get_peripherals().ppu;
        let sprite  = ppu.get_sprite_image(sprite_index, 0);
        let texture = sprite_cache::get_texture_for(ui, &sprite);

        let highlight_state = test_selection(Selected::Sprite(sprite_index))
                .of_view(self)
                .compare_with_ui_states(ui_states, emu)
        ;

        // draw background if selected
        if let Some(highlight_state) = highlight_state {
            Self::draw_highlight(ui, highlight_state.get_color(ui));
        }

        // render the image and receive input-response
        let response = Image::new(&texture)
                .fit_to_exact_size(Vec2::splat(SPRITE_DISPLAY_SIZE))
                .sense(egui::Sense::click())
                .ui(ui)
        ;

        // handle hover state
        ui_states.hover.set(Selected::Sprite(sprite_index), response.hovered());

        // handle click
        if response.clicked() {
            ui_states.focus.toggle(Selected::Sprite(sprite_index));
        }
    }


    /// Draw a highlight for the current sprite.
    fn draw_highlight(ui: &mut Ui, color: Color32) {
        let sprite_bounds = egui::Rect::from_min_size(
            ui.cursor().left_top(),
            Vec2::splat(SPRITE_DISPLAY_SIZE)
        ).expand(3.0);

        ui.painter().rect_filled(
            sprite_bounds,
            3.0,
            color,
        );
    }
}
