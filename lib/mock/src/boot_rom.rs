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

use gemi_core::cartridge::image_data::FixedSizeImageData;


/// A mock representation of a Boot ROM Image.
///
/// This mock image contains a 256-byte image of zeroed data.
/// Running this Boot ROM will have no effect as each zero byte will be decoded
/// as a NOP instruction.
pub struct MockBootRom {
    data: [u8; 256],
}


impl MockBootRom {
    pub fn new() -> Self {
        MockBootRom {
            data: [0x00; 256],
        }
    }
}


impl FixedSizeImageData<256> for MockBootRom {
    fn get_data(&self) -> &[u8; 256] {
        &self.data
    }
}

