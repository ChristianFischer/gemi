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

use gbemu_core::gameboy::GameBoy;

const MOONEYE_RESULT_SEQ_PASS : &[u8] = &[3, 5, 8, 13, 21, 34];
const MOONEYE_RESULT_SEQ_FAIL : &[u8] = &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42];


/// Takes a reference to an emulator running a mooneye test rom and checks whether it's passed or not.
/// Mooneye test roms send a specific sequence of bytes to the serial port to indicate whether they passed or not.
pub fn check_mooneye_test_passed(gb: &GameBoy) {
    let test_result_message = gb.get_peripherals().serial.get_output();

    // Fail if the test rom sent the fail sequence
    assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");

    // Fail if the test rom didn't send the pass sequence
    assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
}
