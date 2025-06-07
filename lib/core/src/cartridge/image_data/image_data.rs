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

use crate::utils::ioerr;
use crate::utils::ioerr::ErrorCode;
use std::ops::Range;


// todo: doc
pub trait ImageData {
    fn get_size(&self) -> usize;
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


pub trait ImageDataMut : ImageData {
    fn get_data_mut(&mut self) -> &mut [u8];


    /// Writes a single byte to the image data.
    /// If the requested offset is not within the range of the data,
    /// this function will panic.
    fn write(&mut self, offset: usize, value: u8) {
        self.get_data_mut()[offset] = value;
    }


    // todo: doc
    fn flush(&self) -> ioerr::Result<bool> {
        Err(ioerr::Error {
            error_code:     ErrorCode::NotSupported,
            source:         None,
            #[cfg(feature = "file_io")]
            source_file:    None,
        })
    }
}
