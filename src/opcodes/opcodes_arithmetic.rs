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
//// Flag types

enum ShiftOp {
    ShiftLogical,
    ShiftArithmetic,
    Rotate,
    RotateThroughCarry,
}


////////////////////////////////////////////////
//// INC opcodes

/// Increments a 8bit value.
fn increment_u8v(gb: &mut GameBoy, value: u8) -> u8 {
    let result = if value == 0xff {
        0x00
    }
    else {
        value + 1
    };

    gb.cpu.set_flags_by_result(value as u32, result as u32);
    gb.cpu.set_flag(CpuFlag::Negative, false);

    result
}

/// Increments a 16bit value.
fn increment_u16v(gb: &mut GameBoy, value: u16) -> u16 {
    let result = if value == 0xff {
        0x00
    }
    else {
        value + 1
    };

    result
}

/// Increments a value
/// r8 <- r8 + 1
fn increment_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = increment_u8v(gb, value);
    gb.cpu.set_r8(r8, result);
}

/// Increments a value
/// r16 <- r16 + 1
fn increment_r16(gb: &mut GameBoy, r16: RegisterR16) {
    let value  = gb.cpu.get_r16(r16);
    let result = increment_u16v(gb, value);
    gb.cpu.set_r16(r16, result);
}

/// Increments a value.
/// (r16) <- (r16) + 1
fn increment_r16ptr(gb: &mut GameBoy, r16_ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16_ptr);
    let value   = gb.mem.read_u8(address);
    let result  = increment_u8v(gb, value);
    gb.mem.write_u8(address, result);
}


pub fn inc_a(gb: &mut GameBoy) {
    increment_r8(gb, RegisterR8::A);
}

pub fn inc_b(gb: &mut GameBoy) {
    increment_r8(gb, RegisterR8::B);
}

pub fn inc_c(gb: &mut GameBoy) {
    increment_r8(gb, RegisterR8::C);
}

pub fn inc_d(gb: &mut GameBoy) {
    increment_r8(gb, RegisterR8::D);
}

pub fn inc_e(gb: &mut GameBoy) {
    increment_r8(gb, RegisterR8::E);
}

pub fn inc_h(gb: &mut GameBoy) {
    increment_r8(gb, RegisterR8::H);
}

pub fn inc_l(gb: &mut GameBoy) {
    increment_r8(gb, RegisterR8::L);
}

pub fn inc_bc(gb: &mut GameBoy) {
    increment_r16(gb, RegisterR16::BC);
}

pub fn inc_de(gb: &mut GameBoy) {
    increment_r16(gb, RegisterR16::DE);
}

pub fn inc_hl(gb: &mut GameBoy) {
    increment_r16(gb, RegisterR16::HL);
}

pub fn inc_hlptr(gb: &mut GameBoy) {
    increment_r16ptr(gb, RegisterR16::HL);
}

pub fn inc_sp(gb: &mut GameBoy) {
    let sp_old = gb.cpu.get_stack_pointer();
    let sp_new = sp_old + 1;
    gb.cpu.set_stack_pointer(sp_new);
}


////////////////////////////////////////////////
//// DEC opcodes

/// Decrements a 8bit value.
fn decrement_u8v(gb: &mut GameBoy, value: u8) -> u8 {
    let result = if value == 0x00 {
        0xff
    }
    else {
        value - 1
    };

    gb.cpu.set_flags_by_result(value as u32, result as u32);
    gb.cpu.set_flag(CpuFlag::Negative, true);

    result
}

/// Decrements a 16bit value.
fn decrement_u16v(gb: &mut GameBoy, value: u16) -> u16 {
    let result = if value == 0x00 {
        0xff
    }
    else {
        value - 1
    };

    result
}

/// Decrements a value
/// r8 <- r8 - 1
fn decrement_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = decrement_u8v(gb, value);
    gb.cpu.set_r8(r8, result);
}

/// Decrements a value
/// r16 <- r16 - 1
fn decrement_r16(gb: &mut GameBoy, r16: RegisterR16) {
    let value  = gb.cpu.get_r16(r16);
    let result = decrement_u16v(gb, value);
    gb.cpu.set_r16(r16, result);
}

/// Decrements a value.
/// (r16) <- (r16) - 1
fn decrement_r16ptr(gb: &mut GameBoy, r16_ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16_ptr);
    let value   = gb.mem.read_u8(address);
    let result  = decrement_u8v(gb, value);
    gb.mem.write_u8(address, result);
}


pub fn dec_a(gb: &mut GameBoy) {
    decrement_r8(gb, RegisterR8::A);
}

pub fn dec_b(gb: &mut GameBoy) {
    decrement_r8(gb, RegisterR8::B);
}

pub fn dec_c(gb: &mut GameBoy) {
    decrement_r8(gb, RegisterR8::C);
}

