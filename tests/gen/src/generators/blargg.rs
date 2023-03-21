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

use gbemu_core::gameboy::DeviceType;
use tests_shared::test_config::{CheckResultConfig, EmulatorTestConfig, RunConfig, SetUpConfig};
use crate::generators::common::TEST_FILE_HEADER;
use crate::io_utils::{filename_to_symbol, FindRomCallbacks, HandleDirectory, recursive_visit_directory, update_file};
use crate::rom_utils::file_is_rom;
use crate::test_generator::TestGenerator;

const BLARGG_ADDITIONAL_SRC : &str = /* language=rust */
    r#"use testrunner::checks::blargg_checks::check_blargg_test_passed;
"#;

const BLARGG_CHECK_TEST : &str = /* language=rust */ r#"    let result = check_blargg_test_passed(&mut gb);
    assert!(result.is_ok(), "Failed test: '{}'", result.err().unwrap());
"#;


/// Create tests for Blargg's test roms.
pub fn generate_tests_blargg(gen: &TestGenerator) {
    let blargg_root      = gen.base_path_roms.join("blargg");
    let blargg_test_file = gen.base_path_tests.join("blargg.rs");
    let cgb_sound_folder = String::from("cgb_sound");

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
                    Some(file_device_type) => {
                        // check whether this file is in a subdir of cgb_sound
                        let is_gbc_subfolder = state.stack.contains(&cgb_sound_folder);

                        // when folder is gbc, this overrides the file type
                        let device_type = if is_gbc_subfolder {
                            DeviceType::GameBoyColor
                        }
                        else {
                            file_device_type
                        };

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
                            Some(BLARGG_CHECK_TEST.to_string()),
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
    content.push_str(BLARGG_ADDITIONAL_SRC);
    content.push_str(&tests_content);

    update_file(&blargg_test_file, &content);
}
