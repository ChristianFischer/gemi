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

use crate::cpu::{CpuFlag, RegisterR16, RegisterR8};
use crate::gameboy::GameBoy;
use crate::memory::{MemoryRead, MemoryWrite};


////////////////////////////////////////////////
//// INC opcodes

/// Increments a value.
/// (r16) <- (r16) + 1
fn inc_r16ptr(gb: &mut GameBoy, r16_ptr: RegisterR16) {
    let address = gb.cpu.get_r16(RegisterR16::HL);
    let value   = gb.mem.read_u8(address);
    let result  = value + 1;
    gb.mem.write_u8(address, result);
}


pub fn inc_a(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::A);
}

pub fn inc_b(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::B);
}

pub fn inc_c(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::C);
}

pub fn inc_d(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::D);
}

pub fn inc_e(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::E);
}

pub fn inc_h(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::H);
}

pub fn inc_l(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::L);
}

pub fn inc_bc(gb: &mut GameBoy) {
    gb.cpu.increment_r16(RegisterR16::BC);
}

pub fn inc_de(gb: &mut GameBoy) {
    gb.cpu.increment_r16(RegisterR16::DE);
}

pub fn inc_hl(gb: &mut GameBoy) {
    gb.cpu.increment_r16(RegisterR16::HL);
}

pub fn inc_hlptr(gb: &mut GameBoy) {
    inc_r16ptr(gb, RegisterR16::HL);
}

pub fn inc_sp(gb: &mut GameBoy) {
    gb.cpu.increment_sp();
}


////////////////////////////////////////////////
//// DEC opcodes

/// Decrements a value.
/// (r16) <- (r16) - 1
fn dec_r16ptr(gb: &mut GameBoy, r16_ptr: RegisterR16) {
    let address = gb.cpu.get_r16(RegisterR16::HL);
    let value   = gb.mem.read_u8(address);
    let result  = value - 1;
    gb.mem.write_u8(address, result);
}


pub fn dec_a(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::A);
}

pub fn dec_b(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::B);
}

pub fn dec_c(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::C);
}

pub fn dec_d(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::D);
}

pub fn dec_e(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::E);
}

pub fn dec_h(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::H);
}

pub fn dec_l(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::L);
}

pub fn dec_bc(gb: &mut GameBoy) {
    gb.cpu.decrement_r16(RegisterR16::BC);
}

pub fn dec_de(gb: &mut GameBoy) {
    gb.cpu.decrement_r16(RegisterR16::DE);
}

pub fn dec_hl(gb: &mut GameBoy) {
    gb.cpu.decrement_r16(RegisterR16::HL);
}

pub fn dec_hlptr(gb: &mut GameBoy) {
    dec_r16ptr(gb, RegisterR16::HL);
}

pub fn dec_sp(gb: &mut GameBoy) {
    let sp_old = gb.cpu.get_stack_pointer();
    let sp_new = sp_old + 1;
    gb.cpu.set_stack_pointer(sp_new);
}


////////////////////////////////////////////////
//// ADD / ADC opcodes

/// Adds two values and stores it into a 8bit register.
/// r8 <- r8 + value + (carry flag, if add_carry)
fn add_r8_u8v(gb: &mut GameBoy, r8: RegisterR8, value: u8, add_carry: bool) {
    let carry      = add_carry && gb.cpu.is_flag_set(CpuFlag::Carry);
    let old_value = gb.cpu.get_r8(r8) as u32;
    let result    = old_value + (value as u32) + (carry as u32);
    gb.cpu.set_flags_by_result(old_value, result);
    gb.cpu.set_r8(r8, result as u8);
}

/// Adds two values and stores it into a 8bit register.
/// dst <- dst + u8 + (carry flag, if add_carry)
fn add_r8_u8(gb: &mut GameBoy, dst: RegisterR8, add_carry: bool) {
    let value = gb.cpu.fetch_u8();
    add_r8_u8v(gb, dst, value, add_carry);
}