pub fn dec_d(gb: &mut GameBoy) {
    decrement_r8(gb, RegisterR8::D);
}

pub fn dec_e(gb: &mut GameBoy) {
    decrement_r8(gb, RegisterR8::E);
}

pub fn dec_h(gb: &mut GameBoy) {
    decrement_r8(gb, RegisterR8::H);
}

pub fn dec_l(gb: &mut GameBoy) {
    decrement_r8(gb, RegisterR8::L);
}

pub fn dec_bc(gb: &mut GameBoy) {
    decrement_r16(gb, RegisterR16::BC);
}

pub fn dec_de(gb: &mut GameBoy) {
    decrement_r16(gb, RegisterR16::DE);
}

pub fn dec_hl(gb: &mut GameBoy) {
    decrement_r16(gb, RegisterR16::HL);
}

pub fn dec_hlptr(gb: &mut GameBoy) {
    decrement_r16ptr(gb, RegisterR16::HL);
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
    let result    = ((old_value as i32) - (value as i32) - (carry as i32)) as u32;
    gb.cpu.set_flags_by_result(old_value, result);
    gb.cpu.set_r8(r8, result as u8);
}

/// Adds two values and stores it into a 8bit register.
/// dst <- dst + u8 + (carry flag, if add_carry)
fn sub_r8_u8(gb: &mut GameBoy, dst: RegisterR8, add_carry: bool) {
    let value = gb.cpu.fetch_u8();
    sub_r8_u8v(gb, dst, value, add_carry);
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


pub fn sub_a_u8(gb: &mut GameBoy) {
    sub_r8_u8(gb, RegisterR8::A, false);
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

pub fn sbc_a_u8(gb: &mut GameBoy) {
    sub_r8_u8(gb, RegisterR8::A, true);
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


////////////////////////////////////////////////
//// RL / RLC opcodes

/// Shifts or rotates a value to the left.
fn shift_left_u8v(gb: &mut GameBoy, value: u8, op: ShiftOp) -> u8 {
    let carry    = gb.cpu.is_flag_set(CpuFlag::Carry) as u8;
    let left_bit = (value >> 7) & 1;

    let result = (value << 1) | (match op {
        ShiftOp::ShiftLogical       => (value << 1) | 0x0000,
        ShiftOp::ShiftArithmetic    => (value << 1) | 0x0000,
        ShiftOp::Rotate             => (value << 1) | left_bit,
        ShiftOp::RotateThroughCarry => (value << 1) | carry,
    });

    gb.cpu.set_flag(CpuFlag::Carry, left_bit != 0);
    gb.cpu.set_flag(CpuFlag::Zero,  result != 0);

    result
}

/// Performs an arithmetic shift left of the value of a register.
fn sla_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = shift_left_u8v(gb, value, ShiftOp::ShiftArithmetic);
    gb.cpu.set_r8(r8, result);
}

/// Performs an arithmetic shift left of the value on a memory location.
fn sla_r16ptr(gb: &mut GameBoy, r16ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16ptr);
    let value   = gb.mem.read_u8(address);
    let result  = shift_left_u8v(gb, value, ShiftOp::ShiftArithmetic);
    gb.mem.write_u8(address, result);
}

/// Rotates the value of a register to the left through the carry flag.
fn rl_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = shift_left_u8v(gb, value, ShiftOp::RotateThroughCarry);
    gb.cpu.set_r8(r8, result);
}

/// Rotates the value on a memory location to the left through the carry flag.
fn rl_r16ptr(gb: &mut GameBoy, r16ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16ptr);
    let value   = gb.mem.read_u8(address);
    let result  = shift_left_u8v(gb, value, ShiftOp::RotateThroughCarry);
    gb.mem.write_u8(address, result);
}

/// Rotates the value of a register to the left.
fn rlc_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = shift_left_u8v(gb, value, ShiftOp::Rotate);
    gb.cpu.set_r8(r8, result);
}

/// Rotates the value on a memory location to the left.
fn rlc_r16ptr(gb: &mut GameBoy, r16ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16ptr);
    let value   = gb.mem.read_u8(address);
    let result  = shift_left_u8v(gb, value, ShiftOp::Rotate);
    gb.mem.write_u8(address, result);
}


pub fn sla_a(gb: &mut GameBoy) {
    sla_r8(gb, RegisterR8::A);
}

pub fn sla_b(gb: &mut GameBoy) {
    sla_r8(gb, RegisterR8::B);
}

pub fn sla_c(gb: &mut GameBoy) {
    sla_r8(gb, RegisterR8::C);
}

pub fn sla_d(gb: &mut GameBoy) {
    sla_r8(gb, RegisterR8::D);
}

pub fn sla_e(gb: &mut GameBoy) {
    sla_r8(gb, RegisterR8::E);
}

pub fn sla_h(gb: &mut GameBoy) {
    sla_r8(gb, RegisterR8::H);
}

