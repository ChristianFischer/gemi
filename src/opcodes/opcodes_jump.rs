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

use crate::cpu::CpuFlag;
use crate::gameboy::GameBoy;

/// Performs a jump to an address if a condition is met.
fn jp_if_u16(gb: &mut GameBoy, flag: CpuFlag, value: bool) {
    let address = gb.cpu.fetch_u16();
    if gb.cpu.is_flag_set(flag) == value {
        gb.cpu.jump_to(address);
    }
}

/// Performs a relative jump if a condition is met.
fn jr_if_i8(gb: &mut GameBoy, flag: CpuFlag, value: bool) {
    let offset = gb.cpu.fetch_i8();
    if gb.cpu.is_flag_set(flag) == value {
        gb.cpu.jump_relative(offset as i16);
    }
}

pub fn jr_i8(gb: &mut GameBoy) {
    let offset = gb.cpu.fetch_i8();
    gb.cpu.jump_relative(offset as i16);
}

pub fn jp_u16(gb: &mut GameBoy) {
    let address = gb.cpu.fetch_u16();
    gb.cpu.jump_to(address);
}

pub fn jr_z_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Zero, true);
}

pub fn jr_n_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Negative, true);
}

pub fn jr_h_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::HalfCarry, true);
}

pub fn jr_c_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Carry, true);
}

pub fn jr_nz_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Zero, false);
}

pub fn jr_nn_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Negative, false);
}

pub fn jr_nh_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::HalfCarry, false);
}

pub fn jr_nc_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Carry, false);
}

pub fn jp_z_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Zero, true);
}

pub fn jp_n_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Negative, true);
}

pub fn jp_h_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::HalfCarry, true);
}

pub fn jp_c_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Carry, true);
}

pub fn jp_nz_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Zero, false);
}

pub fn jp_nn_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Negative, false);
}

pub fn jp_nh_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::HalfCarry, false);
}

pub fn jp_nc_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Carry, false);
}
