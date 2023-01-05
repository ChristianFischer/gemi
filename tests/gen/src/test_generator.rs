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

use std::path::PathBuf;
use gbemu_core::gameboy::DeviceType;
use tests_shared::test_config::{EmulatorTestConfig, LcdColorMod, RunConfig};
use crate::io_utils::{IteratorState, starts_with_number};


/// Configuration for generating tests.
pub struct TestGenerator {
    pub base_path_roms: PathBuf,
    pub base_path_tests: PathBuf,
    pub tests_known_to_fail: Vec<&'static str>,
}


impl TestGenerator {
    /// Creates a test using a given EmulatorTestConfig and
    /// optional a string containing rust code with additional checks.
    pub fn create_test(
            &self,
            name: &str,
            test_cfg: EmulatorTestConfig,
            additional_checks: Option<String>,
            state: &IteratorState
    ) -> String {
        let mut test_code = String::new();

        // the ROM file being tested, stripping the BASE_PATH_ROM_FILES
        let rom_file = test_cfg.setup.cartridge_path
            .replace('\\', "/")
            .replace(self.base_path_roms.to_str().unwrap(), "")
        ;

        // the module path to the current test
        let current_test_path = format!("{}/{}", state.path, name);

        test_code.push_str("#[test]\n");

        // tests known to fail will be ignored to avoid spamming
        // the test log with irrelevant errors
        if self.is_test_known_to_fail(&current_test_path) {
            test_code.push_str("#[ignore]\n");
        }

        // prefix the test case, if it starts with a non-symbol character
        let test_prefix = if starts_with_number(name) {
            if let Some(last) = state.stack.last() {
                format!("{last}_")
            }
            else {
                "test_".to_string()
            }
        }
        else {
            String::new()
        };

        // test function body
        test_code.push_str(&format!("fn {test_prefix}{name}() {{\n"));
        test_code.push_str("    let cfg = EmulatorTestConfig {\n");

        // SetUpConfig
        {
            test_code.push_str("        setup: SetUpConfig {\n");

            if let Some(device_type) = test_cfg.setup.device {
                let device_type_strval = match device_type {
                    DeviceType::GameBoyDmg     => "GameBoyDmg",
                    DeviceType::GameBoyColor   => "GameBoyColor",
                    DeviceType::GameBoyAdvance => "GameBoyAdvance",
                    DeviceType::SuperGameBoy   => "SuperGameBoy",
                    DeviceType::SuperGameBoy2  => "SuperGameBoy2",
                };

                test_code.push_str(&format!("            device: Some(DeviceType::{device_type_strval}),\n"));
            }

            if let Some(palette) = test_cfg.setup.dmg_display_palette {
                let palette_colors = palette.get_colors();

                test_code.push_str(&format!("            dmg_display_palette: Some(DmgDisplayPalette::new([\n"));
                test_code.push_str(&format!("                Color::from_rgba32(0x{:08x}),\n", palette_colors[0].to_u32()));
                test_code.push_str(&format!("                Color::from_rgba32(0x{:08x}),\n", palette_colors[1].to_u32()));
                test_code.push_str(&format!("                Color::from_rgba32(0x{:08x}),\n", palette_colors[2].to_u32()));
                test_code.push_str(&format!("                Color::from_rgba32(0x{:08x}),\n", palette_colors[3].to_u32()));
                test_code.push_str(&format!("            ])),\n"));
            }

            if test_cfg.setup.enable_serial_output != false {
                let enable_serial_output = test_cfg.setup.enable_serial_output;
                test_code.push_str(&format!("            enable_serial_output: {enable_serial_output},\n"));
            }

            test_code.push_str(&format!("            .. SetUpConfig::with_rom_file(\"{rom_file}\")\n"));
            test_code.push_str("        },\n");
        }

        // RunConfig
        {
            test_code.push_str("        run_config: RunConfig {\n");

            if let Some(run_frames) = test_cfg.run_config.run_frames {
                test_code.push_str(&format!("            run_frames: Some({run_frames}),\n"));
            }

            if test_cfg.run_config.stop_on_halt != RunConfig::default().stop_on_halt {
                let stop_on_halt = test_cfg.run_config.stop_on_halt;
                test_code.push_str(&format!("            stop_on_halt: {stop_on_halt},\n"));
            }

            if test_cfg.run_config.stop_on_infinite_loop != RunConfig::default().stop_on_infinite_loop {
                let stop_on_infinite_loop = test_cfg.run_config.stop_on_infinite_loop;
                test_code.push_str(&format!("            stop_on_infinite_loop: {stop_on_infinite_loop},\n"));
            }

            test_code.push_str("            .. RunConfig::default()\n");
            test_code.push_str("        },\n");
        }

        // CheckResultConfig
        {
            test_code.push_str("        result: CheckResultConfig {\n");

            if let Some(ref_image) = test_cfg.result.compare_lcd_with_image {
                let ref_image_path = ref_image
                    .replace('\\', "/")
                    .replace(self.base_path_roms.to_str().unwrap(), "")
                ;

                test_code.push_str(&format!("            compare_lcd_with_image: Some(\"{ref_image_path}\".to_string()),\n"));
            }

            match test_cfg.result.color_mod {
                LcdColorMod::None => {}
                LcdColorMod::Gambatte => test_code.push_str("            color_mod: LcdColorMod::Gambatte,\n"),
            }

            if let Some(gambatte_display_code) = test_cfg.result.gambatte_display_result_code {
                test_code.push_str(&format!("            gambatte_display_result_code: Some(\"{gambatte_display_code}\".to_string()),\n"));
            }

            test_code.push_str("            .. CheckResultConfig::default()\n");
            test_code.push_str("        },\n");
        }

        // footer
        test_code.push_str("    };\n");
        test_code.push_str("\n");

        if let Some(additional_checks) = additional_checks {
            test_code.push_str("    let mut gb = run_with_config(cfg);\n");
            test_code.push_str("\n");
            test_code.push_str(&additional_checks);
        }
        else {
            test_code.push_str("    run_with_config(cfg);\n");
        }

        test_code.push_str("}\n");

        test_code = Self::apply_indents_to(&test_code, state);

        test_code
    }


    /// Given a module path for any test, checks if this is known to fail
    /// and therefor should be ignored.
    pub fn is_test_known_to_fail(&self, path: &str) -> bool {
        for to_ignore_test in &self.tests_known_to_fail {
            if path.starts_with(to_ignore_test) {
                return true;
            }
        }

        false
    }


    /// Apply indentation to any given text.
    pub fn apply_indents_to(content: &str, state: &IteratorState) -> String {
        let empty_newline = "\n".to_string();

        content
            .split('\n')
            .into_iter()
            .map(|line| if !(line.is_empty()) {
                    format!("{}{}\n", state.indent, line)
                }
                else {
                    empty_newline.clone()
                }
            )
            .collect()
    }
}