pub fn sla_l(gb: &mut GameBoy) {
    sla_r8(gb, RegisterR8::L);
}

pub fn sla_hlptr(gb: &mut GameBoy) {
    sla_r16ptr(gb, RegisterR16::HL);
}

pub fn rla(gb: &mut GameBoy) {
    rl_r8(gb, RegisterR8::A);
}

pub fn rl_a(gb: &mut GameBoy) {
    rl_r8(gb, RegisterR8::A);
}

pub fn rl_b(gb: &mut GameBoy) {
    rl_r8(gb, RegisterR8::B);
}

pub fn rl_c(gb: &mut GameBoy) {
    rl_r8(gb, RegisterR8::C);
}

pub fn rl_d(gb: &mut GameBoy) {
    rl_r8(gb, RegisterR8::D);
}

pub fn rl_e(gb: &mut GameBoy) {
    rl_r8(gb, RegisterR8::E);
}

pub fn rl_h(gb: &mut GameBoy) {
    rl_r8(gb, RegisterR8::H);
}

pub fn rl_l(gb: &mut GameBoy) {
    rl_r8(gb, RegisterR8::L);
}

pub fn rl_hlptr(gb: &mut GameBoy) {
    rl_r16ptr(gb, RegisterR16::HL);
}

pub fn rlca(gb: &mut GameBoy) {
    rlc_r8(gb, RegisterR8::A);
}

pub fn rlc_a(gb: &mut GameBoy) {
    rlc_r8(gb, RegisterR8::A);
}

pub fn rlc_b(gb: &mut GameBoy) {
    rlc_r8(gb, RegisterR8::B);
}

pub fn rlc_c(gb: &mut GameBoy) {
    rlc_r8(gb, RegisterR8::C);
}

pub fn rlc_d(gb: &mut GameBoy) {
    rlc_r8(gb, RegisterR8::D);
}

pub fn rlc_e(gb: &mut GameBoy) {
    rlc_r8(gb, RegisterR8::E);
}

pub fn rlc_h(gb: &mut GameBoy) {
    rlc_r8(gb, RegisterR8::H);
}

pub fn rlc_l(gb: &mut GameBoy) {
    rlc_r8(gb, RegisterR8::L);
}

pub fn rlc_hlptr(gb: &mut GameBoy) {
    rlc_r16ptr(gb, RegisterR16::HL);
}


////////////////////////////////////////////////
//// RR / RRC opcodes

/// Shifts or rotates a value to the left.
fn shift_right_u8v(gb: &mut GameBoy, value: u8, op: ShiftOp) -> u8 {
    let carry    = gb.cpu.is_flag_set(CpuFlag::Carry) as u8;
    let left_bit = (value >> 7) & 1;
    let right_bit= value & 1;

    let result = (value << 1) | (match op {
        ShiftOp::ShiftLogical       => (value >> 1) | 0x0000,
        ShiftOp::ShiftArithmetic    => (value >> 1) | (left_bit << 7),
        ShiftOp::Rotate             => (value >> 1) | (right_bit << 7),
        ShiftOp::RotateThroughCarry => (value >> 1) | carry,
    });

    gb.cpu.set_flag(CpuFlag::Carry, right_bit != 0);
    gb.cpu.set_flag(CpuFlag::Zero,  result != 0);

    result
}

/// Performs an arithmetic shift right of the value of a register.
fn sra_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = shift_right_u8v(gb, value, ShiftOp::ShiftArithmetic);
    gb.cpu.set_r8(r8, result);
}

/// Performs an arithmetic shift right of the value on a memory location.
fn sra_r16ptr(gb: &mut GameBoy, r16ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16ptr);
    let value   = gb.mem.read_u8(address);
    let result  = shift_right_u8v(gb, value, ShiftOp::ShiftArithmetic);
    gb.mem.write_u8(address, result);
}

/// Performs an arithmetic shift right of the value of a register.
fn srl_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = shift_right_u8v(gb, value, ShiftOp::ShiftLogical);
    gb.cpu.set_r8(r8, result);
}

/// Performs an arithmetic shift right of the value on a memory location.
fn srl_r16ptr(gb: &mut GameBoy, r16ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16ptr);
    let value   = gb.mem.read_u8(address);
    let result  = shift_right_u8v(gb, value, ShiftOp::ShiftLogical);
    gb.mem.write_u8(address, result);
}

/// Rotates the value of a register to the right through the carry flag.
fn rr_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = shift_right_u8v(gb, value, ShiftOp::RotateThroughCarry);
    gb.cpu.set_r8(r8, result);
}

/// Rotates the value on a memory location to the right through the carry flag.
fn rr_r16ptr(gb: &mut GameBoy, r16ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16ptr);
    let value   = gb.mem.read_u8(address);
    let result  = shift_right_u8v(gb, value, ShiftOp::RotateThroughCarry);
    gb.mem.write_u8(address, result);
}

