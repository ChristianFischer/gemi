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

use crate::cpu::{CpuFlag, RegisterR16};
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

/// Calls a subroutine by storing the current instruction pointer on the stack
/// and set the instruction pointer to a new address.
fn call_addr(gb: &mut GameBoy, address: u16) {
    let instruction_pointer = gb.cpu.get_instruction_pointer();
    gb.cpu.push_u16(instruction_pointer);
    gb.cpu.set_instruction_pointer(address);
}

/// Calls a subroutine by storing the current instruction pointer on the stack
/// and set the instruction pointer to a new address.
fn call_addr_u16(gb: &mut GameBoy) {
    let address = gb.cpu.fetch_u16();
    call_addr(gb, address);
}

/// Calls a subroutine by storing the current instruction pointer on the stack
/// and set the instruction pointer to a new address, if a condition is met.
fn call_addr_if(gb: &mut GameBoy, flag: CpuFlag, value: bool, address: u16) {
    if gb.cpu.is_flag_set(flag) == value {
        call_addr(gb, address);
    }
}

/// Calls a subroutine by storing the current instruction pointer on the stack
/// and set the instruction pointer to a new address, taken from the current instruction pointer,
/// if a condition is met.
fn call_u16_if(gb: &mut GameBoy, flag: CpuFlag, value: bool) {
    let address = gb.cpu.fetch_u16();
    call_addr_if(gb, flag, value, address);
}

/// Returns from a subroutine by taking the previous instruction pointer address from the stack.
fn ret_from_call(gb: &mut GameBoy) {
    let instruction_pointer = gb.cpu.pop_u16();
    gb.cpu.set_instruction_pointer(instruction_pointer);
}

/// Returns from a subroutine by taking the previous instruction pointer address from the stack,
/// if a condition is met.
fn ret_if(gb: &mut GameBoy, flag: CpuFlag, value: bool) {
    if gb.cpu.is_flag_set(flag) == value {
        ret_from_call(gb);
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

pub fn jp_hl(gb: &mut GameBoy) {
    let address = gb.cpu.get_r16(RegisterR16::HL);
    gb.cpu.jump_to(address);
}

pub fn jr_z_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Zero, true);
}

pub fn jr_c_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Carry, true);
}

pub fn jr_nz_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Zero, false);
}

pub fn jr_nc_i8(gb: &mut GameBoy) {
    jr_if_i8(gb, CpuFlag::Carry, false);
}

pub fn jp_z_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Zero, true);
}

pub fn jp_c_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Carry, true);
}

pub fn jp_nz_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Zero, false);
}

pub fn jp_nc_u16(gb: &mut GameBoy) {
    jp_if_u16(gb, CpuFlag::Carry, false);
}

pub fn call_u16(gb: &mut GameBoy) {
    call_addr_u16(gb);
}

pub fn call_z_u16(gb: &mut GameBoy) {
    call_u16_if(gb, CpuFlag::Zero, true);
}

pub fn call_c_u16(gb: &mut GameBoy) {
    call_u16_if(gb, CpuFlag::Carry, true);
}

pub fn call_nz_u16(gb: &mut GameBoy) {
    call_u16_if(gb, CpuFlag::Zero, false);
}

pub fn call_nc_u16(gb: &mut GameBoy) {
    call_u16_if(gb, CpuFlag::Carry, false);
}

pub fn rst_00h(gb: &mut GameBoy) {
    call_addr(gb, 0x0000);
}

pub fn rst_08h(gb: &mut GameBoy) {
    call_addr(gb, 0x0008);
}

pub fn rst_10h(gb: &mut GameBoy) {
    call_addr(gb, 0x0010);
}

pub fn rst_18h(gb: &mut GameBoy) {
    call_addr(gb, 0x0018);
}

pub fn rst_20h(gb: &mut GameBoy) {
    call_addr(gb, 0x0020);
}

pub fn rst_28h(gb: &mut GameBoy) {
    call_addr(gb, 0x0028);
}

pub fn rst_30h(gb: &mut GameBoy) {
    call_addr(gb, 0x0030);
}

pub fn rst_38h(gb: &mut GameBoy) {
    call_addr(gb, 0x0038);
}

pub fn ret(gb: &mut GameBoy) {
    ret_from_call(gb);
}

pub fn ret_z(gb: &mut GameBoy) {
    ret_if(gb, CpuFlag::Zero, true);
}

pub fn ret_c(gb: &mut GameBoy) {
    ret_if(gb, CpuFlag::Carry, true);
}

pub fn ret_nz(gb: &mut GameBoy) {
    ret_if(gb, CpuFlag::Zero, false);
}

pub fn ret_nc(gb: &mut GameBoy) {
    ret_if(gb, CpuFlag::Carry, false);
}

pub fn reti(gb: &mut GameBoy) {
    ret_from_call(gb);
    gb.cpu.enable_interrupts();
}