/// Adds two values and stores it into a 8bit register.
/// dst <- dst + src + (carry flag, if add_carry)
fn add_r8_r8(gb: &mut GameBoy, dst: RegisterR8, src: RegisterR8, add_carry: bool) {
    let value = gb.cpu.get_r8(src);
    add_r8_u8v(gb, dst, value, add_carry);
}

/// Adds two values and stores it into a 8bit register.
/// dst <- dst + (src_ptr) + (carry flag, if add_carry)
fn add_r8_r16ptr(gb: &mut GameBoy, dst: RegisterR8, src_ptr: RegisterR16, add_carry: bool) {
    let address = gb.cpu.get_r16(src_ptr);
    let value   = gb.mem.read_u8(address);
    add_r8_u8v(gb, dst, value, add_carry);
}

/// Adds two values and stores it into a 16bit register.
/// r16 <- r16 + value
fn add_r16_u16v(gb: &mut GameBoy, r16: RegisterR16, value: u16) {
    let old_value = gb.cpu.get_r16(r16) as u32;
    let result    = old_value + (value as u32);
    gb.cpu.set_flags_by_result(old_value, result);
    gb.cpu.set_r16(r16, result as u16);
}

/// Adds two values and stores it into a 16bit register.
/// dst <- dst + src
fn add_r16_r16(gb: &mut GameBoy, dst: RegisterR16, src: RegisterR16) {
    let value = gb.cpu.get_r16(src);
    add_r16_u16v(gb, dst, value);
}


pub fn add_a_u8(gb: &mut GameBoy) {
    add_r8_u8(gb, RegisterR8::A, false);
}

pub fn add_a_a(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::A, false);
}

pub fn add_a_b(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::B, false);
}

pub fn add_a_c(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::C, false);
}

pub fn add_a_d(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::D, false);
}

pub fn add_a_e(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::E, false);
}

pub fn add_a_h(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::H, false);
}

pub fn add_a_l(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::L, false);
}

pub fn add_a_hlptr(gb: &mut GameBoy) {
    add_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL, false);
}

pub fn adc_a_u8(gb: &mut GameBoy) {
    add_r8_u8(gb, RegisterR8::A, true);
}

pub fn adc_a_a(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::A, true);
}

pub fn adc_a_b(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::B, true);
}

pub fn adc_a_c(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::C, true);
}

pub fn adc_a_d(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::D, true);
}

pub fn adc_a_e(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::E, true);
}

pub fn adc_a_h(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::H, true);
}

pub fn adc_a_l(gb: &mut GameBoy) {
    add_r8_r8(gb, RegisterR8::A, RegisterR8::L, true);
}

pub fn adc_a_hlptr(gb: &mut GameBoy) {
    add_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL, true);
}

pub fn add_hl_bc(gb: &mut GameBoy) {
    add_r16_r16(gb, RegisterR16::HL, RegisterR16::BC);
}

pub fn add_hl_de(gb: &mut GameBoy) {
    add_r16_r16(gb, RegisterR16::HL, RegisterR16::DE);
}

pub fn add_hl_hl(gb: &mut GameBoy) {
    add_r16_r16(gb, RegisterR16::HL, RegisterR16::HL);
}

pub fn add_hl_sp(gb: &mut GameBoy) {
    add_r16_u16v(gb, RegisterR16::HL, gb.cpu.get_stack_pointer());
}


////////////////////////////////////////////////
//// SUB / SBC opcodes

/// Subtracts a value from another one and stores the result into a 8bit register.
/// r8 <- r8 - value - (carry flag, if sub_carry)
fn sub_r8_u8v(gb: &mut GameBoy, r8: RegisterR8, value: u8, sub_carry: bool) {
    let carry      = sub_carry && gb.cpu.is_flag_set(CpuFlag::Carry);
    let old_value = gb.cpu.get_r8(r8) as u32;
    let result    = old_value - (value as u32) - (carry as u32);
    gb.cpu.set_flags_by_result(old_value, result);
    gb.cpu.set_r8(r8, result as u8);
}