/// Rotates the value of a register to the right.
fn rrc_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = shift_right_u8v(gb, value, ShiftOp::Rotate);
    gb.cpu.set_r8(r8, result);
}

/// Rotates the value on a memory location to the right.
fn rrc_r16ptr(gb: &mut GameBoy, r16ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16ptr);
    let value   = gb.mem.read_u8(address);
    let result  = shift_right_u8v(gb, value, ShiftOp::Rotate);
    gb.mem.write_u8(address, result);
}


pub fn sra_a(gb: &mut GameBoy) {
    sra_r8(gb, RegisterR8::A);
}

pub fn sra_b(gb: &mut GameBoy) {
    sra_r8(gb, RegisterR8::B);
}

pub fn sra_c(gb: &mut GameBoy) {
    sra_r8(gb, RegisterR8::C);
}

pub fn sra_d(gb: &mut GameBoy) {
    sra_r8(gb, RegisterR8::D);
}

pub fn sra_e(gb: &mut GameBoy) {
    sra_r8(gb, RegisterR8::E);
}

pub fn sra_h(gb: &mut GameBoy) {
    sra_r8(gb, RegisterR8::H);
}

pub fn sra_l(gb: &mut GameBoy) {
    sra_r8(gb, RegisterR8::L);
}

pub fn sra_hlptr(gb: &mut GameBoy) {
    sra_r16ptr(gb, RegisterR16::HL);
}

pub fn srl_a(gb: &mut GameBoy) {
    srl_r8(gb, RegisterR8::A);
}

pub fn srl_b(gb: &mut GameBoy) {
    srl_r8(gb, RegisterR8::B);
}

pub fn srl_c(gb: &mut GameBoy) {
    srl_r8(gb, RegisterR8::C);
}

pub fn srl_d(gb: &mut GameBoy) {
    srl_r8(gb, RegisterR8::D);
}

pub fn srl_e(gb: &mut GameBoy) {
    srl_r8(gb, RegisterR8::E);
}

pub fn srl_h(gb: &mut GameBoy) {
    srl_r8(gb, RegisterR8::H);
}

pub fn srl_l(gb: &mut GameBoy) {
    srl_r8(gb, RegisterR8::L);
}

pub fn srl_hlptr(gb: &mut GameBoy) {
    srl_r16ptr(gb, RegisterR16::HL);
}

pub fn rra(gb: &mut GameBoy) {
    rr_r8(gb, RegisterR8::A);
}

pub fn rr_a(gb: &mut GameBoy) {
    rr_r8(gb, RegisterR8::A);
}

pub fn rr_b(gb: &mut GameBoy) {
    rr_r8(gb, RegisterR8::B);
}

pub fn rr_c(gb: &mut GameBoy) {
    rr_r8(gb, RegisterR8::C);
}

pub fn rr_d(gb: &mut GameBoy) {
    rr_r8(gb, RegisterR8::D);
}

pub fn rr_e(gb: &mut GameBoy) {
    rr_r8(gb, RegisterR8::E);
}

pub fn rr_h(gb: &mut GameBoy) {
    rr_r8(gb, RegisterR8::H);
}

pub fn rr_l(gb: &mut GameBoy) {
    rr_r8(gb, RegisterR8::L);
}

pub fn rr_hlptr(gb: &mut GameBoy) {
    rr_r16ptr(gb, RegisterR16::HL);
}

pub fn rrca(gb: &mut GameBoy) {
    rrc_r8(gb, RegisterR8::A);
}

pub fn rrc_a(gb: &mut GameBoy) {
    rrc_r8(gb, RegisterR8::A);
}

pub fn rrc_b(gb: &mut GameBoy) {
    rrc_r8(gb, RegisterR8::B);
}

pub fn rrc_c(gb: &mut GameBoy) {
    rrc_r8(gb, RegisterR8::C);
}

pub fn rrc_d(gb: &mut GameBoy) {
    rrc_r8(gb, RegisterR8::D);
}

pub fn rrc_e(gb: &mut GameBoy) {
    rrc_r8(gb, RegisterR8::E);
}

pub fn rrc_h(gb: &mut GameBoy) {
    rrc_r8(gb, RegisterR8::H);
}

pub fn rrc_l(gb: &mut GameBoy) {
    rrc_r8(gb, RegisterR8::L);
}

pub fn rrc_hlptr(gb: &mut GameBoy) {
    rrc_r16ptr(gb, RegisterR16::HL);
}


////////////////////////////////////////////////
//// SWAP opcodes

/// Swaps the low and high nibble of a byte.
fn swap_nibbles_u8v(gb: &mut GameBoy, value: u8) -> u8 {
    let low   = (value >> 0) & 0x0f;
    let high  = (value >> 4) & 0x0f;
    let result = (low << 4) | (high);

    gb.cpu.clear_flags();
    gb.cpu.set_flag(CpuFlag::Zero, result != 0);

    result
}

