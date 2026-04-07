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

use gemi_core::cartridge::image_data::{ImageData, ImageDataMut};


/// A data buffer holding 32k of data, which fits into two memory banks.
///
/// This can be used for creating mock instances of an emulator client.
pub struct MockImageData {
    data: [u8; 0x8000],
}


impl MockImageData {
    /// Creates a new instance with all bytes set to `0xff`.
    pub fn new() -> Self {
        MockImageData {
            data: [0xff; 0x8000],
        }
    }


    /// Creates a new instance with all bytes set to the given value.
    pub fn with_fill(fill: u8) -> Self {
        MockImageData {
            data: [fill; 0x8000],
        }
    }


    /// Creates a new instance with the given data.
    pub fn with_data(data: [u8; 0x8000]) -> Self {
        MockImageData {
            data,
        }
    }
}


impl ImageData for MockImageData {
    fn get_size(&self) -> usize {
        self.data.len()
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }
}


impl ImageDataMut for MockImageData {
    fn get_data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
