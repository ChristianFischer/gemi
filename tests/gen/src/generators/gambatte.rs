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

use std::path::PathBuf;
use gbemu_core::gameboy::DeviceType;
use gbemu_core::graphic_data::{Color, DmgDisplayPalette};
use tests_shared::test_config::{CheckResultConfig, EmulatorTestConfig, LcdColorMod, RunConfig, SetUpConfig};
use crate::generators::common::TEST_FILE_HEADER;
use crate::io_utils::{filename_to_symbol, FindRomCallbacks, get_plain_filename, HandleDirectory, recursive_visit_directory, update_file};
use crate::rom_utils::file_is_rom;
use crate::test_generator::TestGenerator;


const GAMBATTE_ADDITIONAL_SRC : &str = /* language=rust */
r#"use gbemu_core::graphic_data::{Color, DmgDisplayPalette};
"#;


/// Lookup table to check for device types within gambatte test rom file names.
const GAMBATTE_DEVICE_CODE_TABLE : &[(&str, DeviceType)] = &[
    ("dmg08",  DeviceType::GameBoyDmg),
    ("xdmg08", DeviceType::GameBoyDmg),
    ("cgb04c", DeviceType::GameBoyColor),
    ("xcgb",   DeviceType::GameBoyColor),
];


/// A test entry containing a test config and it's name.
struct GambatteEmuTestConfig {
    name: String,
    config: EmulatorTestConfig,
}


/// Creates a test config for a particular file with any given settings.
fn make_config_for_file(
    file: &PathBuf,
    name: &str,
    device_type: DeviceType,
    checks: &CheckResultConfig
) -> GambatteEmuTestConfig {
    let mut test_name = name.to_string().to_lowercase();

    // add test code to test name, because there are some similar test configs
    // with just a different result code
    if let Some(display_code) = &checks.gambatte_display_result_code {
        test_name = format!("{}_out{}", test_name, display_code.to_lowercase());
    }

    GambatteEmuTestConfig {
        name: format!("{}_{}", test_name, get_device_type_short_name(&device_type)),
        config: EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(device_type),
                .. SetUpConfig::with_rom_file(&file.to_str().unwrap())
            },
            run_config: RunConfig {
                run_frames: Some(16),
                .. RunConfig::default()
            },
            result: checks.clone(),
        }
    }
}


