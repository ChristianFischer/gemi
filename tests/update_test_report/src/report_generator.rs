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

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use rpb::bar::Bar;
use rpb::styles::Themes;
use tokio::task::JoinHandle;
use gbemu_core::gameboy::DeviceType;
use tests_shared::io_utils::{IteratorState, TestConfigVisitor, update_file, Workspace};
use tests_shared::runner::{run_test_case_safe, TestCaseError};
use tests_shared::test_config::{EmulatorTestCase, EmulatorTestConfig};


/// A result code indicating whether a test was successful or not
/// with some additional information on the reason of failure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TestResult {
    Success,
    Failed,
    Error,
    Panic,
}


/// A struct containing the statistics of how much tests ended in each state.
#[derive(Default)]
pub struct TestStats {
    pub total:      u32,
    pub success:    u32,
    pub failed:     u32,
    pub errors:     u32,
    pub panicked:   u32,
}


/// A generic union type to store either one or the other type of value.
#[derive(Copy, Clone)]
pub enum Either<A, B> {
    First(A),
    Second(B),
}

type StringOrTestCase         = Either<String, EmulatorTestCase>;
type StringOrFutureTestResult = Either<String, JoinHandle<TestResult>>;


/// A set of test cases which belong to a single test ROM.
/// Used to stack test cases of the same ROM, which are considered as
/// different test configurations otherwise.
pub struct TestCaseSet {
    rom: String,
    test_cases: HashMap<DeviceType, EmulatorTestCase>,
}


/// An object used to generate unit tests for the emulator based on the results of the file visitors.
pub struct TestReportGenerator {
    /// The workspace where to find test ROMs.
    workspace: Workspace,

    /// The pending headline of the latest module found.
    /// Will only be submitted, when followed by any test case.
    pending_headline: Option<String>,

    /// The last test cases found. Stored until the current module gets closed
    /// or other test cases will be found which do not belong to the same ROM file.
    pending_tests: Option<TestCaseSet>,

    /// All entries found, which will be either a fixed string or a test case to be run.
    entries: Vec<StringOrTestCase>,
}


impl TestResult {
    /// Get an icon displaying the test result.
    pub fn get_icon(&self) -> &str {
        match self {
            TestResult::Success       => "✔️",
            TestResult::Failed        => "❌",
            TestResult::Error         => "⚠️",
            TestResult::Panic         => "☠️",

        }
    }
}