/// Subtracts a value from another one and stores the result into a 8bit register.
/// dst <- dst - src - (carry flag, if sub_carry)
fn sub_r8_r8(gb: &mut GameBoy, dst: RegisterR8, src: RegisterR8, sub_carry: bool) {
    let value = gb.cpu.get_r8(src);
    sub_r8_u8v(gb, dst, value, sub_carry);
}

/// Subtracts a value from another one and stores the result into a 8bit register.
/// dst <- dst - (src_ptr) - (carry flag, if sub_carry)
fn sub_r8_r16ptr(gb: &mut GameBoy, dst: RegisterR8, src_ptr: RegisterR16, sub_carry: bool) {
    let address = gb.cpu.get_r16(src_ptr);
    let value   = gb.mem.read_u8(address);
    sub_r8_u8v(gb, dst, value, sub_carry);
}

/// Subtracts a value from another one and stores the result into a 8bit register.
/// r16 <- r16 - value
fn sub_r16_u16v(gb: &mut GameBoy, r16: RegisterR16, value: u16) {
    let old_value = gb.cpu.get_r16(r16) as u32;
    let result    = old_value - (value as u32);
    gb.cpu.set_flags_by_result(old_value, result);
    gb.cpu.set_r16(r16, result as u16);
}

/// Subtracts a value from another one and stores the result into a 8bit register.
/// dst <- dst - src
fn sub_r16_r16(gb: &mut GameBoy, dst: RegisterR16, src: RegisterR16) {
    let value = gb.cpu.get_r16(src);
    sub_r16_u16v(gb, dst, value);
}


pub fn sub_a_a(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::A, false);
}

pub fn sub_a_b(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::B, false);
}

pub fn sub_a_c(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::C, false);
}

pub fn sub_a_d(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::D, false);
}

pub fn sub_a_e(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::E, false);
}

pub fn sub_a_h(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::H, false);
}

pub fn sub_a_l(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::L, false);
}

pub fn sub_a_hlptr(gb: &mut GameBoy) {
    sub_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL, false);
}

pub fn sbc_a_a(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::A, true);
}

pub fn sbc_a_b(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::B, true);
}

pub fn sbc_a_c(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::C, true);
}

pub fn sbc_a_d(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::D, true);
}

pub fn sbc_a_e(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::E, true);
}

pub fn sbc_a_h(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::H, true);
}

pub fn sbc_a_l(gb: &mut GameBoy) {
    sub_r8_r8(gb, RegisterR8::A, RegisterR8::L, true);
}

pub fn sbc_a_hlptr(gb: &mut GameBoy) {
    sub_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL, true);
}

pub fn sub_hl_bc(gb: &mut GameBoy) {
    sub_r16_r16(gb, RegisterR16::HL, RegisterR16::BC);
}

pub fn sub_hl_de(gb: &mut GameBoy) {
    sub_r16_r16(gb, RegisterR16::HL, RegisterR16::DE);
}

pub fn sub_hl_hl(gb: &mut GameBoy) {
    sub_r16_r16(gb, RegisterR16::HL, RegisterR16::HL);
}

pub fn sub_hl_sp(gb: &mut GameBoy) {
    sub_r16_u16v(gb, RegisterR16::HL, gb.cpu.get_stack_pointer());
}


////////////////////////////////////////////////
//// RL / RLC opcodes

/// Rotates the value of a register to the left through the carry flag.
fn rl_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let carry    = gb.cpu.is_flag_set(CpuFlag::Carry) as u8;
    let value    = gb.cpu.get_r8(r8);
    let left_bit = (value >> 7) & 1;
    let result   = (value << 1) | carry;
    gb.cpu.set_flag(CpuFlag::Carry, left_bit != 0);
    gb.cpu.set_flag(CpuFlag::Zero,  result != 0);
    gb.cpu.set_r8(r8, result);
}

/// Rotates the value of a register to the left.
fn rlc_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value    = gb.cpu.get_r8(r8);
    let left_bit = (value >> 7) & 1;
    let result   = (value << 1) | left_bit;
    gb.cpu.set_flag(CpuFlag::Carry, left_bit != 0);
    gb.cpu.set_flag(CpuFlag::Zero,  result != 0);
    gb.cpu.set_r8(r8, result);
}