/// Swaps the low and high nibble of a 8bit register.
fn swap_r8(gb: &mut GameBoy, r8: RegisterR8) {
    let value  = gb.cpu.get_r8(r8);
    let result = swap_nibbles_u8v(gb, value);
    gb.cpu.set_r8(r8, result);
}

/// Swaps the low and high nibble of a byte at the address of a 16bit register pointer.
fn swap_r16ptr(gb: &mut GameBoy, r16_ptr: RegisterR16) {
    let address = gb.cpu.get_r16(r16_ptr);
    let value   = gb.mem.read_u8(address);
    let result  = swap_nibbles_u8v(gb, value);
    gb.mem.write_u8(address, result);
}


pub fn swap_a(gb: &mut GameBoy) {
    swap_r8(gb, RegisterR8::A);
}

pub fn swap_b(gb: &mut GameBoy) {
    swap_r8(gb, RegisterR8::B);
}

pub fn swap_c(gb: &mut GameBoy) {
    swap_r8(gb, RegisterR8::C);
}

pub fn swap_d(gb: &mut GameBoy) {
    swap_r8(gb, RegisterR8::D);
}

pub fn swap_e(gb: &mut GameBoy) {
    swap_r8(gb, RegisterR8::E);
}

pub fn swap_h(gb: &mut GameBoy) {
    swap_r8(gb, RegisterR8::H);
}

pub fn swap_l(gb: &mut GameBoy) {
    swap_r8(gb, RegisterR8::L);
}

pub fn swap_hlptr(gb: &mut GameBoy) {
    swap_r16ptr(gb, RegisterR16::HL);
}


////////////////////////////////////////////////
//// Set Bit opcodes

/// Set bit n of a given 8bit value.
/// value | (1 << bit)
fn set_bit_u8v(_gb: &mut GameBoy, value: u8, bit: u8) -> u8 {
    let result = value | (1 << bit);
    result
}

/// Set bit n of in the given register.
/// r8 <- r8 | (1 << bit)
fn set_bit_r8(gb: &mut GameBoy, r8: RegisterR8, bit: u8) {
    let value  = gb.cpu.get_r8(r8);
    let result = set_bit_u8v(gb, value, bit);
    gb.cpu.set_r8(r8, result);
}

/// Set bit n on a memory address.
/// (r16) <- (r16) | (1 << bit)
fn set_bit_r16ptr(gb: &mut GameBoy, r16_ptr: RegisterR16, bit: u8) {
    let address = gb.cpu.get_r16(r16_ptr);
    let value   = gb.mem.read_u8(address);
    let result  = set_bit_u8v(gb, value, bit);
    gb.mem.write_u8(address, result);
}


pub fn set_bit_0_a(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::A, 0);
}

pub fn set_bit_0_b(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::B, 0);
}

pub fn set_bit_0_c(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::C, 0);
}

pub fn set_bit_0_d(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::D, 0);
}

pub fn set_bit_0_e(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::E, 0);
}

pub fn set_bit_0_h(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::H, 0);
}

pub fn set_bit_0_l(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::L, 0);
}

pub fn set_bit_0_hlptr(gb: &mut GameBoy) {
    set_bit_r16ptr(gb, RegisterR16::HL, 0);
}

pub fn set_bit_1_a(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::A, 1);
}

pub fn set_bit_1_b(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::B, 1);
}

pub fn set_bit_1_c(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::C, 1);
}

pub fn set_bit_1_d(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::D, 1);
}

pub fn set_bit_1_e(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::E, 1);
}

pub fn set_bit_1_h(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::H, 1);
}

pub fn set_bit_1_l(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::L, 1);
}

pub fn set_bit_1_hlptr(gb: &mut GameBoy) {
    set_bit_r16ptr(gb, RegisterR16::HL, 1);
}

pub fn set_bit_2_a(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::A, 2);
}

pub fn set_bit_2_b(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::B, 2);
}

pub fn set_bit_2_c(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::C, 2);
}

pub fn set_bit_2_d(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::D, 2);
}

pub fn set_bit_2_e(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::E, 2);
}

pub fn set_bit_2_h(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::H, 2);
}

pub fn set_bit_2_l(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::L, 2);
}

pub fn set_bit_2_hlptr(gb: &mut GameBoy) {
    set_bit_r16ptr(gb, RegisterR16::HL, 2);
}

pub fn set_bit_3_a(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::A, 3);
}

pub fn set_bit_3_b(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::B, 3);
}

pub fn set_bit_3_c(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::C, 3);
}

pub fn set_bit_3_d(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::D, 3);
}

pub fn set_bit_3_e(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::E, 3);
}

pub fn set_bit_3_h(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::H, 3);
}

pub fn set_bit_3_l(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::L, 3);
}

