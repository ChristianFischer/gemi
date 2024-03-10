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

use std::ops::{Add, Div, Sub};

use eframe::emath::Rect;
use eframe::epaint::{ColorImage, Stroke};
use eframe::epaint::textures::TextureOptions;
use egui::{Color32, Image, Pos2, Sense, Ui, Vec2, vec2, Widget};

use gemi_core::gameboy::GameBoy;
use gemi_core::ppu::flags::LcdControlFlag;
use gemi_core::ppu::graphic_data::Sprite;
use gemi_core::ppu::ppu::{SCREEN_H, SCREEN_W};

use crate::highlight::test_selection;
use crate::selection::Selected;
use crate::state::{EmulatorState, UiStates};
use crate::views::View;


/// The main view to show the emulator's display.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmulatorDisplayView {

}


impl View for EmulatorDisplayView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Display"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        match state.emu.get_emulator() {
            None => {}

            Some(emu) => {
                self.render_display_image(ui, emu, &mut state.ui);
            }
        }
    }
}


impl EmulatorDisplayView {
    /// Render the display image of the currently running emulator.
    fn render_display_image(&self, ui: &mut Ui, emu: &GameBoy, ui_states: &mut UiStates) {
        let lcd    = emu.get_peripherals().ppu.get_lcd();
        let size   = [lcd.get_width() as _, lcd.get_height() as _];
        let pixels = lcd.get_pixels_as_slice();

        // create a texture from the pixel data
        let image   = ColorImage::from_rgba_unmultiplied(size, pixels);
        let texture = ui.ctx().load_texture("display", image, TextureOptions::NEAREST);

        let texture_size   = texture.size_vec2();
        let available_size = ui.available_size();

        // compute the scale factor to fit the image into the available space
        // (but only whole numbers and not smaller than 1)
        let scale = f32::max(
            1.0,
            f32::min(
                available_size.x / texture_size.x,
                available_size.y / texture_size.y
            )
        ).floor();

        // store the origin of the draw area
        let origin = ui.cursor().left_top();

        // render the texture
        Image::new(&texture)
                .fit_to_exact_size(texture_size * scale)
                .ui(ui)
        ;
        
        self.handle_interactions(ui, ui_states, emu, origin, scale);

        self.render_selection_overlays(ui, ui_states, emu, origin, scale);
    }


    /// Handles interactions of the user with the UI.
    fn handle_interactions(&self, ui: &mut Ui, ui_states: &mut UiStates, emu: &GameBoy, origin: Pos2, scale: f32) {
        let display_bounds = Rect::from_min_size(
                origin,
                Vec2::new(
                        SCREEN_W as f32, 
                        SCREEN_H as f32
                ) * scale
        );

        // listen for click and hover interactions
        let response = ui.interact(
                display_bounds, 
                ui.id().with(1), 
                Sense::click()
        );
        
        if response.hovered() {
            if let Some(hover_pos) = ui.input(|input| input.pointer.hover_pos()) {
                let ppu           = &emu.get_peripherals().ppu;
                let oam           = ppu.get_oam();
                let large_sprites = ppu.check_lcdc(LcdControlFlag::SpritesSize);
                let sprite_size   = if large_sprites { 16 } else { 8 };

                // transform the position of the mouse cursor into OAM position space
                let oam_pos = hover_pos
                        .sub(display_bounds.left_top())
                        .div(scale)
                        .add(vec2(8.0, 16.0))
                        .to_pos2()
                ;

                // test for all OAM entries
                for oam_index in 0..40 {
                    let oam_entry  = oam[oam_index];
                    let oam_bounds = Rect::from_min_size(
                            Pos2::new(oam_entry.pos_x as f32, oam_entry.pos_y as f32),
                            Vec2::new(8.0, sprite_size as f32)
                    );

                    let hit = oam_bounds.contains(oam_pos);
                    ui_states.hover.set(Selected::OamEntry(oam_index), hit);

                    if hit && response.clicked() {
                        ui_states.focus.toggle(Selected::OamEntry(oam_index));
                    }
                }
            }
        }
    }


    /// Render overlays on the display of the currently running emulator to
    /// highlight any currently selected sprites and tiles.
    fn render_selection_overlays(&self, ui: &mut Ui, ui_states: &mut UiStates, emu: &GameBoy, origin: Pos2, scale: f32) {
        let ppu           = &emu.get_peripherals().ppu;
        let oam           = ppu.get_oam();
        let large_sprites = ppu.check_lcdc(LcdControlFlag::SpritesSize);

        for oam_index in 0..40 {
            let entry = &oam[oam_index];

            // check whether this item is highlighted or not
            let highlight_state = test_selection(Selected::OamEntry(oam_index))
                    .of_view(self)
                    .compare_with_ui_states(ui_states, emu)
            ;

            // render highlight if selected
            if let Some(highlight_state) = highlight_state {
                let highlight_color = highlight_state.get_color(ui);

                Self::render_sprite_outline(
                        ui,
                        &entry,
                        origin,
                        highlight_color,
                        large_sprites,
                        scale,
                );
            }
        }
    }


    /// Draws an outline on the location where an entry from the OAM will
    /// be displayed.
    fn render_sprite_outline(ui: &mut Ui, sprite: &Sprite, origin: Pos2, color: Color32, large: bool, scale: f32) {
        let sprite_size = if large { 16 } else { 8 };

        let sprite_bounds = Rect::from_min_size(
            Pos2::new(
                origin.x + ((sprite.pos_x as f32 -  8.0) * scale),
                origin.y + ((sprite.pos_y as f32 - 16.0) * scale)
            ),
            Vec2::new(
                8.0 * scale,
                sprite_size as f32 * scale
            )
        );

        ui.painter().rect_stroke(
            sprite_bounds,
            2.0,
            Stroke::new(2.0, color)
        );
    }
}