pub fn rl_a(gb: &mut GameBoy) {
    rl_r8(gb, RegisterR8::A);
}

pub fn rlc_a(gb: &mut GameBoy) {
    rlc_r8(gb, RegisterR8::A);
}


////////////////////////////////////////////////
//// RR / RRC opcodes

/// Rotates the value of a register to the right through the carry flag.
fn rr_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let carry     = gb.cpu.is_flag_set(CpuFlag::Carry) as u8;
    let value     = gb.cpu.get_r8(r8);
    let right_bit = value & 1;
    let result    = (value >> 1) | (carry << 7);
    gb.cpu.set_flag(CpuFlag::Carry, right_bit != 0);
    gb.cpu.set_flag(CpuFlag::Zero,  result != 0);
    gb.cpu.set_r8(r8, result);
}

/// Rotates the value of a register to the right.
fn rrc_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value     = gb.cpu.get_r8(r8);
    let right_bit = value & 1;
    let result    = (value >> 1) | (right_bit << 7);
    gb.cpu.set_flag(CpuFlag::Carry, right_bit != 0);
    gb.cpu.set_flag(CpuFlag::Zero,  result != 0);
    gb.cpu.set_r8(r8, result);
}


pub fn rr_a(gb: &mut GameBoy) {
    rr_r8(gb, RegisterR8::A);
}

pub fn rrc_a(gb: &mut GameBoy) {
    rrc_r8(gb, RegisterR8::A);
}


////////////////////////////////////////////////
//// CP opcodes

/// Compares two values.
fn cp_u8v_u8v(gb: &mut GameBoy, value1: u8, value2: u8) {
    let result = value1 - value2;
    gb.cpu.set_flags_by_result(value1 as u32, result as u32);
    gb.cpu.set_flag(CpuFlag::Negative, true);
}

/// Compares two values.
/// cp r8, u8
fn cp_r8_u8(gb: &mut GameBoy, r8: RegisterR8) {
    let value1 = gb.cpu.get_r8(r8);
    let value2 = gb.cpu.fetch_u8();
    cp_u8v_u8v(gb, value1, value2);
}

/// Compares two values.
/// cp dst, src
fn cp_r8_r8(gb: &mut GameBoy, dst: RegisterR8, src: RegisterR8) {
    let value1 = gb.cpu.get_r8(dst);
    let value2 = gb.cpu.get_r8(src);
    cp_u8v_u8v(gb, value1, value2);
}

/// Compares two values.
/// cp dst, (src_ptr)
fn cp_r8_r16ptr(gb: &mut GameBoy, dst: RegisterR8, src_ptr: RegisterR16) {
    let value1  = gb.cpu.get_r8(dst);
    let address = gb.cpu.get_r16(src_ptr);
    let value2  = gb.mem.read_u8(address);
    cp_u8v_u8v(gb, value1, value2);
}


pub fn cp_a_u8(gb: &mut GameBoy) {
    cp_r8_u8(gb, RegisterR8::A);
}

pub fn cp_a_a(gb: &mut GameBoy) {
    cp_r8_r8(gb, RegisterR8::A, RegisterR8::A);
}

pub fn cp_a_b(gb: &mut GameBoy) {
    cp_r8_r8(gb, RegisterR8::A, RegisterR8::B);
}

pub fn cp_a_c(gb: &mut GameBoy) {
    cp_r8_r8(gb, RegisterR8::A, RegisterR8::C);
}

pub fn cp_a_d(gb: &mut GameBoy) {
    cp_r8_r8(gb, RegisterR8::A, RegisterR8::D);
}

pub fn cp_a_e(gb: &mut GameBoy) {
    cp_r8_r8(gb, RegisterR8::A, RegisterR8::E);
}

pub fn cp_a_h(gb: &mut GameBoy) {
    cp_r8_r8(gb, RegisterR8::A, RegisterR8::H);
}

pub fn cp_a_l(gb: &mut GameBoy) {
    cp_r8_r8(gb, RegisterR8::A, RegisterR8::L);
}

