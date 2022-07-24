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

use crate::cpu::{RegisterR16, RegisterR8};
use crate::gameboy::GameBoy;
use crate::memory::{MemoryRead, MemoryWrite};

/// Loads the content of a 8bit register into another one.
fn ld_r8_r8(gb: &mut GameBoy, dst: RegisterR8, src: RegisterR8) {
    let value = gb.cpu.get_r8(src);
    gb.cpu.set_r8(dst, value);
}

/// Loads a constant 8bit value from the current instruction pointer into a 8bit register.
fn ld_r8_u8(gb: &mut GameBoy, dst: RegisterR8) {
    let value = gb.cpu.fetch_u8();
    gb.cpu.set_r8(dst, value);
}

/// Loads the content of a 8bit register into the device memory.
fn ld_addr_r8(gb: &mut GameBoy, dst_address: u16, src: RegisterR8) {
    let value = gb.cpu.get_r8(src);
    gb.mem.write_u8(dst_address, value);
}

/// Writes a 8bit value to a given address in the device memory.
fn ld_addr_u8(gb: &mut GameBoy, dst_address: u16, value: u8) {
    gb.mem.write_u8(dst_address, value);
}

/// Writes a 8bit value to a given address in the device memory.
fn ld_addr_u16(gb: &mut GameBoy, dst_address: u16, value: u16) {
    gb.mem.write_u16(dst_address, value);
}

/// Loads the value on a given address into a 8bit register.
fn ld_r8_addr(gb: &mut GameBoy, dst: RegisterR8, src_address: u16) {
    let value = gb.mem.read_u8(src_address);
    gb.cpu.set_r8(dst, value);
}

/// Loads the content of a 8bit register into the address stored in the target R16 register.
fn ld_r16ptr_r8(gb: &mut GameBoy, dst: RegisterR16, src: RegisterR8) {
    let address = gb.cpu.get_r16(dst);
    ld_addr_r8(gb, address, src);
}

/// Loads the content of a 8bit register into a constant address of the device memory.
fn ld_u16ptr_r8(gb: &mut GameBoy, src: RegisterR8) {
    let address = gb.cpu.fetch_u16();
    ld_addr_r8(gb, address, src);
}

/// Loads a constant 8bit value from the current instruction pointer
/// into the address stored in the target R16 register.
fn ld_r16ptr_u8(gb: &mut GameBoy, dst: RegisterR16) {
    let address = gb.cpu.get_r16(dst);
    let value   = gb.cpu.fetch_u8();
    ld_addr_u8(gb, address, value);
}

/// Loads a 16bit value into a constant address of the device memory.
fn ld_u16ptr_u16v(gb: &mut GameBoy, value: u16) {
    let address = gb.cpu.fetch_u16();
    ld_addr_u16(gb, address, value);
}

/// Loads the value at the address stored in a 16bit register
/// into a 8bit register.
fn ld_r8_r16ptr(gb: &mut GameBoy, dst: RegisterR8, src: RegisterR16) {
    let address = gb.cpu.get_r16(src);
    ld_r8_addr(gb, dst, address);
}

/// Loads the value at the address stored in a 16bit constant
/// into a 8bit register.
fn ld_r8_u16ptr(gb: &mut GameBoy, dst: RegisterR8) {
    let address = gb.cpu.fetch_u16();
    ld_r8_addr(gb, dst, address);
}

/// Loads a constant 16bit value from the current instruction pointer into a 16bit register.
fn ld_r16_u16(gb: &mut GameBoy, dst: RegisterR16) {
    let value = gb.cpu.fetch_u16();
    gb.cpu.set_r16(dst, value);
}

/// Loads the value of a 8bit register into the device memory at the address (0xff00 + u8).
fn ldh_u8_r8(gb: &mut GameBoy, src: RegisterR8) {
    let value     = gb.cpu.get_r8(src);
    let address_h = gb.cpu.fetch_u8();
    let address   = 0xff00 | (address_h as u16);
    gb.mem.write_u8(address, value);
}

