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

use tests_shared::test_config::{CheckResultConfig, EmulatorTestConfig, RunConfig, SetUpConfig};
use crate::generators::common::TEST_FILE_HEADER;
use crate::io_utils::{filename_to_symbol, FindRomCallbacks, HandleDirectory, recursive_visit_directory, update_file};
use crate::rom_utils::file_is_rom;
use crate::test_generator::TestGenerator;

/// Create tests for Blargg's test roms.
pub fn generate_tests_blargg(gen: &TestGenerator) {
    let blargg_root      = gen.base_path_roms.join("blargg");
    let blargg_test_file = gen.base_path_tests.join("blargg.rs");

    // extra code to check, if the last line sent to the serial port is either 'Passed' or 'Passed all tests'
    let blargg_checks = /*language=rust*/ "    let output = gb.serial.take_output_as_text();
    match output.trim().split('\\n').into_iter().last() {
        Some(\"Passed all tests\") => { }
        Some(\"Passed\") => { }
        _ => { panic!(\"Unexpected output:\\n{}\", output); }
    }\n";

    let tests_content = recursive_visit_directory(
        blargg_root,
        &FindRomCallbacks {
            // open module for new directories
            on_handle_dir: Box::new(|_, _| {
                HandleDirectory::CreateModule
            }),

            // create tests for ROM files
            on_file_found: Box::new(|f, state| {
                match file_is_rom(f) {
                    Some(device_type) => {
                        let cfg = EmulatorTestConfig {
                            setup: SetUpConfig {
                                device: Some(device_type),
                                enable_serial_output: true,
                                .. SetUpConfig::with_rom_file(&f.to_str().unwrap())
                            },
                            run_config: RunConfig {
                                stop_on_infinite_loop: true,
                                .. RunConfig::default()
                            },
                            result: CheckResultConfig::default(),
                        };

                        gen.create_test(
                            &filename_to_symbol(f.to_str().unwrap()),
                            cfg,
                            Some(blargg_checks.to_string()),
                            state
                        )
                    }

                    _ => "".to_string()
                }
            }),
        }
    );

    let mut content = String::new();
    content.push_str(TEST_FILE_HEADER);
    content.push_str(&tests_content);

    update_file(&blargg_test_file, &content);
}