pub fn cp_a_hlptr(gb: &mut GameBoy) {
    cp_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL);
}


////////////////////////////////////////////////
//// AND opcodes

/// Computes a bitwise AND.
/// r8 <- r8 & value
fn and_r8_u8v(gb: &mut GameBoy, r8: RegisterR8, value: u8) {
    let old_value = gb.cpu.get_r8(r8);
    let result    = old_value & value;
    gb.cpu.set_flag(CpuFlag::Zero,      result != 0);
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, true);
    gb.cpu.set_flag(CpuFlag::Carry,     false);
    gb.cpu.set_r8(r8, result);
}

/// Computes a bitwise AND.
/// dst <- dst & src
fn and_r8_r8(gb: &mut GameBoy, dst: RegisterR8, src: RegisterR8) {
    let value = gb.cpu.get_r8(src);
    and_r8_u8v(gb, dst, value);
}

/// Computes a bitwise AND.
/// dst <- dst & (src_ptr)
fn and_r8_r16ptr(gb: &mut GameBoy, dst: RegisterR8, src_ptr: RegisterR16) {
    let address = gb.cpu.get_r16(src_ptr);
    let value   = gb.mem.read_u8(address);
    and_r8_u8v(gb, dst, value);
}


pub fn and_a_a(gb: &mut GameBoy) {
    and_r8_r8(gb, RegisterR8::A, RegisterR8::A);
}

pub fn and_a_b(gb: &mut GameBoy) {
    and_r8_r8(gb, RegisterR8::A, RegisterR8::B);
}

pub fn and_a_c(gb: &mut GameBoy) {
    and_r8_r8(gb, RegisterR8::A, RegisterR8::C);
}

pub fn and_a_d(gb: &mut GameBoy) {
    and_r8_r8(gb, RegisterR8::A, RegisterR8::D);
}

pub fn and_a_e(gb: &mut GameBoy) {
    and_r8_r8(gb, RegisterR8::A, RegisterR8::E);
}

pub fn and_a_h(gb: &mut GameBoy) {
    and_r8_r8(gb, RegisterR8::A, RegisterR8::H);
}

pub fn and_a_l(gb: &mut GameBoy) {
    and_r8_r8(gb, RegisterR8::A, RegisterR8::L);
}

pub fn and_a_hlptr(gb: &mut GameBoy) {
    and_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL);
}


////////////////////////////////////////////////
//// OR opcodes

/// Computes a bitwise OR.
/// r8 <- r8 | value
fn or_r8_u8v(gb: &mut GameBoy, r8: RegisterR8, value: u8) {
    let old_value = gb.cpu.get_r8(r8);
    let result    = old_value | value;
    gb.cpu.set_flag(CpuFlag::Zero,      result != 0);
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, false);
    gb.cpu.set_flag(CpuFlag::Carry,     false);
    gb.cpu.set_r8(r8, result);
}

/// Computes a bitwise OR.
/// dst <- dst | src
fn or_r8_r8(gb: &mut GameBoy, dst: RegisterR8, src: RegisterR8) {
    let value = gb.cpu.get_r8(src);
    or_r8_u8v(gb, dst, value);
}

/// Computes a bitwise OR.
/// dst <- dst | (src_ptr)
fn or_r8_r16ptr(gb: &mut GameBoy, dst: RegisterR8, src_ptr: RegisterR16) {
    let address = gb.cpu.get_r16(src_ptr);
    let value   = gb.mem.read_u8(address);
    or_r8_u8v(gb, dst, value);
}


pub fn or_a_a(gb: &mut GameBoy) {
    or_r8_r8(gb, RegisterR8::A, RegisterR8::A);
}

pub fn or_a_b(gb: &mut GameBoy) {
    or_r8_r8(gb, RegisterR8::A, RegisterR8::B);
}

pub fn or_a_c(gb: &mut GameBoy) {
    or_r8_r8(gb, RegisterR8::A, RegisterR8::C);
}

pub fn or_a_d(gb: &mut GameBoy) {
    or_r8_r8(gb, RegisterR8::A, RegisterR8::D);
}

