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

use std::io;
use std::fs::File;
use std::io::{Read, Write};

pub use dynamic_size::*;
pub use fixed_size::*;


/// This object represents any kind of addressable memory storage like ROM or RAM data.
pub trait MemoryData {
    /// Get the total size of the memory data block.
    fn size(&self) -> usize;

    /// Get the byte on a specific memory location.
    fn get_at(&self, address: usize) -> u8;

    /// Set the byte on a specific memory location.
    fn set_at(&mut self, address: usize, value: u8);

    /// Get the memory slice of the data below.
    fn as_slice(&self) -> &[u8];

    /// Get the mutable memory slice of the data below.
    fn as_slice_mut(&mut self) -> &mut [u8];

    /// Save the RAM image into a file.
    fn save_to_file(&self, filepath: &str) -> io::Result<()> {
        let mut file = File::create(filepath)?;
        file.write(self.as_slice())?;

        Ok(())
    }

    /// Load the RAM image from a file.
    fn read_from_file(&mut self, filepath: &str) -> io::Result<()> {
        let mut file = File::open(filepath)?;
        file.read(self.as_slice_mut())?;

        Ok(())
    }
}


mod dynamic_size {
    use crate::mmu::memory_data::MemoryData;


    /// A data object storing data of variable size.
    pub struct MemoryDataDynamic {
        data: Vec<u8>,
    }


    impl MemoryDataDynamic {
        /// Allocates memory of a certain size.
        pub fn alloc(size: usize) -> Self {
            Self {
                data: vec![0xff; size]
            }
        }
    }


    impl MemoryData for MemoryDataDynamic {
        fn size(&self) -> usize {
            self.data.len()
        }

        fn get_at(&self, address: usize) -> u8 {
            self.data[address]
        }

        fn set_at(&mut self, address: usize, value: u8) {
            self.data[address] = value;
        }

        fn as_slice(&self) -> &[u8] {
            self.data.as_slice()
        }

        fn as_slice_mut(&mut self) -> &mut [u8] {
            self.data.as_mut_slice()
        }
    }
}


pub mod fixed_size {
    use crate::mmu::memory_data::MemoryData;


    /// A data object storing data of fixed size.
    #[derive(Clone)]
    pub struct MemoryDataFixedSize<const SIZE: usize> {
        arr: Box<[u8; SIZE]>,
    }


    impl<const SIZE: usize> MemoryDataFixedSize<SIZE> {
        /// Allocates a new memory block.
        pub fn new() -> Self {
            Self {
                arr: Box::new([0x00; SIZE])
            }
        }
    }


    impl<const SIZE: usize> MemoryData for MemoryDataFixedSize<SIZE> {
        fn size(&self) -> usize {
            SIZE
        }

        fn get_at(&self, address: usize) -> u8 {
            self.arr[address]
        }

        fn set_at(&mut self, address: usize, value: u8) {
            self.arr[address] = value;
        }

        fn as_slice(&self) -> &[u8] {
            self.arr.as_slice()
        }

        fn as_slice_mut(&mut self) -> &mut [u8] {
            self.arr.as_mut_slice()
        }
    }
}


pub mod mapped {
    use std::borrow::{Borrow, BorrowMut};
    use std::mem::size_of;
    use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
    use crate::mmu::memory_data::MemoryData;


    /// A memory data object storing data being represented by another data type like a struct.
    pub struct MemoryDataMapped<T> {
        data: Box<T>,
    }


    impl<T> MemoryDataMapped<T> {
        /// Creates a new data type with a given content object.
        pub fn new(data: T) -> Self {
            Self {
                data: Box::new(data)
            }
        }

        /// Get the inner data object.
        pub fn get(&self) -> &T {
            &self.data
        }

        /// Get the inner data object.
        pub fn get_mut(&mut self) -> &mut T {
            &mut self.data
        }
    }


    impl<T> MemoryData for MemoryDataMapped<T> {
        fn size(&self) -> usize {
            size_of::<T>()
        }

        fn get_at(&self, address: usize) -> u8 {
            self.as_slice()[address]
        }

        fn set_at(&mut self, address: usize, value: u8) {
            self.as_slice_mut()[address] = value;
        }

        fn as_slice(&self) -> &[u8] {
            let ptr = self.data.borrow() as *const T as *const u8;

            unsafe {
                &*slice_from_raw_parts(ptr, self.size())
            }
        }

        fn as_slice_mut(&mut self) -> &mut [u8] {
            let ptr = self.data.borrow_mut() as *mut T as *mut u8;

            unsafe {
                &mut *slice_from_raw_parts_mut(ptr, self.size())
            }
        }
    }
}
