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
use tests_shared::config::TESTRUNNER_SUBDIR_ROM_FILES;
use tests_shared::io_utils::Workspace;
use tests_shared::runner::{print_run_command, run_test_case_for_result};
use tests_shared::test_config::EmulatorTestCase;


/// Runs a test case and checks for it's result.
/// Passes on success, but panics on each error.
pub fn run_test_case(test_case: EmulatorTestCase) {
    // unit test workspace
    let workspace = Workspace::for_root_path(PathBuf::from(TESTRUNNER_SUBDIR_ROM_FILES));

    // print commandline arg to easily re-run the test
    print_run_command(&workspace, &test_case.device, &test_case.setup);

    // run the test case and fail on error
    run_test_case_for_result(
        &workspace,
        &test_case
    ).unwrap();
}
