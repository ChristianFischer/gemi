/*
 * Copyright (C) 2022 by Christian Fischer
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

use std::io::Cursor;
use std::path::{Path, PathBuf};

const BASE_PATH:    &str = "res/test_roms/";
const SOURCE_URL:   &str = "https://github.com/c-sp/gameboy-test-roms/releases/download/v4.0/gameboy-test-roms-v4.0.zip";


/// Download and unzip the archive containing test ROMs
fn download_test_roms() {
    let path = PathBuf::from(BASE_PATH);

    // download archive
    let archive = reqwest::blocking::get(SOURCE_URL)
        .unwrap()
        .bytes()
        .unwrap()
        .to_vec()
    ;

    // unzip into target dir
    zip_extract::extract(
        Cursor::new(archive),
        &path,
        false
    ).unwrap();
}


fn main() {
    let path = Path::new(BASE_PATH);

    if !path.is_dir() {
        download_test_roms();
    }
}