pub fn or_a_e(gb: &mut GameBoy) {
    or_r8_r8(gb, RegisterR8::A, RegisterR8::E);
}

pub fn or_a_h(gb: &mut GameBoy) {
    or_r8_r8(gb, RegisterR8::A, RegisterR8::H);
}

pub fn or_a_l(gb: &mut GameBoy) {
    or_r8_r8(gb, RegisterR8::A, RegisterR8::L);
}

pub fn or_a_hlptr(gb: &mut GameBoy) {
    or_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL);
}


////////////////////////////////////////////////
//// XOR opcodes

/// Computes a bitwise XOR.
/// r8 <- r8 ^ value
fn xor_r8_u8v(gb: &mut GameBoy, r8: RegisterR8, value: u8) {
    let old_value = gb.cpu.get_r8(r8);
    let result    = old_value ^ value;
    gb.cpu.set_flag(CpuFlag::Zero,      result != 0);
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, false);
    gb.cpu.set_flag(CpuFlag::Carry,     false);
    gb.cpu.set_r8(r8, result);
}

/// Computes a bitwise XOR.
/// dst <- dst ^ src
fn xor_r8_r8(gb: &mut GameBoy, dst: RegisterR8, src: RegisterR8) {
    let value = gb.cpu.get_r8(src);
    xor_r8_u8v(gb, dst, value);
}

/// Computes a bitwise XOR.
/// dst <- dst ^ (src_ptr)
fn xor_r8_r16ptr(gb: &mut GameBoy, dst: RegisterR8, src_ptr: RegisterR16) {
    let address = gb.cpu.get_r16(src_ptr);
    let value   = gb.mem.read_u8(address);
    xor_r8_u8v(gb, dst, value);
}


fn xor_r8_u8(gb: &mut GameBoy, r8: RegisterR8) {
    let value1 = gb.cpu.get_r8(r8);
    let value2 = gb.cpu.fetch_u8();
    let result = value1 ^ value2;
    gb.cpu.set_r8(r8, result);
}


pub fn xor_a_u8(gb: &mut GameBoy) {
    xor_r8_u8(gb, RegisterR8::A);
}

pub fn xor_a_a(gb: &mut GameBoy) {
    xor_r8_r8(gb, RegisterR8::A, RegisterR8::A);
}

pub fn xor_a_b(gb: &mut GameBoy) {
    xor_r8_r8(gb, RegisterR8::A, RegisterR8::B);
}

pub fn xor_a_c(gb: &mut GameBoy) {
    xor_r8_r8(gb, RegisterR8::A, RegisterR8::C);
}

pub fn xor_a_d(gb: &mut GameBoy) {
    xor_r8_r8(gb, RegisterR8::A, RegisterR8::D);
}

pub fn xor_a_e(gb: &mut GameBoy) {
    xor_r8_r8(gb, RegisterR8::A, RegisterR8::E);
}

pub fn xor_a_h(gb: &mut GameBoy) {
    xor_r8_r8(gb, RegisterR8::A, RegisterR8::H);
}

pub fn xor_a_l(gb: &mut GameBoy) {
    xor_r8_r8(gb, RegisterR8::A, RegisterR8::L);
}

pub fn xor_a_hlptr(gb: &mut GameBoy) {
    xor_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL);
}


////////////////////////////////////////////////
//// other

/// Convert a BCD Number.
pub fn daa(gb: &mut GameBoy) {
    todo!();
}

/// Complement
pub fn cpl_a(gb: &mut GameBoy) {
    let value  = gb.cpu.get_r8(RegisterR8::A);
    let result = !value;
    gb.cpu.set_r8(RegisterR8::A, result);
}

/// Set Carry flag.
pub fn scf(gb: &mut GameBoy) {
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, false);
    gb.cpu.set_flag(CpuFlag::Carry,     true);
}

/// Change carry flag
pub fn ccf(gb: &mut GameBoy) {
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, false);
    gb.cpu.set_flag(CpuFlag::Carry,     !gb.cpu.is_flag_set(CpuFlag::Carry));
}