/// Loads a value from the device memory at the address (0xff00 + u8) into a 8bit register.
fn ldh_r8_u8(gb: &mut GameBoy, dst: RegisterR8) {
    let address_h = gb.cpu.fetch_u8();
    let address   = 0xff00 | (address_h as u16);
    let value     = gb.mem.read_u8(address);
    gb.cpu.set_r8(dst, value);
}

pub fn ld_a_a(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::A, RegisterR8::A);
}

pub fn ld_a_b(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::A, RegisterR8::B);
}

pub fn ld_a_c(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::A, RegisterR8::C);
}

pub fn ld_a_d(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::A, RegisterR8::D);
}

pub fn ld_a_e(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::A, RegisterR8::E);
}

pub fn ld_a_l(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::A, RegisterR8::L);
}

pub fn ld_a_h(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::A, RegisterR8::H);
}

pub fn ld_b_a(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::B, RegisterR8::A);
}

pub fn ld_b_b(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::B, RegisterR8::B);
}

pub fn ld_b_c(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::B, RegisterR8::C);
}

pub fn ld_b_d(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::B, RegisterR8::D);
}

pub fn ld_b_e(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::B, RegisterR8::E);
}

pub fn ld_b_l(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::B, RegisterR8::L);
}

pub fn ld_b_h(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::B, RegisterR8::H);
}

pub fn ld_c_a(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::C, RegisterR8::A);
}

pub fn ld_c_b(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::C, RegisterR8::B);
}

pub fn ld_c_c(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::C, RegisterR8::C);
}

pub fn ld_c_d(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::C, RegisterR8::D);
}

pub fn ld_c_e(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::C, RegisterR8::E);
}

pub fn ld_c_l(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::C, RegisterR8::L);
}

pub fn ld_c_h(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::C, RegisterR8::H);
}

pub fn ld_d_a(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::D, RegisterR8::A);
}

pub fn ld_d_b(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::D, RegisterR8::B);
}

pub fn ld_d_c(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::D, RegisterR8::C);
}

pub fn ld_d_d(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::D, RegisterR8::D);
}

pub fn ld_d_e(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::D, RegisterR8::E);
}

pub fn ld_d_l(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::D, RegisterR8::L);
}

pub fn ld_d_h(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::D, RegisterR8::H);
}

pub fn ld_e_a(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::E, RegisterR8::A);
}

pub fn ld_e_b(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::E, RegisterR8::B);
}

pub fn ld_e_c(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::E, RegisterR8::C);
}

pub fn ld_e_d(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::E, RegisterR8::D);
}

pub fn ld_e_e(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::E, RegisterR8::E);
}

pub fn ld_e_l(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::E, RegisterR8::L);
}

pub fn ld_e_h(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::E, RegisterR8::H);
}

pub fn ld_l_a(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::L, RegisterR8::A);
}

pub fn ld_l_b(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::L, RegisterR8::B);
}

pub fn ld_l_c(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::L, RegisterR8::C);
}

pub fn ld_l_d(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::L, RegisterR8::D);
}

pub fn ld_l_e(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::L, RegisterR8::E);
}

pub fn ld_l_l(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::L, RegisterR8::L);
}

pub fn ld_l_h(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::L, RegisterR8::H);
}

pub fn ld_h_a(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::H, RegisterR8::A);
}

pub fn ld_h_b(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::H, RegisterR8::B);
}

pub fn ld_h_c(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::H, RegisterR8::C);
}

pub fn ld_h_d(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::H, RegisterR8::D);
}

pub fn ld_h_e(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::H, RegisterR8::E);
}

pub fn ld_h_l(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::H, RegisterR8::L);
}

pub fn ld_h_h(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::H, RegisterR8::H);
}

pub fn ld_a_u8(gb: &mut GameBoy) {
    ld_r8_u8(gb, RegisterR8::A);
}

