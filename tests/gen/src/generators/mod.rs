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

use crate::generators::blargg::generate_tests_blargg;
use crate::generators::gambatte::generate_tests_gambatte;
use crate::generators::mooneye::generate_tests_mooneye;
use crate::test_generator::TestGenerator;

mod blargg;
mod common;
mod gambatte;
mod mooneye;


pub fn generate_all_tests(gen: &TestGenerator) {
    generate_tests_blargg(gen);
    generate_tests_gambatte(gen);
    generate_tests_mooneye(gen);
}