impl TestReportGenerator {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            workspace:          workspace.clone(),
            pending_headline:   None,
            pending_tests:      None,
            entries:            Vec::new(),
        }
    }


    /// Helper function to create a string containing the table header
    /// with the name of each device being tested.
    pub fn create_devices_header() -> String {
        let mut line1 = String::new();
        let mut line2 = String::new();

        line1.push_str("|                                                                          |");
        line2.push_str("|--------------------------------------------------------------------------|");

        for device in DeviceType::ALL_DEVICES {
            line1.push_str(&format!(" {:4} |", device.get_abbreviation()));
            line2.push_str(":----:|");
        }

        format!("{}\n{}", line1, line2)
    }


    /// Adds an string entry immediately.
    pub fn add_str_entry(&mut self, s: &str) {
        self.add_string_entry(s.to_string());
    }


    /// Adds an string entry immediately.
    pub fn add_string_entry(&mut self, s: String) {
        self.entries.push(StringOrTestCase::First(s));
    }


    /// Set the headline of the current module.
    /// The headline will be submitted once any tests were found.
    pub fn set_headline(&mut self, headline: String) {
        self.pending_headline = Some(headline);
    }


    /// Clear the currently pending headline, if any.
    pub fn clear_headline(&mut self) {
        self.pending_headline = None;
    }


    /// Submit the current headline, if any pending.
    /// The pending headline will be cleared after this.
    fn submit_headline_if_any(&mut self) {
        if let Some(headline) = self.pending_headline.take() {
            let headline_formatted = format!("**{}**", headline);
            let mut line = String::new();

            line.push_str(&format!("| {:72} |", headline_formatted));

            for _ in DeviceType::ALL_DEVICES {
                line.push_str("      |");
            }

            self.add_string_entry(format!("{}\n", line));

            self.clear_headline();
        }
    }


    /// Submits any pending set of test cases, if any.
    /// Clears the pending tests after submitting them.
    fn submit_tests(&mut self) {
        if let Some(pending_test_set) = std::mem::replace(&mut self.pending_tests, None) {
            let rom_file = Self::get_filename_from_path(&pending_test_set.rom);

            // any pending headline needs to be submitted first
            self.submit_headline_if_any();

            // add the string containing the ROM file name
            self.add_string_entry(format!("| {:72} |", rom_file));

            // check all possible device types
            for device_type in DeviceType::ALL_DEVICES {
                let entry = if let Some(test_case) = pending_test_set.test_cases.get(&device_type) {
                    // verify the test case contains the same rom file as our test set
                    assert_eq!(pending_test_set.rom, test_case.setup.cartridge_path);

                    StringOrTestCase::Second(test_case.clone())
                }
                else {
                    // produce an empty cell for each not covered device
                    let result_string = String::from("      |");
                    StringOrTestCase::First(result_string)
                };

                self.entries.push(entry);
            }

            self.add_str_entry("\n");
        }
    }


    /// Checks whether any new tests may be stacked with the currently pending tests,
    /// submits the pending tests if stacking is not possible
    fn submit_tests_if_not_stackable(&mut self, rom: &String, devices: &HashSet<DeviceType>) {
        if !self.can_stack_tests(rom, devices) {
            self.submit_tests();
        }
    }


    /// Checks whether any new tests may be stacked with the currently pending tests.
    fn can_stack_tests(&self, rom: &String, devices: &HashSet<DeviceType>) -> bool {
        if let Some(pending_test_set) = &self.pending_tests {
            // cannot stack with different rom files
            if !pending_test_set.rom.eq(rom) {
                return false;
            }

            // check if any device is already present in the pending set
            for device in devices {
                if pending_test_set.test_cases.contains_key(&device) {
                    // cannot stack if any device is already added
                    return false;
                }
            }

        }

        true
    }


    /// Runs a test, return with a test result.
    pub fn run_test(workspace: &Workspace, test_case: &EmulatorTestCase) -> TestResult {
        match run_test_case_safe(workspace, test_case) {
            Ok(_)  => TestResult::Success,
            Err(e) => match e {
                TestCaseError::Failed(_)     => TestResult::Failed,
                TestCaseError::SetUpError(_) => TestResult::Error,
                TestCaseError::Panic(_)      => TestResult::Panic,
            },
        }
    }


    /// Get the ROM file name out of a path reference.
    pub fn get_filename_from_path(path: &str) -> String {
        let path_normalized = path.replace("\\", "/");

        if let Some(idx) = path_normalized.rfind("/") {
            path_normalized[idx + 1..].to_string()
        }
        else {
            path_normalized.to_string()
        }
    }


    /// Runs all tests which were found in parallel, producing a string
    /// containing the results in a markdown table.
    #[tokio::main()]
    pub async fn collect_contents(&self) -> (String, TestStats) {
        // clone into Arc to make it accessible from multiple threads
        let workspace = Arc::new(self.workspace.clone());

        // track statistics
        let mut stats = TestStats::default();

        let entries = self.entries
            .clone().into_iter()
            .map(|e| match e {
                StringOrTestCase::First(s) => StringOrFutureTestResult::First(s),

                StringOrTestCase::Second(tc) => {
                    // create a clone of the Arc to be moved into the async closure
                    let workspace = workspace.clone();

                    StringOrFutureTestResult::Second(tokio::spawn(async move {
                        Self::run_test(&workspace, &tc)
                    }))
                },
            })
            .collect::<Vec<StringOrFutureTestResult>>()
        ;

        // stores the resulting string
        let mut content = String::new();

        // the progress bar displaying the progress of the report generation
        let mut bar = Bar::new(entries.len() as i64);
        bar.set_theme(Themes::ColoredSmall);

        for entry in entries {
            match entry {
                StringOrFutureTestResult::First(s) => {
                    content.push_str(&s);
                }

                StringOrFutureTestResult::Second(r) => {
                    // wait for the result, which are computed in parallelized worker threads
                    let result = r.await.unwrap();

                    // track statistics
                    match result {
                        TestResult::Success => stats.success  += 1,
                        TestResult::Error   => stats.errors   += 1,
                        TestResult::Failed  => stats.failed   += 1,
                        TestResult::Panic   => stats.panicked += 1,
                    }

                    stats.total += 1;

                    // create the table cell containing the test result
                    content.push_str(&format!("  {}   |", result.get_icon()));
                }
            };

            // increment the step for the progress bar
            bar.inc();
        }

        (content, stats)
    }


    /// Updates the target file with the generated content.
    /// The file won't be updated if the content is the same.
    pub fn export_to_file(&mut self, title: &str, path: &PathBuf) -> TestStats {
        let (content, stats) = self.collect_contents();

        let final_content = format!(
r#"## {} Test ROM Results

{}
{}
"#,
            title,
            Self::create_devices_header(),
            content
        );

        update_file(path, &final_content);

        stats
    }
}


impl TestConfigVisitor for TestReportGenerator {
    fn on_open_module(&mut self, module_name: &str, state: &IteratorState) {
        let path = format!("{}/{}", state.path, module_name);
        self.set_headline(path);
    }


    fn on_close_module(&mut self, _module_name: &str, _state: &IteratorState) {
        // if any tests are pending, apply them before leaving the module
        self.submit_tests();
    }


    fn on_visit_test(&mut self, test_cfg: &EmulatorTestConfig, _state: &IteratorState) {
        let rom = &test_cfg.setup.cartridge_path;

        // submit any previous test, except if the new tests belong to the same ROM and the device
        // list can be stacked with the already pending tests
        self.submit_tests_if_not_stackable(&rom, &test_cfg.devices);

        // create test set, if none exists yet
        if self.pending_tests.is_none() {
            self.pending_tests = Some(TestCaseSet {
                rom:        rom.clone(),
                test_cases: HashMap::new(),
            })
        }

        // add all test cases into the existing set
        if let Some(pending_tests) = &mut self.pending_tests {
            for test_case in test_cfg.get_test_cases() {
                pending_tests.test_cases.insert(test_case.device, test_case);
            }
        }
    }
}
