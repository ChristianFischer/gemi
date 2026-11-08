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

use std::fs;
use std::io::Cursor;
use std::path::Path;
use zip::ZipArchive;


/// Download and unzip the archive containing test ROMs (blocking).
pub fn download_test_roms(path: &Path, url: &str) {
    let archive = reqwest::blocking::get(url).unwrap().bytes().unwrap().to_vec();
    extract_roms(path, &archive);
}


/// Download and unzip the archive containing test ROMs (async).
pub async fn download_test_roms_async(path: &Path, url: &str) {
    let archive = reqwest::get(url).await.unwrap().bytes().await.unwrap().to_vec();
    extract_roms(path, &archive);
}


/// Extract a ZIP archive (given as bytes) into the target directory.
fn extract_roms(path: &Path, archive: &[u8]) {
    fs::create_dir_all(path).unwrap();

    let mut zip = ZipArchive::new(Cursor::new(archive)).unwrap();
    zip.extract(path).unwrap();
}
