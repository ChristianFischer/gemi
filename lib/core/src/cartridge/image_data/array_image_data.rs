/*
 * Copyright (C) 2025 by Christian Fischer
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

use crate::cartridge::image_data::{ImageData, ImageDataMut};
use crate::utils::SerializableArray;


// todo: doc
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayImageData<const N: usize> {
    data: SerializableArray<u8, N>,
}


impl<const N: usize> ArrayImageData<N> {
    pub fn new(data: [u8; N]) -> Self {
        Self {
            data: data.into(),
        }
    }


    pub fn alloc() -> Self {
        Self {
            data: [0x00; N].into(),
        }
    }
}


impl<const N: usize> ImageData for ArrayImageData<N> {
    fn get_size(&self) -> usize {
        self.data.len()
    }

    fn get_data(&self) -> &[u8] {
        self.data.as_slice()
    }
}


impl<const N: usize> ImageDataMut for ArrayImageData<N> {
    fn get_data_mut(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }
}
