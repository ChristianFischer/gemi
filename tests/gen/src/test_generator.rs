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

use std::collections::HashSet;
use std::path::PathBuf;
use tests_shared::config::{BASE_PATH_ROM_FILES, TESTS_KNOWN_TO_FAIL};
use tests_shared::io_utils::{IteratorState, starts_with_number, TestConfigVisitor, update_file};
use tests_shared::test_config::{EmulatorTestCase, EmulatorTestConfig, LcdColorMod, RunConfig};
use crate::common::TEST_FILE_HEADER;


/// An object used to generate unit tests for the emulator based on the results of the file visitors.
pub struct UnitTestGenerator {
    content: String,
    additional_imports: HashSet<String>,
}


impl UnitTestGenerator {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            additional_imports: HashSet::new(),
        }
    }


    /// Creates a set of tests using a given [EmulatorTestConfig] and
    /// optional a string containing rust code with additional checks.
    /// For each device configured in the [EmulatorTestConfig], a separate
    /// [EmulatorTestCase] will be created.
    pub fn create_tests(
        &mut self,
        test_cfg: &EmulatorTestConfig,
        state: &IteratorState
    ) -> String {
        test_cfg
            // get all test cases
            .get_test_cases()

            // create the source code of each test case
            .map(|test_case|
                self.create_test(
                    test_cfg.name.as_str(),
                    &test_case,
                    state
                )
            )

            // concatenate all generated source code elements
            .collect::<Vec<String>>()
            .join("\n\n")
    }


    /// Creates a test using data from a given [EmulatorTestCase] and
    /// optional a string containing rust code with additional checks.
    pub fn create_test(
            &mut self,
            name: &str,
            test_case: &EmulatorTestCase,
            state: &IteratorState
    ) -> String {
        let mut test_code = String::new();

        // the ROM file being tested, stripping the BASE_PATH_ROM_FILES
        let rom_file = test_case.setup.cartridge_path
            .replace('\\', "/")
            .replace(BASE_PATH_ROM_FILES, "")
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

        // get a device code suffix for the test name
        let device_code = test_case.device.get_abbreviation();

        // test function body
        test_code.push_str(&format!("fn {test_prefix}{name}_{device_code}() {{\n"));
        test_code.push_str("    let test_case = EmulatorTestCase {\n");

        // device type
        {
            let device_type_strval = test_case.device.to_string();
            test_code.push_str(&format!("        device: DeviceType::{device_type_strval},\n"));
        }

        // SetUpConfig
        {
            test_code.push_str("        setup: SetUpConfig {\n");

            if let Some(palette) = test_case.setup.dmg_display_palette {
                let palette_colors = palette.get_colors();

                test_code.push_str(&format!("            dmg_display_palette: Some(DmgDisplayPalette::new([\n"));
                test_code.push_str(&format!("                Color::from_rgba32(0x{:08x}),\n", palette_colors[0].to_u32()));
                test_code.push_str(&format!("                Color::from_rgba32(0x{:08x}),\n", palette_colors[1].to_u32()));
                test_code.push_str(&format!("                Color::from_rgba32(0x{:08x}),\n", palette_colors[2].to_u32()));
                test_code.push_str(&format!("                Color::from_rgba32(0x{:08x}),\n", palette_colors[3].to_u32()));
                test_code.push_str(&format!("            ])),\n"));

                // requires import for Color and palette
                self.additional_imports.insert("use crate::video::palette::{Color.DmgDisplayPalette};".to_string());
            }

            test_code.push_str(&format!("            .. SetUpConfig::with_rom_file(\"{rom_file}\")\n"));
            test_code.push_str("        },\n");
        }

        // RunConfig
        {
            test_code.push_str("        run_config: RunConfig {\n");

            if let Some(run_frames) = test_case.run_config.run_frames {
                test_code.push_str(&format!("            run_frames: Some({run_frames}),\n"));
            }

            if test_case.run_config.stop_on_halt != RunConfig::default().stop_on_halt {
                let stop_on_halt = test_case.run_config.stop_on_halt;
                test_code.push_str(&format!("            stop_on_halt: {stop_on_halt},\n"));
            }

            if test_case.run_config.stop_on_infinite_loop != RunConfig::default().stop_on_infinite_loop {
                let stop_on_infinite_loop = test_case.run_config.stop_on_infinite_loop;
                test_code.push_str(&format!("            stop_on_infinite_loop: {stop_on_infinite_loop},\n"));
            }

            test_code.push_str("            .. RunConfig::default()\n");
            test_code.push_str("        },\n");
        }

        // CheckResultConfig
        {
            test_code.push_str("        result: CheckResultConfig {\n");

            if let Some(ref_image) = &test_case.result.compare_lcd_with_image {
                let ref_image_path = ref_image
                    .replace('\\', "/")
                    .replace(BASE_PATH_ROM_FILES, "")
                ;

                test_code.push_str(&format!("            compare_lcd_with_image: Some(\"{ref_image_path}\".to_string()),\n"));
            }

            match test_case.result.color_mod {
                LcdColorMod::None => {}
                LcdColorMod::Gambatte => test_code.push_str("            color_mod: LcdColorMod::Gambatte,\n"),
            }

            if let Some(gambatte_display_code) = &test_case.result.gambatte_display_result_code {
                test_code.push_str(&format!("            gambatte_display_result_code: Some(\"{gambatte_display_code}\".to_string()),\n"));
            }

            if test_case.result.blargg_check_result_code {
                test_code.push_str(&format!("            blargg_check_result_code: true,\n"));
            }

            if test_case.result.mooneye_check_result_code {
                test_code.push_str(&format!("            mooneye_check_result_code: true,\n"));
            }

            test_code.push_str("            .. CheckResultConfig::default()\n");
            test_code.push_str("        },\n");
        }

        // footer
        test_code.push_str("    };\n");
        test_code.push_str("\n");
        test_code.push_str("    run_test_case(test_case);\n");

        test_code.push_str("}\n");

        test_code = Self::apply_indents_to(&test_code, state);

        test_code
    }


    /// Given a module path for any test, checks if this is known to fail
    /// and therefor should be ignored.
    pub fn is_test_known_to_fail(&self, path: &str) -> bool {
        for to_ignore_test in TESTS_KNOWN_TO_FAIL {
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
            .trim_end()
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


    /// Updates the target file with the generated content.
    /// The file won't be updated if the content is the same.
    pub fn to_file(&self, path: &PathBuf) {
        let final_content = format!(
            "{}\n{}",
            TEST_FILE_HEADER,
            self.content
        );

        update_file(path, &final_content);
    }
}


impl TestConfigVisitor for UnitTestGenerator {
    fn on_open_module(&mut self, module_name: &str, state: &IteratorState) {
        self.content.push_str("\n\n");
        self.content.push_str(&format!("{}mod {} {{\n", state.indent, module_name));
        self.content.push_str(&format!("{}    use super::*;\n", state.indent))
    }


    fn on_close_module(&mut self, _module_name: &str, state: &IteratorState) {
        self.content.push_str(&format!("{}}}\n", state.indent));
    }


    fn on_visit_test(&mut self, test_cfg: &EmulatorTestConfig, state: &IteratorState) {
        // generate source code for each single test
        let tests_str = self.create_tests(test_cfg, state);

        if !tests_str.is_empty() {
            self.content.push_str("\n");
            self.content.push_str("\n");
            self.content.push_str(&tests_str);
        }
    }
}