pub fn set_bit_3_hlptr(gb: &mut GameBoy) {
    set_bit_r16ptr(gb, RegisterR16::HL, 3);
}

pub fn set_bit_4_a(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::A, 4);
}

pub fn set_bit_4_b(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::B, 4);
}

pub fn set_bit_4_c(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::C, 4);
}

pub fn set_bit_4_d(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::D, 4);
}

pub fn set_bit_4_e(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::E, 4);
}

pub fn set_bit_4_h(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::H, 4);
}

pub fn set_bit_4_l(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::L, 4);
}

pub fn set_bit_4_hlptr(gb: &mut GameBoy) {
    set_bit_r16ptr(gb, RegisterR16::HL, 4);
}

pub fn set_bit_5_a(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::A, 5);
}

pub fn set_bit_5_b(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::B, 5);
}

pub fn set_bit_5_c(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::C, 5);
}

pub fn set_bit_5_d(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::D, 5);
}

pub fn set_bit_5_e(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::E, 5);
}

pub fn set_bit_5_h(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::H, 5);
}

pub fn set_bit_5_l(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::L, 5);
}

pub fn set_bit_5_hlptr(gb: &mut GameBoy) {
    set_bit_r16ptr(gb, RegisterR16::HL, 5);
}

pub fn set_bit_6_a(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::A, 6);
}

pub fn set_bit_6_b(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::B, 6);
}

pub fn set_bit_6_c(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::C, 6);
}

pub fn set_bit_6_d(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::D, 6);
}

pub fn set_bit_6_e(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::E, 6);
}

pub fn set_bit_6_h(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::H, 6);
}

pub fn set_bit_6_l(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::L, 6);
}

pub fn set_bit_6_hlptr(gb: &mut GameBoy) {
    set_bit_r16ptr(gb, RegisterR16::HL, 6);
}

pub fn set_bit_7_a(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::A, 7);
}

pub fn set_bit_7_b(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::B, 7);
}

pub fn set_bit_7_c(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::C, 7);
}

pub fn set_bit_7_d(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::D, 7);
}

pub fn set_bit_7_e(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::E, 7);
}

pub fn set_bit_7_h(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::H, 7);
}

pub fn set_bit_7_l(gb: &mut GameBoy) {
    set_bit_r8(gb, RegisterR8::L, 7);
}

pub fn set_bit_7_hlptr(gb: &mut GameBoy) {
    set_bit_r16ptr(gb, RegisterR16::HL, 7);
}


////////////////////////////////////////////////
//// Reset Bit opcodes

/// Resets bit n of a given 8bit value.
/// value & !(1 << bit)
fn res_bit_u8v(_gb: &mut GameBoy, value: u8, bit: u8) -> u8 {
    let result = value & !(1 << bit);
    result
}

/// Resets bit n of in the given register.
/// r8 <- r8 & !(1 << bit)
fn res_bit_r8(gb: &mut GameBoy, r8: RegisterR8, bit: u8) {
    let value  = gb.cpu.get_r8(r8);
    let result = res_bit_u8v(gb, value, bit);
    gb.cpu.set_r8(r8, result);
}

/// Resets bit n on a memory address.
/// (r16) <- (r16) & !(1 << bit)
fn res_bit_r16ptr(gb: &mut GameBoy, r16_ptr: RegisterR16, bit: u8) {
    let address = gb.cpu.get_r16(r16_ptr);
    let value   = gb.mem.read_u8(address);
    let result  = res_bit_u8v(gb, value, bit);
    gb.mem.write_u8(address, result);
}


pub fn res_bit_0_a(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::A, 0);
}

pub fn res_bit_0_b(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::B, 0);
}

pub fn res_bit_0_c(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::C, 0);
}

pub fn res_bit_0_d(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::D, 0);
}

pub fn res_bit_0_e(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::E, 0);
}

pub fn res_bit_0_h(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::H, 0);
}

pub fn res_bit_0_l(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::L, 0);
}

pub fn res_bit_0_hlptr(gb: &mut GameBoy) {
    res_bit_r16ptr(gb, RegisterR16::HL, 0);
}

pub fn res_bit_1_a(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::A, 1);
}

pub fn res_bit_1_b(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::B, 1);
}

pub fn res_bit_1_c(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::C, 1);
}

pub fn res_bit_1_d(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::D, 1);
}

pub fn res_bit_1_e(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::E, 1);
}

pub fn res_bit_1_h(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::H, 1);
}

pub fn res_bit_1_l(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::L, 1);
}

pub fn res_bit_1_hlptr(gb: &mut GameBoy) {
    res_bit_r16ptr(gb, RegisterR16::HL, 1);
}

pub fn res_bit_2_a(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::A, 2);
}

pub fn res_bit_2_b(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::B, 2);
}

pub fn res_bit_2_c(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::C, 2);
}

pub fn res_bit_2_d(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::D, 2);
}

