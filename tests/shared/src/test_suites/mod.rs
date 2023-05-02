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
use crate::io_utils::TestConfigVisitorRef;
use crate::test_suites::blargg::visit_tests_blargg;
use crate::test_suites::gambatte::visit_tests_gambatte;
use crate::test_suites::mooneye::visit_tests_mooneye;

mod blargg;
mod gambatte;
mod mooneye;


type FileVisitor = fn(path: PathBuf, visitor: TestConfigVisitorRef);


/// A reference to an existing test suite. This contains the name of the test suite, the
/// subdirectory where the test suite is located and a function pointer to the test suite's
/// file visitor.
pub struct TestSuite {
    /// The name of the test suite.
    pub name: &'static str,

    /// The subdirectory where the test suite is located.
    pub subdir: &'static str,

    /// The test suite's file visitor.
    visitor: FileVisitor,
}


/// A list of all known test suites.
pub const ALL_TEST_SUITES : &'static [TestSuite] = &[
    TestSuite { name: "blargg",     subdir: "blargg",               visitor: visit_tests_blargg    },
    TestSuite { name: "gambatte",   subdir: "gambatte",             visitor: visit_tests_gambatte  },
    TestSuite { name: "mooneye",    subdir: "mooneye-test-suite",   visitor: visit_tests_mooneye   },
];


impl TestSuite {
    /// Starts iterating over files in the test suite's directory.
    /// The test suite's file visitor will check the files being present and create test
    /// configurations for each test being found.
    /// The [TestConfigVisitorRef] will be invoked for each test found.
    pub fn start(&self, root_path: &PathBuf, visitor: TestConfigVisitorRef) {
        let tests_path = root_path.join(self.subdir);
        (self.visitor)(tests_path, visitor);
    }
}


pub fn generate_all_tests() {
}
