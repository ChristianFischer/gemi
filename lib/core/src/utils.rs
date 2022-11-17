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

/// Combines a high and low byte into a 16 bit value.
pub const fn to_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

/// Splits a 16 bit value into it's high and low bytes.
pub const fn to_u8(value: u16) -> (u8, u8) {
    (get_high(value), get_low(value))
}

/// Get the high byte of a 16 bit value.
pub const fn get_high(value: u16) -> u8 {
    ((value >> 8) & 0xff) as u8
}

/// Get the low byte of a 16 bit value.
pub const fn get_low(value: u16) -> u8 {
    (value & 0xff) as u8
}

/// Get the n'th bit of the given value.
pub const fn get_bit(byte: u8, bit: u8) -> bool {
    (byte & (1 << bit)) != 0
}

/// Set the n'th bit of the given value.
pub const fn change_bit(byte: u8, bit: u8, value: bool) -> u8 {
    if value {
        set_bit(byte, bit)
    }
    else {
        clear_bit(byte, bit)
    }
}

/// Set the n'th bit of the given value to 0.
pub const fn clear_bit(byte: u8, bit: u8) -> u8 {
    byte & !((1 << bit) as u8)
}

/// Set the n'th bit of the given value to 1.
pub const fn set_bit(byte: u8, bit: u8) -> u8 {
    byte | ((1 << bit) as u8)
}

/// Get the integer value of a character, if it matches a hex digit (0-9, a-f)
pub fn as_hex_digit(c: char) -> Option<u8> {
    match c {
        '0' ..= '9' => Some((c as u8) - ('0' as u8)),
        'a' ..= 'f' => Some((c as u8) - ('a' as u8)),
        'A' ..= 'F' => Some((c as u8) - ('A' as u8)),
        _           => None,
    }
}


/// Add two numbers and the carry flag together.
/// Returns the numeric result and the new carry flag
pub const fn carrying_add_u8(a: u8, b: u8, carry: bool) -> (u8, bool, bool) {
    let half_add   = (a & 0x0f).wrapping_add(b & 0x0f).wrapping_add(carry as u8);
    let half_carry = (half_add & 0xf0) != 0;

    let (result1, carry1) = a.overflowing_add(b);
    let (result2, carry2) = result1.overflowing_add(carry as u8);
    let carry = carry1 || carry2;

    (result2, half_carry, carry)
}

/// Add two numbers and the carry flag together.
/// Returns the numeric result and the new carry flag
pub const fn carrying_add_u16(a: u16, b: u16, carry: bool) -> (u16, bool, bool) {
    let half_add   = (a & 0x0fff).wrapping_add(b & 0x0fff).wrapping_add(carry as u16);
    let half_carry = (half_add & 0xf000) != 0;

    let (result1, carry1) = a.overflowing_add(b);
    let (result2, carry2) = result1.overflowing_add(carry as u16);
    let carry = carry1 || carry2;

    (result2, half_carry, carry)
}


/// Subtracts two numbers and the carry flag.
/// Returns the numeric result and the new carry flag
pub const fn carrying_sub_u8(a: u8, b: u8, carry: bool) -> (u8, bool, bool) {
    let half_sub   = (a & 0x0f).wrapping_sub(b & 0x0f).wrapping_sub(carry as u8);
    let half_carry = (half_sub & 0xf0) != 0;

    let (result1, carry1) = a.overflowing_sub(b);
    let (result2, carry2) = result1.overflowing_sub(carry as u8);
    let carry = carry1 || carry2;

    (result2, half_carry, carry)
}

/// Subtracts two numbers and the carry flag.
/// Returns the numeric result and the new carry flag
pub const fn carrying_sub_u16(a: u16, b: u16, carry: bool) -> (u16, bool, bool) {
    let half_sub   = (a & 0x0fff).wrapping_sub(b & 0x0fff).wrapping_sub(carry as u16);
    let half_carry = (half_sub & 0xf000) != 0;

    let (result1, carry1) = a.overflowing_sub(b);
    let (result2, carry2) = result1.overflowing_sub(carry as u16);
    let carry = carry1 || carry2;

    (result2, !half_carry, !carry)
}


/// Adds a signed value to an unsigned value.
pub const fn signed_overflow_add_u8(a: u8, b: i8) -> (u8, bool, bool) {
    if b >= 0 {
        carrying_add_u8(a, b as u8, false)
    }
    else {
        let (result, half_carry, carry) = carrying_sub_u8(a, b.abs() as u8, false);
        (result, !half_carry, !carry)
    }
}

/// Adds a signed value to an unsigned value.
pub const fn signed_overflow_add_u16(a: u16, b: i16) -> (u16, bool, bool) {
    if b >= 0 {
        carrying_add_u16(a, b as u16, false)
    }
    else {
        let (result, half_carry, carry) = carrying_sub_u16(a, b.abs() as u16, false);
        (result, !half_carry, !carry)
    }
}
