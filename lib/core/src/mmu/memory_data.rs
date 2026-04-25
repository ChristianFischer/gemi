/*
 * Copyright (C) 2022-2026 by Christian Fischer
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

pub use fixed_size::*;

#[cfg(feature = "dyn_alloc")]
pub use dynamic_size::*;


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
}


#[cfg(feature = "dyn_alloc")]
mod dynamic_size {
    use crate::mmu::memory_data::MemoryData;
    use crate::utils::ioerr;
    use alloc::{vec, vec::Vec};


    /// A data object storing data of variable size.
    #[derive(Clone)]
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


    impl TryFrom<Vec<u8>> for MemoryDataDynamic {
        type Error = ioerr::ErrorCode;

        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self {
                data: value
            })
        }
    }


    impl<const N: usize> TryFrom<[u8; N]> for MemoryDataDynamic {
        type Error = ioerr::ErrorCode;

        fn try_from(value: [u8; N]) -> Result<Self, Self::Error> {
            Ok(Self {
                data: value.into()
            })
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


    impl core::ops::Index<usize> for MemoryDataDynamic {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }


    impl core::ops::IndexMut<usize> for MemoryDataDynamic {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index]
        }
    }
}


pub mod fixed_size {
    use crate::mmu::memory_data::MemoryData;
    use crate::utils::{ioerr, SerializableArray};

    #[cfg(feature = "dyn_alloc")]
    use alloc::vec::Vec;


    /// A data object storing data of fixed size.
    #[derive(Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct MemoryDataFixedSize<const SIZE: usize> {
        arr: SerializableArray<u8, SIZE>,
    }


    impl<const SIZE: usize> MemoryDataFixedSize<SIZE> {
        /// Allocates a new memory block.
        pub fn new() -> Self {
            Self {
                arr: [0x00; SIZE].into()
            }
        }
    }


    impl<const SIZE: usize, const N: usize> TryFrom<[u8; N]> for MemoryDataFixedSize<SIZE>
    {
        type Error = ioerr::ErrorCode;

        fn try_from(value: [u8; N]) -> Result<Self, Self::Error> {
            TryFrom::<&[u8]>::try_from(&value)
        }
    }


    #[cfg(feature = "dyn_alloc")]
    impl<const SIZE: usize> TryFrom<Vec<u8>> for MemoryDataFixedSize<SIZE> {
        type Error = ioerr::ErrorCode;

        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            TryFrom::<&[u8]>::try_from(&value)
        }
    }


    impl<const SIZE: usize> TryFrom<&[u8]> for MemoryDataFixedSize<SIZE> {
        type Error = ioerr::ErrorCode;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            let array: [u8; SIZE] = match TryInto::<[u8; SIZE]>::try_into(value) {
                Ok(array) => Ok(array),
                Err(_) => Err(ioerr::ErrorCode::InvalidFileSize(ioerr::InvalidFileSizeError {
                        actual: value.len(),
                        expected: SIZE,
                })),
            }?;

            Ok(Self {
                arr: array.into()
            })
        }
    }


    impl<const SIZE: usize> Into<[u8; SIZE]> for MemoryDataFixedSize<SIZE> {
        fn into(self) -> [u8; SIZE] {
            self.arr.into()
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


    impl<const SIZE: usize> core::ops::Index<usize> for MemoryDataFixedSize<SIZE> {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.arr[index]
        }
    }


    impl<const SIZE: usize> core::ops::IndexMut<usize> for MemoryDataFixedSize<SIZE> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.arr[index]
        }
    }
}


pub mod mapped {
    use core::borrow::{Borrow, BorrowMut};
    use core::mem::size_of;
    use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

    use crate::mmu::memory_data::MemoryData;


    /// A memory data object storing data being represented by another data type like a struct.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct MemoryDataMapped<T> {
        data: T,
    }


    impl<T> MemoryDataMapped<T> {
        /// Creates a new data type with a given content object.
        pub fn new(data: impl Into<T>) -> Self {
            Self {
                data: data.into()
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
