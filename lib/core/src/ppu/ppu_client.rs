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

use crate::ppu::graphic_data::{Color, DmgLcdPixel};


/// A trait representing the interface between the PPU and the client application.
///
/// The PPU will use this trait to send output from the pixel processing to
/// the display.
///
/// The client will receive pixel data in either monochrome classic GameBoy
/// format (DMG) or color pixel data (CGB).
/// When receiving DMG pixel data, the client is responsible for converting
/// them into RGB color values.
pub trait PpuClient {
    /// Notifies the client to clear the screen.
    fn clear_screen(&mut self);

    /// Sends a single pixel to the client.
    /// The pixel is sent as a monochrome GameBoy pixel.
    fn put_dmg_pixel(&mut self, x: u32, y: u32, pixel: DmgLcdPixel);

    /// Sends a single pixel to the client.
    /// The pixel is sent as a color pixel when the emulator is running in GameBoy Color mode.
    fn put_color_pixel(&mut self, x: u32, y: u32, color: Color);

    /// Notifies the client that a frame was fully processed.
    fn on_frame_finished(&mut self);
}



/// A dummy implementation of the PpuClient trait without functionality.
#[derive(Default)]
pub struct NullPpuClient;

impl PpuClient for NullPpuClient {
    fn clear_screen(&mut self) {}

    fn put_dmg_pixel(&mut self, _x: u32, _y: u32, _pixel: DmgLcdPixel) {}

    fn put_color_pixel(&mut self, _x: u32, _y: u32, _color: Color) {}

    fn on_frame_finished(&mut self) {}
}


