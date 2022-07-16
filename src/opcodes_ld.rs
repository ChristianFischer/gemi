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

use crate::cpu::{Cpu, RegisterR16, RegisterR8};
use crate::gameboy::GameBoy;
use crate::memory::MemoryWrite;

/// Loads the content of a 8bit register into another one.
fn ld_r8_r8(gb: &mut GameBoy, dst: RegisterR8, src: RegisterR8) {
    let value = gb.cpu.get_r8(src);
    gb.cpu.set_r8(dst, value);
}

/// Loads the content of a 8bit register into the device memory.
fn ld_addr_r8(gb: &mut GameBoy, dst_address: u16, src: RegisterR8) {
    let value = gb.cpu.get_r8(src);
    gb.mem.write_u8(dst_address, value);
}

/// Loads a constant 16bit value from the current instruction pointer into a 16bit register.
fn ld_r16_u16(gb: &mut GameBoy, dst: RegisterR16) {
    let value = gb.cpu.fetch_u16();
    gb.cpu.set_r16(dst, value);
}

pub fn ld_d_a(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::D, RegisterR8::A);
}

pub fn ld_e_b(gb: &mut GameBoy) {
    ld_r8_r8(gb, RegisterR8::E, RegisterR8::B);
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

