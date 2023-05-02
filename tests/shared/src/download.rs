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

use std::fs;
use std::io::Cursor;
use std::path::Path;

/// Download and unzip the archive containing test ROMs
pub fn download_test_roms(path: &Path, url: &str) {
    // download archive
    let archive = reqwest::blocking::get(url)
        .unwrap()
        .bytes()
        .unwrap()
        .to_vec()
    ;

    // ensure the target directory exists
    fs::create_dir_all(path).unwrap();

    // unzip into target dir
    zip_extract::extract(
        Cursor::new(archive),
        &path,
        false
    ).unwrap();
}
