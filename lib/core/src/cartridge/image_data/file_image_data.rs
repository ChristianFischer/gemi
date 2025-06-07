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

#[cfg(feature = "file_io")]
pub use file_image_data_impl::*;


#[cfg(feature = "file_io")]
pub mod file_image_data_impl {
    use crate::cartridge::image_data::{ImageData, ImageDataMut};
    use crate::utils::ioerr;
    use std::fs::File;
    use std::io;
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};


    // todo: doc
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct FileImageData {
        file_path: PathBuf,
        data: Vec<u8>,
    }


    impl FileImageData {
        pub fn load(file_path: &Path) -> io::Result<Self> {
            let mut file   = File::open(file_path)?;
            let metadata   = file.metadata()?;
            let mut buffer = vec![0; metadata.len() as usize];

            file.read_exact(&mut buffer)?;

            Ok(Self {
                file_path: file_path.to_path_buf(),
                data: buffer,
            })
        }


        pub fn get_file_path(&self) -> &Path {
            &self.file_path
        }
    }


    impl ImageData for FileImageData {
        fn get_size(&self) -> usize {
            self.data.len()
        }

        fn get_data(&self) -> &[u8] {
            self.data.as_slice()
        }
    }


    impl ImageDataMut for FileImageData {
        fn get_data_mut(&mut self) -> &mut [u8] {
            self.data.as_mut_slice()
        }


        fn flush(&self) -> crate::utils::ioerr::Result<bool> {
            || -> io::Result<()> {
                let mut file = File::create(&self.file_path)?;
                file.write_all(self.get_data())?;

                Ok(())
            }()
            .map_err(|_| ioerr::Error {
                error_code: ioerr::ErrorCode::FailedToWriteFile,
                source: None,
                source_file: Some(self.file_path.clone()),
            })?;
            
            Ok(true)
        }
    }
}
