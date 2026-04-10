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

use crate::mmu::memory_data::mapped::MemoryDataMapped;
use crate::mmu::memory_data::MemoryData;
use crate::ppu::graphic_data::Color;
use crate::ppu::ppu::{SCREEN_H, SCREEN_PIXELS, SCREEN_W};
use crate::utils::SerializableArray;


type PixelArray160x144  = SerializableArray<Color, SCREEN_PIXELS>;
type PixelBuffer160x144 = MemoryDataMapped<PixelArray160x144>;


/// Storage for the pixel content of the LCD screen.
/// Allows to manipulate and retrieve pixel data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LcdBuffer {
    pixels: PixelBuffer160x144,
}


impl LcdBuffer {
    /// Allocate a new buffer with a solid white color.
    pub fn alloc() -> LcdBuffer {
        LcdBuffer::alloc_with_color(Color::white())
    }

    /// Allocate a new buffer with a solid color.
    pub fn alloc_with_color(color: Color) -> LcdBuffer {
        LcdBuffer {
            pixels: PixelBuffer160x144::new([color; SCREEN_PIXELS])
        }
    }


    /// Get the width of the buffer image content.
    pub fn get_width(&self) -> u32 {
        SCREEN_W
    }

    /// Get the height of the buffer image content.
    pub fn get_height(&self) -> u32 {
        SCREEN_H
    }

    /// Get the value of a specific pixel.
    pub fn get_pixel(&self, x: u32, y: u32) -> &Color {
        let index = x + (y * SCREEN_W);
        &self.pixels.get()[index as usize]
    }

    /// Set the value of a specific pixel.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let index = x + (y * SCREEN_W);
        self.pixels.get_mut()[index as usize] = color;
    }

    /// Fill the whole screen with a single solid color.
    pub fn fill(&mut self, color: Color) {
        for pixel in self.pixels.get_mut() {
            *pixel = color;
        }
    }

    /// Get the pixel data to be displayed.
    pub fn get_pixels(&self) -> &PixelBuffer160x144 {
        &self.pixels
    }

    /// Get the pixel data to be displayed as a slice of bytes.
    pub fn get_pixels_as_slice(&self) -> &[u8] {
        self.pixels.as_slice()
    }
}
