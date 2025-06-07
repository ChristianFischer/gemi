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
use crate::utils::{ioerr, SerializableBuffer};
#[cfg(feature = "file_io")]
use std::{
    fs::File,
    io,
    io::{Read, Write},
    path::{Path, PathBuf},
};


// todo: naming? move to support lib?
// todo: doc
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataBuffer {
    /// The data associated with this data buffer.
    data: SerializableBuffer<u8>,

    /// The path to the file, this data buffer was loaded from.
    #[cfg(feature = "file_io")]
    file_path: Option<PathBuf>,
}


impl DataBuffer {
    /// Creates a data buffer with `size` bytes.
    pub fn alloc(size: usize) -> Self {
        Self {
            data: vec![0; size].into(),
            #[cfg(feature = "file_io")]
            file_path: None,
        }
    }


    /// Creates a new data buffer with given data.
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self {
            data: data.into().into(),
            #[cfg(feature = "file_io")]
            file_path: None,
        }
    }


    /// Creates a new, empty data buffer.
    pub fn new_empty() -> Self {
        Self::alloc(0)
    }
}


#[cfg(feature = "file_io")]
impl DataBuffer {
    /// Loads a data buffer into a file.
    pub fn load_from_file(file_path: &Path) -> io::Result<Self> {
        let mut file   = File::open(file_path)?;
        let metadata   = file.metadata()?;
        let mut buffer = vec![0; metadata.len() as usize];

        file.read_exact(&mut buffer)?;

        Ok(Self {
            data:      buffer.into(),
            file_path: Some(file_path.to_path_buf()),
        })
    }


    /// Writes the content of this data buffer into a file.
    pub fn write_to_file(&self, file_path: &Path) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        file.write_all(&self.data)?;

        Ok(())
    }


    /// Get the path to the file, this data buffer was loaded from.
    pub fn get_file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }


    /// Set the file path of this data buffer.
    /// Future calls to [flush] will write into the new path.
    pub fn set_file_path(&mut self, file_path: impl Into<PathBuf>) {
        self.file_path = Some(file_path.into());
    }
}


impl ImageData for DataBuffer {
    fn get_size(&self) -> usize {
        self.data.len()
    }

    fn get_data(&self) -> &[u8] {
        self.data.as_slice()
    }
}


impl ImageDataMut for DataBuffer {
    fn get_data_mut(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    fn flush(&self) -> ioerr::Result<bool> {
        if let Some(file_path) = &self.file_path {
            self
                    .write_to_file(file_path)
                    .map_err(|_| ioerr::Error {
                        error_code:  ioerr::ErrorCode::FailedToWriteFile,
                        source:      None,
                        source_file: Some(file_path.clone()),
                    })
                    ?;

            Ok(true)
        }
        else {
            Ok(false)
        }
    }
}