pub fn res_bit_2_e(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::E, 2);
}

pub fn res_bit_2_h(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::H, 2);
}

pub fn res_bit_2_l(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::L, 2);
}

pub fn res_bit_2_hlptr(gb: &mut GameBoy) {
    res_bit_r16ptr(gb, RegisterR16::HL, 2);
}

pub fn res_bit_3_a(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::A, 3);
}

pub fn res_bit_3_b(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::B, 3);
}

pub fn res_bit_3_c(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::C, 3);
}

pub fn res_bit_3_d(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::D, 3);
}

pub fn res_bit_3_e(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::E, 3);
}

pub fn res_bit_3_h(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::H, 3);
}

pub fn res_bit_3_l(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::L, 3);
}

pub fn res_bit_3_hlptr(gb: &mut GameBoy) {
    res_bit_r16ptr(gb, RegisterR16::HL, 3);
}

pub fn res_bit_4_a(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::A, 4);
}

pub fn res_bit_4_b(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::B, 4);
}

pub fn res_bit_4_c(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::C, 4);
}

pub fn res_bit_4_d(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::D, 4);
}

pub fn res_bit_4_e(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::E, 4);
}

pub fn res_bit_4_h(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::H, 4);
}

pub fn res_bit_4_l(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::L, 4);
}

pub fn res_bit_4_hlptr(gb: &mut GameBoy) {
    res_bit_r16ptr(gb, RegisterR16::HL, 4);
}

pub fn res_bit_5_a(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::A, 5);
}

pub fn res_bit_5_b(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::B, 5);
}

pub fn res_bit_5_c(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::C, 5);
}

pub fn res_bit_5_d(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::D, 5);
}

pub fn res_bit_5_e(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::E, 5);
}

pub fn res_bit_5_h(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::H, 5);
}

pub fn res_bit_5_l(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::L, 5);
}

pub fn res_bit_5_hlptr(gb: &mut GameBoy) {
    res_bit_r16ptr(gb, RegisterR16::HL, 5);
}

pub fn res_bit_6_a(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::A, 6);
}

pub fn res_bit_6_b(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::B, 6);
}

pub fn res_bit_6_c(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::C, 6);
}

pub fn res_bit_6_d(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::D, 6);
}

pub fn res_bit_6_e(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::E, 6);
}

pub fn res_bit_6_h(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::H, 6);
}

pub fn res_bit_6_l(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::L, 6);
}

pub fn res_bit_6_hlptr(gb: &mut GameBoy) {
    res_bit_r16ptr(gb, RegisterR16::HL, 6);
}

pub fn res_bit_7_a(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::A, 7);
}

pub fn res_bit_7_b(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::B, 7);
}

pub fn res_bit_7_c(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::C, 7);
}

pub fn res_bit_7_d(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::D, 7);
}

pub fn res_bit_7_e(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::E, 7);
}

pub fn res_bit_7_h(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::H, 7);
}

pub fn res_bit_7_l(gb: &mut GameBoy) {
    res_bit_r8(gb, RegisterR8::L, 7);
}

pub fn res_bit_7_hlptr(gb: &mut GameBoy) {
    res_bit_r16ptr(gb, RegisterR16::HL, 7);
}


////////////////////////////////////////////////
//// Check Bit opcodes


/// Checks if bit n of a value is set.
/// Set the Zero flag, if the bit was 0.
fn check_bit_u8v(gb: &mut GameBoy, value: u8, bit: u8) {
    let result = value & (1 << bit);
    gb.cpu.set_flag(CpuFlag::Zero,      result == 0);
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, true);
}

/// Checks if bit n of a register is set.
/// Set the Zero flag, if the bit was 0.
fn check_bit_r8(gb: &mut GameBoy, r8: RegisterR8, bit: u8) {
    let value  = gb.cpu.get_r8(r8);
    check_bit_u8v(gb, value, bit);
}

/// Checks if bit n on a memory address is set.
/// Set the Zero flag, if the bit was 0.
fn check_bit_r16ptr(gb: &mut GameBoy, r16_ptr: RegisterR16, bit: u8) {
    let address = gb.cpu.get_r16(r16_ptr);
    let value   = gb.mem.read_u8(address);
    check_bit_u8v(gb, value, bit);
}


pub fn check_bit_0_a(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::A, 0);
}

pub fn check_bit_0_b(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::B, 0);
}

pub fn check_bit_0_c(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::C, 0);
}

pub fn check_bit_0_d(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::D, 0);
}

pub fn check_bit_0_e(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::E, 0);
}

pub fn check_bit_0_h(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::H, 0);
}

pub fn check_bit_0_l(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::L, 0);
}

pub fn check_bit_0_hlptr(gb: &mut GameBoy) {
    check_bit_r16ptr(gb, RegisterR16::HL, 0);
}

pub fn check_bit_1_a(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::A, 1);
}

