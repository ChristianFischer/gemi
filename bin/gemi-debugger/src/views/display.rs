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

use eframe::epaint::ColorImage;
use eframe::epaint::textures::TextureOptions;
use egui::Ui;
use gemi_core::gameboy::GameBoy;
use crate::state::EmulatorState;
use crate::view_response::ViewResponse;
use crate::views::View;


/// The main view to show the emulator's display.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmulatorDisplayView {

}


impl View for EmulatorDisplayView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Display"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) -> ViewResponse {
        match state.get_emulator() {
            None => {}

            Some(emu) => {
                Self::render_display_image(ui, emu);
            }
        }

        ViewResponse::none()
    }
}


impl EmulatorDisplayView {
    /// Render the display image of the currently running emulator.
    fn render_display_image(ui: &mut Ui, emu: &GameBoy) {
        let lcd    = emu.get_peripherals().ppu.get_lcd();
        let size   = [lcd.get_width() as _, lcd.get_height() as _];
        let pixels = lcd.get_pixels_as_slice();

        // create a texture from the pixel data
        let image   = ColorImage::from_rgba_unmultiplied(size, pixels);
        let texture = ui.ctx().load_texture("display", image, TextureOptions::NEAREST);

        let texture_size = texture.size_vec2();
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

        // render the texture
        ui.image(
            &texture,
            texture_size * scale
        );
    }
}
