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


/// Represents a mock implementation of [ImageData] with no data.
///
/// This image can be used to build an emulator client and run tests on it,
/// but likely not suitable for actual emulation due to its lack of data.
pub struct ZeroImageData {
    data: [u8; 0],
}


impl ZeroImageData {
    pub fn new() -> Self {
        Self {
            data: [0xff; 0],
        }
    }
}


impl Default for ZeroImageData {
    fn default() -> Self {
        Self::new()
    }
}


impl ImageData for ZeroImageData {
    fn get_size(&self) -> usize {
        0
    }


    fn get_data(&self) -> &[u8] {
        &self.data
    }
}


impl ImageDataMut for ZeroImageData {
    fn get_data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