pub fn check_bit_1_b(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::B, 1);
}

pub fn check_bit_1_c(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::C, 1);
}

pub fn check_bit_1_d(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::D, 1);
}

pub fn check_bit_1_e(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::E, 1);
}

pub fn check_bit_1_h(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::H, 1);
}

pub fn check_bit_1_l(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::L, 1);
}

pub fn check_bit_1_hlptr(gb: &mut GameBoy) {
    check_bit_r16ptr(gb, RegisterR16::HL, 1);
}

pub fn check_bit_2_a(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::A, 2);
}

pub fn check_bit_2_b(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::B, 2);
}

pub fn check_bit_2_c(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::C, 2);
}

pub fn check_bit_2_d(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::D, 2);
}

pub fn check_bit_2_e(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::E, 2);
}

pub fn check_bit_2_h(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::H, 2);
}

pub fn check_bit_2_l(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::L, 2);
}

pub fn check_bit_2_hlptr(gb: &mut GameBoy) {
    check_bit_r16ptr(gb, RegisterR16::HL, 2);
}

pub fn check_bit_3_a(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::A, 3);
}

pub fn check_bit_3_b(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::B, 3);
}

pub fn check_bit_3_c(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::C, 3);
}

pub fn check_bit_3_d(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::D, 3);
}

pub fn check_bit_3_e(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::E, 3);
}

pub fn check_bit_3_h(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::H, 3);
}

pub fn check_bit_3_l(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::L, 3);
}

pub fn check_bit_3_hlptr(gb: &mut GameBoy) {
    check_bit_r16ptr(gb, RegisterR16::HL, 3);
}

pub fn check_bit_4_a(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::A, 4);
}

pub fn check_bit_4_b(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::B, 4);
}

pub fn check_bit_4_c(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::C, 4);
}

pub fn check_bit_4_d(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::D, 4);
}

pub fn check_bit_4_e(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::E, 4);
}

pub fn check_bit_4_h(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::H, 4);
}

pub fn check_bit_4_l(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::L, 4);
}

pub fn check_bit_4_hlptr(gb: &mut GameBoy) {
    check_bit_r16ptr(gb, RegisterR16::HL, 4);
}

pub fn check_bit_5_a(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::A, 5);
}

pub fn check_bit_5_b(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::B, 5);
}

pub fn check_bit_5_c(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::C, 5);
}

pub fn check_bit_5_d(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::D, 5);
}

pub fn check_bit_5_e(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::E, 5);
}

pub fn check_bit_5_h(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::H, 5);
}

pub fn check_bit_5_l(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::L, 5);
}

pub fn check_bit_5_hlptr(gb: &mut GameBoy) {
    check_bit_r16ptr(gb, RegisterR16::HL, 5);
}

pub fn check_bit_6_a(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::A, 6);
}

pub fn check_bit_6_b(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::B, 6);
}

pub fn check_bit_6_c(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::C, 6);
}

pub fn check_bit_6_d(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::D, 6);
}

pub fn check_bit_6_e(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::E, 6);
}

pub fn check_bit_6_h(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::H, 6);
}

pub fn check_bit_6_l(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::L, 6);
}

pub fn check_bit_6_hlptr(gb: &mut GameBoy) {
    check_bit_r16ptr(gb, RegisterR16::HL, 6);
}

pub fn check_bit_7_a(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::A, 7);
}

pub fn check_bit_7_b(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::B, 7);
}

pub fn check_bit_7_c(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::C, 7);
}

pub fn check_bit_7_d(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::D, 7);
}

pub fn check_bit_7_e(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::E, 7);
}

pub fn check_bit_7_h(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::H, 7);
}

pub fn check_bit_7_l(gb: &mut GameBoy) {
    check_bit_r8(gb, RegisterR8::L, 7);
}

pub fn check_bit_7_hlptr(gb: &mut GameBoy) {
    check_bit_r16ptr(gb, RegisterR16::HL, 7);
}


////////////////////////////////////////////////
//// CP opcodes

/// Compares two values.
fn cp_u8v_u8v(gb: &mut GameBoy, value1: u8, value2: u8) {
    let result = (value1 as i32) - (value2 as i32);
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
/// dst <- dst & u8
fn and_r8_u8(gb: &mut GameBoy, dst: RegisterR8) {
    let value = gb.cpu.fetch_u8();
    and_r8_u8v(gb, dst, value);
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


pub fn and_a_u8(gb: &mut GameBoy) {
    and_r8_u8(gb, RegisterR8::A);
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
/// dst <- dst | u8
fn or_r8_u8(gb: &mut GameBoy, dst: RegisterR8) {
    let value = gb.cpu.fetch_u8();
    or_r8_u8v(gb, dst, value);
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


pub fn or_a_u8(gb: &mut GameBoy) {
    or_r8_u8(gb, RegisterR8::A);
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

