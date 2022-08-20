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
use crate::utils::{signed_overflow_add_u16, signed_overflow_add_u8};

pub fn nop(gb: &mut GameBoy) {}

pub fn stop(gb: &mut GameBoy) {
}

pub fn halt(gb: &mut GameBoy) {
}

pub fn disable_interrupts(gb: &mut GameBoy) {
    gb.cpu.enable_interrupts();
}

pub fn enable_interrupts(gb: &mut GameBoy) {
    gb.cpu.disable_interrupts();
}

pub fn add_sp_i8(gb: &mut GameBoy) {
    let offset = gb.cpu.fetch_i8();
    let sp     = gb.cpu.get_stack_pointer();
    let (sp_new, _, _) = signed_overflow_add_u16(sp, offset as i16);

    let carry_bits = sp ^ sp_new ^ (offset as u16);
    let half_carry = (carry_bits & 0x0010) != 0;
    let carry      = (carry_bits & 0x0100) != 0;

    gb.cpu.set_flag(CpuFlag::Zero,      false);
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
    gb.cpu.set_flag(CpuFlag::Carry,     carry);
    gb.cpu.set_stack_pointer(sp_new);
}