/// Finds test configurations for a gambatte test rom.
fn find_gambatte_checks(file: &PathBuf) -> Vec<GambatteEmuTestConfig> {
    if let Some(_) = file_is_rom(file) {
        let mut configs : Vec<GambatteEmuTestConfig> = Vec::new();
        let filename = get_plain_filename(file);

        // check for comparison images
        {
            let path = file.parent().unwrap();

            for entry in GAMBATTE_DEVICE_CODE_TABLE {
                let ref_image_name = format!("{}_{}.png", filename, entry.0);
                let ref_image_path = path.join(ref_image_name);
                let device_type    = entry.1;

                // look for any comparison image matching the current device type
                if ref_image_path.exists() {
                    let mut config = make_config_for_file(
                        file,
                        &filename,
                        device_type,
                        &Default::default()
                    );

                    config.config.result.compare_lcd_with_image = Some(
                        ref_image_path.to_str().unwrap().to_string().replace('\\', "/")
                    );

                    match device_type {
                        // for DMG set the expected color palette
                        DeviceType::GameBoyDmg => {
                            config.config.setup.dmg_display_palette = Some(DmgDisplayPalette::new([
                                Color::from_rgba32(0x000000ff),
                                Color::from_rgba32(0x555555ff),
                                Color::from_rgba32(0xaaaaaaff),
                                Color::from_rgba32(0xffffffff),
                            ]));
                        }

                        // for GameBoyColor define how colors are expected in the comparison image
                        DeviceType::GameBoyColor => {
                            config.config.result.color_mod = LcdColorMod::Gambatte;
                        }

                        _ => { }
                    }

                    configs.push(config);
                }
            }

            if !configs.is_empty() {
                return configs;
            }
        }

        // check for test config in the file name
        {
            let mut filename_elements: Vec<&str> = filename.split("_").collect();
            let mut test_name = String::new();

            // skip elements until we find a device name
            while !filename_elements.is_empty() {
                if let Some(_) = filepart_is_device(filename_elements[0]) {
                    break;
                }

                // from the skipped file parts build up the test name
                if !test_name.is_empty() {
                    test_name.push('_');
                }

                test_name.push_str(filename_elements[0]);

                filename_elements.remove(0);
            }

            // test name should never be empty
            if test_name.is_empty() {
                test_name = filename.clone();
            }

            // ensure the test name won't include special characters
            test_name = filename_to_symbol(&test_name);

            // parse the file name
            while !filename_elements.is_empty() {
                let mut devices : Vec<DeviceType> = Vec::new();
                let mut checks = CheckResultConfig::default();

                // collect all device types found at the current position in the file name
                while let Some(device_type) = filepart_is_device(filename_elements[0]) {
                    devices.push(device_type);
                    filename_elements.remove(0);

                    if filename_elements.is_empty() {
                        break;
                    }
                }

                // when no devices were found, stop here
                if devices.is_empty() {
                    break;
                }

                // check for known checks
                while !filename_elements.is_empty() {
                    if let Some(code) = filepart_is_display_check_code(filename_elements[0]) {
                        checks.gambatte_display_result_code = Some(code);
                        filename_elements.remove(0);
                        continue;
                    }

                    break;
                }

                // when found any checks, create a test config for each device type pending
                if checks.has_any_checks() {
                    for device_type in devices {
                        let test_config = make_config_for_file(
                            file,
                            &test_name,
                            device_type,
                            &checks
                        );

                        configs.push(test_config);
                    }
                }
            }

            if !configs.is_empty() {
                return configs;
            }
        }
    }

    vec![]
}


/// Checks whether the given part of a file name matches a device type.
fn filepart_is_device(part: &str) -> Option<DeviceType> {
    for entry in GAMBATTE_DEVICE_CODE_TABLE {
        if part == entry.0 {
            return Some(entry.1);
        }
    }

    None
}


/// Checks whether the given part of a file name matches a display code.
/// This has the expected format of 'outXX' where XX is a 1-2 digit code
/// expected to be displayed on the emulator screen.
fn filepart_is_display_check_code(part: &str) -> Option<String> {
    if part.starts_with("out") && part.len() >= 4 && part.len() <= 5 {
        return Some(part[3 ..].to_string());
    }

    None
}


/// Creates a short name of any device type.
fn get_device_type_short_name(device_type: &DeviceType) -> &str {
    match device_type {
        DeviceType::GameBoyDmg      => "dmg",
        DeviceType::GameBoyColor    => "cgb",
        DeviceType::GameBoyAdvance  => "agb",
        DeviceType::SuperGameBoy    => "sgb",
        DeviceType::SuperGameBoy2   => "sgb2",
    }
}



/// Create tests for Gambatte test roms.
pub fn generate_tests_gambatte(gen: &TestGenerator) {
    let gambatte_root      = gen.base_path_roms.join("gambatte");
    let gambatte_test_file = gen.base_path_tests.join("gambatte.rs");

    let tests_content = recursive_visit_directory(
        gambatte_root,
        &FindRomCallbacks {
            // open module for new directories
            on_handle_dir: Box::new(|_, _| {
                HandleDirectory::CreateModule
            }),

            // create tests for ROM files
            on_file_found: Box::new(|f, state| {
                let mut content = String::new();

                for test_cfg in find_gambatte_checks(f) {
                    let test_content = gen.create_test(
                        &test_cfg.name,
                        test_cfg.config,
                        None,
                        state
                    );

                    content.push_str(&test_content);
                }

                content
            }),
        }
    );

    let mut content = String::new();
    content.push_str(TEST_FILE_HEADER);
    content.push_str(GAMBATTE_ADDITIONAL_SRC);
    content.push_str(&tests_content);

    update_file(&gambatte_test_file, &content);
}