pub fn ld_b_u8(gb: &mut GameBoy) {
    ld_r8_u8(gb, RegisterR8::B);
}

pub fn ld_c_u8(gb: &mut GameBoy) {
    ld_r8_u8(gb, RegisterR8::C);
}

pub fn ld_d_u8(gb: &mut GameBoy) {
    ld_r8_u8(gb, RegisterR8::D);
}

pub fn ld_e_u8(gb: &mut GameBoy) {
    ld_r8_u8(gb, RegisterR8::E);
}

pub fn ld_h_u8(gb: &mut GameBoy) {
    ld_r8_u8(gb, RegisterR8::H);
}

pub fn ld_l_u8(gb: &mut GameBoy) {
    ld_r8_u8(gb, RegisterR8::L);
}

pub fn ld_bc_u16(gb: &mut GameBoy) {
    ld_r16_u16(gb, RegisterR16::BC);
}

pub fn ld_de_u16(gb: &mut GameBoy) {
    ld_r16_u16(gb, RegisterR16::DE);
}

pub fn ld_hl_u16(gb: &mut GameBoy) {
    ld_r16_u16(gb, RegisterR16::HL);
}

pub fn ld_sp_u16(gb: &mut GameBoy) {
    let value = gb.cpu.fetch_u16();
    gb.cpu.set_stack_pointer(value);
}

pub fn ld_a_u16ptr(gb: &mut GameBoy) {
    ld_r8_u16ptr(gb, RegisterR8::A);
}

pub fn ld_a_bcptr(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::A, RegisterR16::BC);
}

pub fn ld_a_deptr(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::A, RegisterR16::BC);
}

pub fn ld_a_hlptr(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL);
}

pub fn ld_a_hlptri(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL);
    gb.cpu.increment_r16(RegisterR16::HL);
}

pub fn ld_a_hlptrd(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL);
    gb.cpu.decrement_r16(RegisterR16::HL);
}

pub fn ld_b_hlptr(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::B, RegisterR16::HL);
}

pub fn ld_c_hlptr(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::C, RegisterR16::HL);
}

pub fn ld_d_hlptr(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::D, RegisterR16::HL);
}

pub fn ld_e_hlptr(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::E, RegisterR16::HL);
}

pub fn ld_h_hlptr(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::H, RegisterR16::HL);
}

pub fn ld_l_hlptr(gb: &mut GameBoy) {
    ld_r8_r16ptr(gb, RegisterR8::L, RegisterR16::HL);
}

pub fn ld_bcptr_a(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::BC, RegisterR8::A);
}

pub fn ld_deptr_a(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::DE, RegisterR8::A);
}

pub fn ld_hlptr_a(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::A);
}

pub fn ld_hlptri_a(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::A);
    gb.cpu.increment_r16(RegisterR16::HL);
}

pub fn ld_hlptrd_a(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::A);
    gb.cpu.decrement_r16(RegisterR16::HL);
}

pub fn ld_hlptr_b(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::B);
}

pub fn ld_hlptr_c(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::C);
}

pub fn ld_hlptr_d(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::D);
}

pub fn ld_hlptr_e(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::E);
}

pub fn ld_hlptr_h(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::H);
}

pub fn ld_hlptr_l(gb: &mut GameBoy) {
    ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::L);
}

pub fn ld_hlptr_u8(gb: &mut GameBoy) {
    ld_r16ptr_u8(gb, RegisterR16::HL);
}

pub fn ld_u16ptr_a(gb: &mut GameBoy) {
    ld_u16ptr_r8(gb, RegisterR8::A);
}

pub fn ld_u16ptr_sp(gb: &mut GameBoy) {
    ld_u16ptr_u16v(gb, gb.cpu.get_stack_pointer());
}

pub fn ldh_u8_a(gb: &mut GameBoy) {
    ldh_u8_r8(gb, RegisterR8::A);
}

pub fn ldh_a_u8(gb: &mut GameBoy) {
    ldh_u8_r8(gb, RegisterR8::A);
}
