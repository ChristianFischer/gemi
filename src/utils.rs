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