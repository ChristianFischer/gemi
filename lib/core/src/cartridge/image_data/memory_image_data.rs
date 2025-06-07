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

#[cfg(feature = "dyn_alloc")]
pub use dyn_memory_image_data::*;


pub struct RefImageData<'a> {
    data: &'a [u8],
}


pub struct MutableRefImageData<'a> {
    data: &'a mut [u8],
}


impl<'a> RefImageData<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}


impl<'a> MutableRefImageData<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }
}


impl<'a> ImageData for RefImageData<'a> {
    fn get_size(&self) -> usize {
        self.data.len()
    }

    fn get_data(&self) -> &[u8] {
        self.data
    }
}


impl<'a> ImageData for MutableRefImageData<'a> {
    fn get_size(&self) -> usize {
        self.data.len()
    }

    fn get_data(&self) -> &[u8] {
        self.data
    }
}


impl<'a> ImageDataMut for MutableRefImageData<'a> {
    fn get_data_mut(&mut self) -> &mut [u8] {
        self.data
    }
}



#[cfg(feature = "dyn_alloc")]
pub mod dyn_memory_image_data {
    use crate::cartridge::image_data::{ImageData, ImageDataMut};

    // todo: doc
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct MemoryImageData {
        data: Vec<u8>,
    }


    impl MemoryImageData {
        pub fn new(data: impl Into<Vec<u8>>) -> Self {
            Self {
                data: data.into(),
            }
        }
        
        
        pub fn alloc(size: usize) -> Self {
            let mut data = Vec::with_capacity(size);
            data.resize(size, 0);
            
            Self {
                data
            }
        }
    }


    impl ImageData for MemoryImageData {
        fn get_size(&self) -> usize {
            self.data.len()
        }

        fn get_data(&self) -> &[u8] {
            self.data.as_slice()
        }
    }


    impl ImageDataMut for MemoryImageData {
        fn get_data_mut(&mut self) -> &mut [u8] {
            self.data.as_mut_slice()
        }
    }
}
