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

use crate::utils::ioerr;
use core::ops::Range;


/// Trait representing generic image data.
/// This trait allows reading bytes from any sized image data.
pub trait ImageData {
    /// Get the size of the underlying image data.
    fn get_size(&self) -> usize;

    /// Get a reference to the underlying image data.
    fn get_data(&self) -> &[u8];


    /// Read a single byte from the image data.
    /// If the requested offset is not within the range of the data,
    /// this function will panic.
    fn read(&self, offset: usize) -> u8 {
        self.get_data()[offset]
    }


    /// Get a data slice out of the ROM data.
    /// If the data is not large enough, it will return 'None'
    fn get_slice(&self, range: Range<usize>) -> Option<&[u8]> {
        if self.get_size() >= range.end {
            let data = self.get_data();
            Some(&data[range])
        }
        else {
            None
        }
    }
}


/// Trait for write access on image data represented by an [ImageData] trait object.
pub trait ImageDataMut : ImageData {
    /// Get a mutable reference to the underlying image data.
    fn get_data_mut(&mut self) -> &mut [u8];


    /// Writes a single byte to the image data.
    /// If the requested offset is not within the range of the data,
    /// this function will panic.
    fn write(&mut self, offset: usize, value: u8) {
        self.get_data_mut()[offset] = value;
    }


    /// Tells the implementation to store any pending changes to the underlying image data.
    /// For example, this could involve writing to a file or committing changes to a database.
    /// Not all implementations support this operation. If not supported by an implementation,
    /// it is supposed to return an error with the [ioerr::ErrorCode::NotSupported] code.
    fn flush(&self) -> ioerr::Result<bool> {
        Err(ioerr::Error {
            error_code:     ioerr::ErrorCode::NotSupported,
            source:         None,
            #[cfg(feature = "file_io")]
            source_file:    None,
        })
    }
}


/// A specialized variant of the [ImageData] trait for fixed-sized image data.
/// An [ImageData] implementing this trait has a compile time guarantee to be of the given size.
pub trait FixedSizeImageData<const SIZE: usize> {
    /// Constant representing the size of this [ImageData].
    const SIZE: usize = SIZE;

    /// Get a reference to the underlying image data.
    fn get_data(&self) -> &[u8; SIZE];
}
