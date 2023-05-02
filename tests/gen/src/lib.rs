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

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tests_shared::config::{BASE_PATH_ROM_FILES, BASE_PATH_TESTS, SOURCE_URL};
use tests_shared::download::download_test_roms;
use tests_shared::test_suites::ALL_TEST_SUITES;
use crate::test_generator::UnitTestGenerator;

pub mod common;
pub mod test_generator;


/// Fetch all test roms from the internet and store them in the local file system.
pub fn fetch_test_roms() {
    let path = Path::new(BASE_PATH_ROM_FILES);

    if !path.is_dir() {
        download_test_roms(path, &SOURCE_URL);
    }
}


/// Generate all unit tests for the emulator.
pub fn generate_all_tests() {
    let base_path_roms  = PathBuf::from(BASE_PATH_ROM_FILES);
    let base_path_tests = PathBuf::from(BASE_PATH_TESTS);

    for test_suite in ALL_TEST_SUITES {
        let generator = Rc::new(RefCell::new(UnitTestGenerator::new()));
        test_suite.start(&base_path_roms, generator.clone());

        let tests_file = base_path_tests.join(format!("{}.rs", test_suite.name));
        generator.borrow().to_file(&tests_file);
    }
}

