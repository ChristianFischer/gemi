/*
 * Copyright (C) 2022-2025 by Christian Fischer
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

use crate::cpu::cpu::{CpuFlag, RegisterR16, RegisterR8};
use crate::cpu::opcode::{opcode, OpCodeContext, OpCodeResult};
use crate::utils::{carrying_add_u16, carrying_add_u8, carrying_sub_u8};


////////////////////////////////////////////////
//// Flag types

enum ShiftOp {
    ShiftLogical,
    ShiftArithmetic,
    Rotate,
    RotateThroughCarry,
}

enum NullCheck {
    Check,
    ClearFlag
}


////////////////////////////////////////////////
//// INC opcodes
pub mod inc {
    use super::*;

    /// Increments a 8bit value.
    fn increment_u8v(ctx: &mut OpCodeContext, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        ctx.dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, false);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, (result & 0x0f) == 0);

        result
    }

    /// Increments a 16bit value.
    fn increment_u16v(_ctx: &mut OpCodeContext, value: u16) -> u16 {
        let result = value.wrapping_add(1);
        result
    }

    /// Increments a value
    /// r8 <- r8 + 1
    fn increment_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = increment_u8v(ctx, value);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Increments a value
    /// r16 <- r16 + 1
    fn increment_r16(ctx: &mut OpCodeContext, r16: RegisterR16) {
        let value  = ctx.dev.cpu.get_r16(r16);
        let result = increment_u16v(ctx, value);
        ctx.dev.cpu.set_r16(r16, result);
    }

    /// Increments a value.
    /// (r16) <- (r16) + 1
    fn increment_r16ptr(ctx: &mut OpCodeContext, r16_ptr: RegisterR16) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
                ctx.dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value   = ctx.dev.cpu.get_intermediate_value();
                let result  = increment_u8v(ctx, value);
                ctx.dev.get_mmu_mut().write_u8(ctx.ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    // INC r8
    opcode!(inc_a, [ctx] increment_r8(ctx, RegisterR8::A));
    opcode!(inc_b, [ctx] increment_r8(ctx, RegisterR8::B));
    opcode!(inc_c, [ctx] increment_r8(ctx, RegisterR8::C));
    opcode!(inc_d, [ctx] increment_r8(ctx, RegisterR8::D));
    opcode!(inc_e, [ctx] increment_r8(ctx, RegisterR8::E));
    opcode!(inc_h, [ctx] increment_r8(ctx, RegisterR8::H));
    opcode!(inc_l, [ctx] increment_r8(ctx, RegisterR8::L));

    // INC r16
    opcode!(inc_bc, [ctx] increment_r16(ctx, RegisterR16::BC));
    opcode!(inc_de, [ctx] increment_r16(ctx, RegisterR16::DE));
    opcode!(inc_hl, [ctx] increment_r16(ctx, RegisterR16::HL));

    // INC (r16)
    opcode!(inc_hlptr, [ctx] increment_r16ptr(ctx, RegisterR16::HL));

    // INC SP
    opcode!(inc_sp, [ctx] {
        let sp_old = ctx.dev.cpu.get_stack_pointer();
        let sp_new = sp_old.wrapping_add(1);
        ctx.dev.cpu.set_stack_pointer(sp_new);
    });
}


////////////////////////////////////////////////
//// DEC opcodes
pub mod dec {
    use super::*;

    /// Decrements a 8bit value.
    fn decrement_u8v(ctx: &mut OpCodeContext, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
    
        ctx.dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, true);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, (result & 0x0f) == 0x0f);
    
        result
    }
    
    /// Decrements a 16bit value.
    fn decrement_u16v(_ctx: &mut OpCodeContext, value: u16) -> u16 {
        let result = value.wrapping_sub(1);
        result
    }
    
    /// Decrements a value
    /// r8 <- r8 - 1
    fn decrement_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = decrement_u8v(ctx, value);
        ctx.dev.cpu.set_r8(r8, result);
    }
    
    /// Decrements a value
    /// r16 <- r16 - 1
    fn decrement_r16(ctx: &mut OpCodeContext, r16: RegisterR16) {
        let value  = ctx.dev.cpu.get_r16(r16);
        let result = decrement_u16v(ctx, value);
        ctx.dev.cpu.set_r16(r16, result);
    }
    
    /// Decrements a value.
    /// (r16) <- (r16) - 1
    fn decrement_r16ptr(ctx: &mut OpCodeContext, r16_ptr: RegisterR16) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
                ctx.dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            },

            1 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value = ctx.dev.cpu.get_intermediate_value();
                let result  = decrement_u8v(ctx, value);
                ctx.dev.get_mmu_mut().write_u8(ctx.ec, address, result);

                OpCodeResult::Done
            },

            _ => unreachable!()
        }
    }
    

    // DEC r8
    opcode!(dec_a, [ctx] decrement_r8(ctx, RegisterR8::A));
    opcode!(dec_b, [ctx] decrement_r8(ctx, RegisterR8::B));
    opcode!(dec_c, [ctx] decrement_r8(ctx, RegisterR8::C));
    opcode!(dec_d, [ctx] decrement_r8(ctx, RegisterR8::D));
    opcode!(dec_e, [ctx] decrement_r8(ctx, RegisterR8::E));
    opcode!(dec_h, [ctx] decrement_r8(ctx, RegisterR8::H));
    opcode!(dec_l, [ctx] decrement_r8(ctx, RegisterR8::L));

    // DEC r16
    opcode!(dec_bc, [ctx] decrement_r16(ctx, RegisterR16::BC));
    opcode!(dec_de, [ctx] decrement_r16(ctx, RegisterR16::DE));
    opcode!(dec_hl, [ctx] decrement_r16(ctx, RegisterR16::HL));

    // DEC (r16)
    opcode!(dec_hlptr, [ctx] decrement_r16ptr(ctx, RegisterR16::HL));

    // DEC SP
    opcode!(dec_sp, [ctx] {
        let sp_old = ctx.dev.cpu.get_stack_pointer();
        let sp_new = sp_old.wrapping_sub(1);
        ctx.dev.cpu.set_stack_pointer(sp_new);
    });
}


////////////////////////////////////////////////
//// ADD / ADC opcodes
pub mod add {
    use super::*;

    /// Adds two values and stores it into a 8bit register.
    /// r8 <- r8 + value + (carry flag, if add_carry)
    fn add_r8_u8v(ctx: &mut OpCodeContext, r8: RegisterR8, value: u8, add_carry: bool) {
        let current_carry = add_carry && ctx.dev.cpu.is_flag_set(CpuFlag::Carry);
        let current_value = ctx.dev.cpu.get_r8(r8);
        let (result, half_carry, carry) = carrying_add_u8(current_value, value, current_carry);

        ctx.dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, false);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        ctx.dev.cpu.set_flag(CpuFlag::Carry, carry);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + u8 + (carry flag, if add_carry)
    fn add_r8_u8(ctx: &mut OpCodeContext, dst: RegisterR8, add_carry: bool) {
        let value = ctx.dev.cpu.fetch_u8(ctx.ec);
        add_r8_u8v(ctx, dst, value, add_carry);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + src + (carry flag, if add_carry)
    fn add_r8_r8(ctx: &mut OpCodeContext, dst: RegisterR8, src: RegisterR8, add_carry: bool) {
        let value = ctx.dev.cpu.get_r8(src);
        add_r8_u8v(ctx, dst, value, add_carry);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + (src_ptr) + (carry flag, if add_carry)
    fn add_r8_r16ptr(ctx: &mut OpCodeContext, dst: RegisterR8, src_ptr: RegisterR16, add_carry: bool) {
        let address = ctx.dev.cpu.get_r16(src_ptr);
        let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
        add_r8_u8v(ctx, dst, value, add_carry);
    }

    /// Adds two values and stores it into a 16bit register.
    /// r16 <- r16 + value
    fn add_r16_u16v(ctx: &mut OpCodeContext, r16: RegisterR16, value: u16) {
        let current_value = ctx.dev.cpu.get_r16(r16);
        let (result, half_carry, carry) = carrying_add_u16(current_value, value, false);

        ctx.dev.cpu.set_flag(CpuFlag::Negative, false);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        ctx.dev.cpu.set_flag(CpuFlag::Carry, carry);
        ctx.dev.cpu.set_r16(r16, result);
    }

    /// Adds two values and stores it into a 16bit register.
    /// dst <- dst + src
    fn add_r16_r16(ctx: &mut OpCodeContext, dst: RegisterR16, src: RegisterR16) {
        let value = ctx.dev.cpu.get_r16(src);
        add_r16_u16v(ctx, dst, value);
    }


    // ADD r8, ?
    opcode!(add_a_u8,    [ctx] add_r8_u8(ctx, RegisterR8::A, false));
    opcode!(add_a_a,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::A, false));
    opcode!(add_a_b,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::B, false));
    opcode!(add_a_c,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::C, false));
    opcode!(add_a_d,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::D, false));
    opcode!(add_a_e,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::E, false));
    opcode!(add_a_h,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::H, false));
    opcode!(add_a_l,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::L, false));
    opcode!(add_a_hlptr, [ctx] add_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::HL, false));

    // add with carry flag
    // ADC r8, ?
    opcode!(adc_a_u8,    [ctx] add_r8_u8(ctx, RegisterR8::A, true));
    opcode!(adc_a_a,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::A, true));
    opcode!(adc_a_b,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::B, true));
    opcode!(adc_a_c,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::C, true));
    opcode!(adc_a_d,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::D, true));
    opcode!(adc_a_e,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::E, true));
    opcode!(adc_a_h,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::H, true));
    opcode!(adc_a_l,     [ctx] add_r8_r8(ctx, RegisterR8::A, RegisterR8::L, true));
    opcode!(adc_a_hlptr, [ctx] add_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::HL, true));

    // ADD r16, r16
    opcode!(add_hl_bc, [ctx] add_r16_r16(ctx, RegisterR16::HL, RegisterR16::BC));
    opcode!(add_hl_de, [ctx] add_r16_r16(ctx, RegisterR16::HL, RegisterR16::DE));
    opcode!(add_hl_hl, [ctx] add_r16_r16(ctx, RegisterR16::HL, RegisterR16::HL));
    opcode!(add_hl_sp, [ctx] add_r16_u16v(ctx, RegisterR16::HL, ctx.dev.cpu.get_stack_pointer()));
}


////////////////////////////////////////////////
//// SUB / SBC opcodes
pub mod sub {
    use super::*;

    /// Subtracts a value from another one and stores the result into a 8bit register.
    /// r8 <- r8 - value - (carry flag, if sub_carry)
    fn sub_r8_u8v(ctx: &mut OpCodeContext, r8: RegisterR8, value: u8, sub_carry: bool) {
        let current_carry = sub_carry && ctx.dev.cpu.is_flag_set(CpuFlag::Carry);
        let current_value = ctx.dev.cpu.get_r8(r8);
        let (result, half_carry, carry) = carrying_sub_u8(current_value, value, current_carry);

        ctx.dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, true);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        ctx.dev.cpu.set_flag(CpuFlag::Carry, carry);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + u8 + (carry flag, if add_carry)
    fn sub_r8_u8(ctx: &mut OpCodeContext, dst: RegisterR8, add_carry: bool) {
        let value = ctx.dev.cpu.fetch_u8(ctx.ec);
        sub_r8_u8v(ctx, dst, value, add_carry);
    }

    /// Subtracts a value from another one and stores the result into a 8bit register.
    /// dst <- dst - src - (carry flag, if sub_carry)
    fn sub_r8_r8(ctx: &mut OpCodeContext, dst: RegisterR8, src: RegisterR8, sub_carry: bool) {
        let value = ctx.dev.cpu.get_r8(src);
        sub_r8_u8v(ctx, dst, value, sub_carry);
    }

    /// Subtracts a value from another one and stores the result into a 8bit register.
    /// dst <- dst - (src_ptr) - (carry flag, if sub_carry)
    fn sub_r8_r16ptr(ctx: &mut OpCodeContext, dst: RegisterR8, src_ptr: RegisterR16, sub_carry: bool) {
        let address = ctx.dev.cpu.get_r16(src_ptr);
        let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
        sub_r8_u8v(ctx, dst, value, sub_carry);
    }


    // SUB r8, ?
    opcode!(sub_a_u8,    [ctx] sub_r8_u8(ctx, RegisterR8::A, false));
    opcode!(sub_a_a,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::A, false));
    opcode!(sub_a_b,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::B, false));
    opcode!(sub_a_c,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::C, false));
    opcode!(sub_a_d,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::D, false));
    opcode!(sub_a_e,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::E, false));
    opcode!(sub_a_h,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::H, false));
    opcode!(sub_a_l,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::L, false));
    opcode!(sub_a_hlptr, [ctx] sub_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::HL, false));

    // subtract with carry flag
    // SUB r8, ?
    opcode!(sbc_a_u8,    [ctx] sub_r8_u8(ctx, RegisterR8::A, true));
    opcode!(sbc_a_a,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::A, true));
    opcode!(sbc_a_b,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::B, true));
    opcode!(sbc_a_c,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::C, true));
    opcode!(sbc_a_d,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::D, true));
    opcode!(sbc_a_e,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::E, true));
    opcode!(sbc_a_h,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::H, true));
    opcode!(sbc_a_l,     [ctx] sub_r8_r8(ctx, RegisterR8::A, RegisterR8::L, true));
    opcode!(sbc_a_hlptr, [ctx] sub_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::HL, true));
}


////////////////////////////////////////////////
//// RL / RLC opcodes
pub mod rl {
    use super::*;

    /// Shifts or rotates a value to the left.
    fn shift_left_u8v(ctx: &mut OpCodeContext, value: u8, op: ShiftOp) -> u8 {
        shift_left_u8v_nc(ctx, value, op, NullCheck::Check)
    }

    /// Shifts or rotates a value to the left.
    fn shift_left_u8v_nc(ctx: &mut OpCodeContext, value: u8, op: ShiftOp, nullcheck: NullCheck) -> u8 {
        let carry    = ctx.dev.cpu.is_flag_set(CpuFlag::Carry) as u8;
        let left_bit = (value >> 7) & 1;

        let result = match op {
            ShiftOp::ShiftLogical       => (value << 1) | 0x0000,
            ShiftOp::ShiftArithmetic    => (value << 1) | 0x0000,
            ShiftOp::Rotate             => (value << 1) | left_bit,
            ShiftOp::RotateThroughCarry => (value << 1) | carry,
        };

        let null_bit = match nullcheck {
            NullCheck::Check     => result == 0,
            NullCheck::ClearFlag => false,
        };

        ctx.dev.cpu.set_flag(CpuFlag::Zero, null_bit);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, false);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, false);
        ctx.dev.cpu.set_flag(CpuFlag::Carry, left_bit != 0);

        result
    }

    /// Shifts or rotates a value on a 16bit pointer to the left.
    fn shift_left_r16ptr(ctx: &mut OpCodeContext, r16ptr: RegisterR16, op: ShiftOp) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = ctx.dev.cpu.get_r16(r16ptr);
                let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
                ctx.dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = ctx.dev.cpu.get_r16(r16ptr);
                let value   = ctx.dev.cpu.get_intermediate_value();
                let result  = shift_left_u8v(ctx, value, op);
                ctx.dev.get_mmu_mut().write_u8(ctx.ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }

    /// Performs an arithmetic shift left of the value of a register.
    fn sla_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = shift_left_u8v(ctx, value, ShiftOp::ShiftArithmetic);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Performs an arithmetic shift left of the value on a memory location.
    fn sla_r16ptr(ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_left_r16ptr(ctx, r16ptr, ShiftOp::ShiftArithmetic)
    }

    /// Rotates the value of a register to the left through the carry flag.
    fn rl_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        rl_r8_nc(ctx, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the left through the carry flag.
    fn rl_r8_nc(ctx: &mut OpCodeContext, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = shift_left_u8v_nc(ctx, value, ShiftOp::RotateThroughCarry, nullcheck);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the left through the carry flag.
    fn rl_r16ptr(ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_left_r16ptr(ctx, r16ptr, ShiftOp::RotateThroughCarry)
    }

    /// Rotates the value of a register to the left.
    fn rlc_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        rlc_r8_nc(ctx, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the left.
    fn rlc_r8_nc(ctx: &mut OpCodeContext, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = shift_left_u8v_nc(ctx, value, ShiftOp::Rotate, nullcheck);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the left.
    fn rlc_r16ptr(ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_left_r16ptr(ctx, r16ptr, ShiftOp::Rotate)
    }


    // arithmetic shift left
    opcode!(sla_a,     [ctx] sla_r8(ctx, RegisterR8::A));
    opcode!(sla_b,     [ctx] sla_r8(ctx, RegisterR8::B));
    opcode!(sla_c,     [ctx] sla_r8(ctx, RegisterR8::C));
    opcode!(sla_d,     [ctx] sla_r8(ctx, RegisterR8::D));
    opcode!(sla_e,     [ctx] sla_r8(ctx, RegisterR8::E));
    opcode!(sla_h,     [ctx] sla_r8(ctx, RegisterR8::H));
    opcode!(sla_l,     [ctx] sla_r8(ctx, RegisterR8::L));

    // rotate left through carry flag
    opcode!(rla,       [ctx] rl_r8_nc(ctx, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rl_a,      [ctx] rl_r8(ctx, RegisterR8::A));
    opcode!(rl_b,      [ctx] rl_r8(ctx, RegisterR8::B));
    opcode!(rl_c,      [ctx] rl_r8(ctx, RegisterR8::C));
    opcode!(rl_d,      [ctx] rl_r8(ctx, RegisterR8::D));
    opcode!(rl_e,      [ctx] rl_r8(ctx, RegisterR8::E));
    opcode!(rl_h,      [ctx] rl_r8(ctx, RegisterR8::H));
    opcode!(rl_l,      [ctx] rl_r8(ctx, RegisterR8::L));

    // rotate left (carry flag just set)
    opcode!(rlca,      [ctx] rlc_r8_nc(ctx, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rlc_a,     [ctx] rlc_r8(ctx, RegisterR8::A));
    opcode!(rlc_b,     [ctx] rlc_r8(ctx, RegisterR8::B));
    opcode!(rlc_c,     [ctx] rlc_r8(ctx, RegisterR8::C));
    opcode!(rlc_d,     [ctx] rlc_r8(ctx, RegisterR8::D));
    opcode!(rlc_e,     [ctx] rlc_r8(ctx, RegisterR8::E));
    opcode!(rlc_h,     [ctx] rlc_r8(ctx, RegisterR8::H));
    opcode!(rlc_l,     [ctx] rlc_r8(ctx, RegisterR8::L));

    opcode!(sla_hlptr, [ctx] sla_r16ptr(ctx, RegisterR16::HL));
    opcode!(rl_hlptr,  [ctx] rl_r16ptr (ctx, RegisterR16::HL));
    opcode!(rlc_hlptr, [ctx] rlc_r16ptr(ctx, RegisterR16::HL));
}

////////////////////////////////////////////////
//// RR / RRC opcodes
pub mod rr {
    use super::*;

    /// Shifts or rotates a value to the right.
    fn shift_right_u8v(ctx: &mut OpCodeContext, value: u8, op: ShiftOp) -> u8 {
        shift_right_u8v_nc(ctx, value, op, NullCheck::Check)
    }

    /// Shifts or rotates a value to the right.
    fn shift_right_u8v_nc(ctx: &mut OpCodeContext, value: u8, op: ShiftOp, nullcheck: NullCheck) -> u8 {
        let carry    = ctx.dev.cpu.is_flag_set(CpuFlag::Carry) as u8;
        let left_bit = (value >> 7) & 1;
        let right_bit= value & 1;

        let result = match op {
            ShiftOp::ShiftLogical       => (value >> 1) | 0x0000,
            ShiftOp::ShiftArithmetic    => (value >> 1) | (left_bit << 7),
            ShiftOp::Rotate             => (value >> 1) | (right_bit << 7),
            ShiftOp::RotateThroughCarry => (value >> 1) | (carry << 7),
        };

        let null_bit = match nullcheck {
            NullCheck::Check     => result == 0,
            NullCheck::ClearFlag => false,
        };

        ctx.dev.cpu.set_flag(CpuFlag::Zero, null_bit);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, false);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, false);
        ctx.dev.cpu.set_flag(CpuFlag::Carry, right_bit != 0);

        result
    }

    /// Shifts or rotates a value on a 16bit pointer to the right.
    fn shift_right_r16ptr(ctx: &mut OpCodeContext, r16ptr: RegisterR16, op: ShiftOp) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = ctx.dev.cpu.get_r16(r16ptr);
                let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
                ctx.dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = ctx.dev.cpu.get_r16(r16ptr);
                let value   = ctx.dev.cpu.get_intermediate_value();
                let result  = shift_right_u8v(ctx, value, op);
                ctx.dev.get_mmu_mut().write_u8(ctx.ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    /// Performs an arithmetic shift right of the value of a register.
    fn sra_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = shift_right_u8v(ctx, value, ShiftOp::ShiftArithmetic);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Performs an arithmetic shift right of the value on a memory location.
    fn sra_r16ptr(ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(ctx, r16ptr, ShiftOp::ShiftArithmetic)
    }

    /// Performs an arithmetic shift right of the value of a register.
    fn srl_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = shift_right_u8v(ctx, value, ShiftOp::ShiftLogical);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Performs an arithmetic shift right of the value on a memory location.
    fn srl_r16ptr(ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(ctx, r16ptr, ShiftOp::ShiftLogical)
    }

    /// Rotates the value of a register to the right through the carry flag.
    fn rr_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        rr_r8_nc(ctx, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the right through the carry flag.
    fn rr_r8_nc(ctx: &mut OpCodeContext, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = shift_right_u8v_nc(ctx, value, ShiftOp::RotateThroughCarry, nullcheck);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the right through the carry flag.
    fn rr_r16ptr(ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(ctx, r16ptr, ShiftOp::RotateThroughCarry)
    }

    /// Rotates the value of a register to the right.
    fn rrc_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        rrc_r8_nc(ctx, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the right.
    fn rrc_r8_nc(ctx: &mut OpCodeContext, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = shift_right_u8v_nc(ctx, value, ShiftOp::Rotate, nullcheck);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the right.
    fn rrc_r16ptr(ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(ctx, r16ptr, ShiftOp::Rotate)
    }


    // arithmetic shift right
    opcode!(sra_a,     [ctx] sra_r8(ctx, RegisterR8::A));
    opcode!(sra_b,     [ctx] sra_r8(ctx, RegisterR8::B));
    opcode!(sra_c,     [ctx] sra_r8(ctx, RegisterR8::C));
    opcode!(sra_d,     [ctx] sra_r8(ctx, RegisterR8::D));
    opcode!(sra_e,     [ctx] sra_r8(ctx, RegisterR8::E));
    opcode!(sra_h,     [ctx] sra_r8(ctx, RegisterR8::H));
    opcode!(sra_l,     [ctx] sra_r8(ctx, RegisterR8::L));

    // logical shift right
    opcode!(srl_a,     [ctx] srl_r8(ctx, RegisterR8::A));
    opcode!(srl_b,     [ctx] srl_r8(ctx, RegisterR8::B));
    opcode!(srl_c,     [ctx] srl_r8(ctx, RegisterR8::C));
    opcode!(srl_d,     [ctx] srl_r8(ctx, RegisterR8::D));
    opcode!(srl_e,     [ctx] srl_r8(ctx, RegisterR8::E));
    opcode!(srl_h,     [ctx] srl_r8(ctx, RegisterR8::H));
    opcode!(srl_l,     [ctx] srl_r8(ctx, RegisterR8::L));

    // rotate right through carry flag
    opcode!(rra,       [ctx] rr_r8_nc(ctx, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rr_a,      [ctx] rr_r8(ctx, RegisterR8::A));
    opcode!(rr_b,      [ctx] rr_r8(ctx, RegisterR8::B));
    opcode!(rr_c,      [ctx] rr_r8(ctx, RegisterR8::C));
    opcode!(rr_d,      [ctx] rr_r8(ctx, RegisterR8::D));
    opcode!(rr_e,      [ctx] rr_r8(ctx, RegisterR8::E));
    opcode!(rr_h,      [ctx] rr_r8(ctx, RegisterR8::H));
    opcode!(rr_l,      [ctx] rr_r8(ctx, RegisterR8::L));

    // rotate right (carry flag just set)
    opcode!(rrca,      [ctx] rrc_r8_nc(ctx, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rrc_a,     [ctx] rrc_r8(ctx, RegisterR8::A));
    opcode!(rrc_b,     [ctx] rrc_r8(ctx, RegisterR8::B));
    opcode!(rrc_c,     [ctx] rrc_r8(ctx, RegisterR8::C));
    opcode!(rrc_d,     [ctx] rrc_r8(ctx, RegisterR8::D));
    opcode!(rrc_e,     [ctx] rrc_r8(ctx, RegisterR8::E));
    opcode!(rrc_h,     [ctx] rrc_r8(ctx, RegisterR8::H));
    opcode!(rrc_l,     [ctx] rrc_r8(ctx, RegisterR8::L));

    opcode!(sra_hlptr, [ctx] sra_r16ptr(ctx, RegisterR16::HL));
    opcode!(srl_hlptr, [ctx] srl_r16ptr(ctx, RegisterR16::HL));
    opcode!(rr_hlptr,  [ctx] rr_r16ptr (ctx, RegisterR16::HL));
    opcode!(rrc_hlptr, [ctx] rrc_r16ptr(ctx, RegisterR16::HL));
}

////////////////////////////////////////////////
//// SWAP opcodes
pub mod swap {
    use super::*;

    /// Swaps the low and high nibble of a byte.
    fn swap_nibbles_u8v(ctx: &mut OpCodeContext, value: u8) -> u8 {
        let low   = (value >> 0) & 0x0f;
        let high  = (value >> 4) & 0x0f;
        let result = (low << 4) | (high);

        ctx.dev.cpu.clear_flags();
        ctx.dev.cpu.set_flag(CpuFlag::Zero, result == 0);

        result
    }

    /// Swaps the low and high nibble of a 8bit register.
    fn swap_r8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = swap_nibbles_u8v(ctx, value);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Swaps the low and high nibble of a byte at the address of a 16bit register pointer.
    fn swap_r16ptr(ctx: &mut OpCodeContext, r16_ptr: RegisterR16) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
                ctx.dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value   = ctx.dev.cpu.get_intermediate_value();
                let result  = swap_nibbles_u8v(ctx, value);
                ctx.dev.get_mmu_mut().write_u8(ctx.ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    // swap low and high nibbles of registers
    opcode!(swap_a,     [ctx] swap_r8(ctx, RegisterR8::A));
    opcode!(swap_b,     [ctx] swap_r8(ctx, RegisterR8::B));
    opcode!(swap_c,     [ctx] swap_r8(ctx, RegisterR8::C));
    opcode!(swap_d,     [ctx] swap_r8(ctx, RegisterR8::D));
    opcode!(swap_e,     [ctx] swap_r8(ctx, RegisterR8::E));
    opcode!(swap_h,     [ctx] swap_r8(ctx, RegisterR8::H));
    opcode!(swap_l,     [ctx] swap_r8(ctx, RegisterR8::L));

    opcode!(swap_hlptr, [ctx] swap_r16ptr(ctx, RegisterR16::HL));
}

////////////////////////////////////////////////
//// Set Bit opcodes
pub mod set_bit {
    use super::*;

    /// Set bit n of a given 8bit value.
    /// value | (1 << bit)
    fn set_bit_u8v(_ctx: &mut OpCodeContext, value: u8, bit: u8) -> u8 {
        let result = value | (1 << bit);
        result
    }

    /// Set bit n of in the given register.
    /// r8 <- r8 | (1 << bit)
    fn set_bit_r8(ctx: &mut OpCodeContext, r8: RegisterR8, bit: u8) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = set_bit_u8v(ctx, value, bit);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Set bit n on a memory address.
    /// (r16) <- (r16) | (1 << bit)
    fn set_bit_r16ptr(ctx: &mut OpCodeContext, r16_ptr: RegisterR16, bit: u8) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
                ctx.dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value   = ctx.dev.cpu.get_intermediate_value();
                let result  = set_bit_u8v(ctx, value, bit);
                ctx.dev.get_mmu_mut().write_u8(ctx.ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    opcode!(set_bit_0_a, [ctx] set_bit_r8(ctx, RegisterR8::A, 0));
    opcode!(set_bit_0_b, [ctx] set_bit_r8(ctx, RegisterR8::B, 0));
    opcode!(set_bit_0_c, [ctx] set_bit_r8(ctx, RegisterR8::C, 0));
    opcode!(set_bit_0_d, [ctx] set_bit_r8(ctx, RegisterR8::D, 0));
    opcode!(set_bit_0_e, [ctx] set_bit_r8(ctx, RegisterR8::E, 0));
    opcode!(set_bit_0_h, [ctx] set_bit_r8(ctx, RegisterR8::H, 0));
    opcode!(set_bit_0_l, [ctx] set_bit_r8(ctx, RegisterR8::L, 0));

    opcode!(set_bit_1_a, [ctx] set_bit_r8(ctx, RegisterR8::A, 1));
    opcode!(set_bit_1_b, [ctx] set_bit_r8(ctx, RegisterR8::B, 1));
    opcode!(set_bit_1_c, [ctx] set_bit_r8(ctx, RegisterR8::C, 1));
    opcode!(set_bit_1_d, [ctx] set_bit_r8(ctx, RegisterR8::D, 1));
    opcode!(set_bit_1_e, [ctx] set_bit_r8(ctx, RegisterR8::E, 1));
    opcode!(set_bit_1_h, [ctx] set_bit_r8(ctx, RegisterR8::H, 1));
    opcode!(set_bit_1_l, [ctx] set_bit_r8(ctx, RegisterR8::L, 1));

    opcode!(set_bit_2_a, [ctx] set_bit_r8(ctx, RegisterR8::A, 2));
    opcode!(set_bit_2_b, [ctx] set_bit_r8(ctx, RegisterR8::B, 2));
    opcode!(set_bit_2_c, [ctx] set_bit_r8(ctx, RegisterR8::C, 2));
    opcode!(set_bit_2_d, [ctx] set_bit_r8(ctx, RegisterR8::D, 2));
    opcode!(set_bit_2_e, [ctx] set_bit_r8(ctx, RegisterR8::E, 2));
    opcode!(set_bit_2_h, [ctx] set_bit_r8(ctx, RegisterR8::H, 2));
    opcode!(set_bit_2_l, [ctx] set_bit_r8(ctx, RegisterR8::L, 2));

    opcode!(set_bit_3_a, [ctx] set_bit_r8(ctx, RegisterR8::A, 3));
    opcode!(set_bit_3_b, [ctx] set_bit_r8(ctx, RegisterR8::B, 3));
    opcode!(set_bit_3_c, [ctx] set_bit_r8(ctx, RegisterR8::C, 3));
    opcode!(set_bit_3_d, [ctx] set_bit_r8(ctx, RegisterR8::D, 3));
    opcode!(set_bit_3_e, [ctx] set_bit_r8(ctx, RegisterR8::E, 3));
    opcode!(set_bit_3_h, [ctx] set_bit_r8(ctx, RegisterR8::H, 3));
    opcode!(set_bit_3_l, [ctx] set_bit_r8(ctx, RegisterR8::L, 3));

    opcode!(set_bit_4_a, [ctx] set_bit_r8(ctx, RegisterR8::A, 4));
    opcode!(set_bit_4_b, [ctx] set_bit_r8(ctx, RegisterR8::B, 4));
    opcode!(set_bit_4_c, [ctx] set_bit_r8(ctx, RegisterR8::C, 4));
    opcode!(set_bit_4_d, [ctx] set_bit_r8(ctx, RegisterR8::D, 4));
    opcode!(set_bit_4_e, [ctx] set_bit_r8(ctx, RegisterR8::E, 4));
    opcode!(set_bit_4_h, [ctx] set_bit_r8(ctx, RegisterR8::H, 4));
    opcode!(set_bit_4_l, [ctx] set_bit_r8(ctx, RegisterR8::L, 4));

    opcode!(set_bit_5_a, [ctx] set_bit_r8(ctx, RegisterR8::A, 5));
    opcode!(set_bit_5_b, [ctx] set_bit_r8(ctx, RegisterR8::B, 5));
    opcode!(set_bit_5_c, [ctx] set_bit_r8(ctx, RegisterR8::C, 5));
    opcode!(set_bit_5_d, [ctx] set_bit_r8(ctx, RegisterR8::D, 5));
    opcode!(set_bit_5_e, [ctx] set_bit_r8(ctx, RegisterR8::E, 5));
    opcode!(set_bit_5_h, [ctx] set_bit_r8(ctx, RegisterR8::H, 5));
    opcode!(set_bit_5_l, [ctx] set_bit_r8(ctx, RegisterR8::L, 5));

    opcode!(set_bit_6_a, [ctx] set_bit_r8(ctx, RegisterR8::A, 6));
    opcode!(set_bit_6_b, [ctx] set_bit_r8(ctx, RegisterR8::B, 6));
    opcode!(set_bit_6_c, [ctx] set_bit_r8(ctx, RegisterR8::C, 6));
    opcode!(set_bit_6_d, [ctx] set_bit_r8(ctx, RegisterR8::D, 6));
    opcode!(set_bit_6_e, [ctx] set_bit_r8(ctx, RegisterR8::E, 6));
    opcode!(set_bit_6_h, [ctx] set_bit_r8(ctx, RegisterR8::H, 6));
    opcode!(set_bit_6_l, [ctx] set_bit_r8(ctx, RegisterR8::L, 6));

    opcode!(set_bit_7_a, [ctx] set_bit_r8(ctx, RegisterR8::A, 7));
    opcode!(set_bit_7_b, [ctx] set_bit_r8(ctx, RegisterR8::B, 7));
    opcode!(set_bit_7_c, [ctx] set_bit_r8(ctx, RegisterR8::C, 7));
    opcode!(set_bit_7_d, [ctx] set_bit_r8(ctx, RegisterR8::D, 7));
    opcode!(set_bit_7_e, [ctx] set_bit_r8(ctx, RegisterR8::E, 7));
    opcode!(set_bit_7_h, [ctx] set_bit_r8(ctx, RegisterR8::H, 7));
    opcode!(set_bit_7_l, [ctx] set_bit_r8(ctx, RegisterR8::L, 7));

    opcode!(set_bit_0_hlptr, [ctx] set_bit_r16ptr(ctx, RegisterR16::HL, 0));
    opcode!(set_bit_1_hlptr, [ctx] set_bit_r16ptr(ctx, RegisterR16::HL, 1));
    opcode!(set_bit_2_hlptr, [ctx] set_bit_r16ptr(ctx, RegisterR16::HL, 2));
    opcode!(set_bit_3_hlptr, [ctx] set_bit_r16ptr(ctx, RegisterR16::HL, 3));
    opcode!(set_bit_4_hlptr, [ctx] set_bit_r16ptr(ctx, RegisterR16::HL, 4));
    opcode!(set_bit_5_hlptr, [ctx] set_bit_r16ptr(ctx, RegisterR16::HL, 5));
    opcode!(set_bit_6_hlptr, [ctx] set_bit_r16ptr(ctx, RegisterR16::HL, 6));
    opcode!(set_bit_7_hlptr, [ctx] set_bit_r16ptr(ctx, RegisterR16::HL, 7));
}

////////////////////////////////////////////////
//// Reset Bit opcodes
pub mod res_bit {
    use super::*;

    /// Resets bit n of a given 8bit value.
    /// value & !(1 << bit)
    fn res_bit_u8v(_ctx: &mut OpCodeContext, value: u8, bit: u8) -> u8 {
        let result = value & !(1 << bit);
        result
    }

    /// Resets bit n of in the given register.
    /// r8 <- r8 & !(1 << bit)
    fn res_bit_r8(ctx: &mut OpCodeContext, r8: RegisterR8, bit: u8) {
        let value  = ctx.dev.cpu.get_r8(r8);
        let result = res_bit_u8v(ctx, value, bit);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Resets bit n on a memory address.
    /// (r16) <- (r16) & !(1 << bit)
    fn res_bit_r16ptr(ctx: &mut OpCodeContext, r16_ptr: RegisterR16, bit: u8) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
                ctx.dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = ctx.dev.cpu.get_r16(r16_ptr);
                let value   = ctx.dev.cpu.get_intermediate_value();
                let result  = res_bit_u8v(ctx, value, bit);
                ctx.dev.get_mmu_mut().write_u8(ctx.ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    opcode!(res_bit_0_a, [ctx] res_bit_r8(ctx, RegisterR8::A, 0));
    opcode!(res_bit_0_b, [ctx] res_bit_r8(ctx, RegisterR8::B, 0));
    opcode!(res_bit_0_c, [ctx] res_bit_r8(ctx, RegisterR8::C, 0));
    opcode!(res_bit_0_d, [ctx] res_bit_r8(ctx, RegisterR8::D, 0));
    opcode!(res_bit_0_e, [ctx] res_bit_r8(ctx, RegisterR8::E, 0));
    opcode!(res_bit_0_h, [ctx] res_bit_r8(ctx, RegisterR8::H, 0));
    opcode!(res_bit_0_l, [ctx] res_bit_r8(ctx, RegisterR8::L, 0));

    opcode!(res_bit_1_a, [ctx] res_bit_r8(ctx, RegisterR8::A, 1));
    opcode!(res_bit_1_b, [ctx] res_bit_r8(ctx, RegisterR8::B, 1));
    opcode!(res_bit_1_c, [ctx] res_bit_r8(ctx, RegisterR8::C, 1));
    opcode!(res_bit_1_d, [ctx] res_bit_r8(ctx, RegisterR8::D, 1));
    opcode!(res_bit_1_e, [ctx] res_bit_r8(ctx, RegisterR8::E, 1));
    opcode!(res_bit_1_h, [ctx] res_bit_r8(ctx, RegisterR8::H, 1));
    opcode!(res_bit_1_l, [ctx] res_bit_r8(ctx, RegisterR8::L, 1));

    opcode!(res_bit_2_a, [ctx] res_bit_r8(ctx, RegisterR8::A, 2));
    opcode!(res_bit_2_b, [ctx] res_bit_r8(ctx, RegisterR8::B, 2));
    opcode!(res_bit_2_c, [ctx] res_bit_r8(ctx, RegisterR8::C, 2));
    opcode!(res_bit_2_d, [ctx] res_bit_r8(ctx, RegisterR8::D, 2));
    opcode!(res_bit_2_e, [ctx] res_bit_r8(ctx, RegisterR8::E, 2));
    opcode!(res_bit_2_h, [ctx] res_bit_r8(ctx, RegisterR8::H, 2));
    opcode!(res_bit_2_l, [ctx] res_bit_r8(ctx, RegisterR8::L, 2));

    opcode!(res_bit_3_a, [ctx] res_bit_r8(ctx, RegisterR8::A, 3));
    opcode!(res_bit_3_b, [ctx] res_bit_r8(ctx, RegisterR8::B, 3));
    opcode!(res_bit_3_c, [ctx] res_bit_r8(ctx, RegisterR8::C, 3));
    opcode!(res_bit_3_d, [ctx] res_bit_r8(ctx, RegisterR8::D, 3));
    opcode!(res_bit_3_e, [ctx] res_bit_r8(ctx, RegisterR8::E, 3));
    opcode!(res_bit_3_h, [ctx] res_bit_r8(ctx, RegisterR8::H, 3));
    opcode!(res_bit_3_l, [ctx] res_bit_r8(ctx, RegisterR8::L, 3));

    opcode!(res_bit_4_a, [ctx] res_bit_r8(ctx, RegisterR8::A, 4));
    opcode!(res_bit_4_b, [ctx] res_bit_r8(ctx, RegisterR8::B, 4));
    opcode!(res_bit_4_c, [ctx] res_bit_r8(ctx, RegisterR8::C, 4));
    opcode!(res_bit_4_d, [ctx] res_bit_r8(ctx, RegisterR8::D, 4));
    opcode!(res_bit_4_e, [ctx] res_bit_r8(ctx, RegisterR8::E, 4));
    opcode!(res_bit_4_h, [ctx] res_bit_r8(ctx, RegisterR8::H, 4));
    opcode!(res_bit_4_l, [ctx] res_bit_r8(ctx, RegisterR8::L, 4));

    opcode!(res_bit_5_a, [ctx] res_bit_r8(ctx, RegisterR8::A, 5));
    opcode!(res_bit_5_b, [ctx] res_bit_r8(ctx, RegisterR8::B, 5));
    opcode!(res_bit_5_c, [ctx] res_bit_r8(ctx, RegisterR8::C, 5));
    opcode!(res_bit_5_d, [ctx] res_bit_r8(ctx, RegisterR8::D, 5));
    opcode!(res_bit_5_e, [ctx] res_bit_r8(ctx, RegisterR8::E, 5));
    opcode!(res_bit_5_h, [ctx] res_bit_r8(ctx, RegisterR8::H, 5));
    opcode!(res_bit_5_l, [ctx] res_bit_r8(ctx, RegisterR8::L, 5));

    opcode!(res_bit_6_a, [ctx] res_bit_r8(ctx, RegisterR8::A, 6));
    opcode!(res_bit_6_b, [ctx] res_bit_r8(ctx, RegisterR8::B, 6));
    opcode!(res_bit_6_c, [ctx] res_bit_r8(ctx, RegisterR8::C, 6));
    opcode!(res_bit_6_d, [ctx] res_bit_r8(ctx, RegisterR8::D, 6));
    opcode!(res_bit_6_e, [ctx] res_bit_r8(ctx, RegisterR8::E, 6));
    opcode!(res_bit_6_h, [ctx] res_bit_r8(ctx, RegisterR8::H, 6));
    opcode!(res_bit_6_l, [ctx] res_bit_r8(ctx, RegisterR8::L, 6));

    opcode!(res_bit_7_a, [ctx] res_bit_r8(ctx, RegisterR8::A, 7));
    opcode!(res_bit_7_b, [ctx] res_bit_r8(ctx, RegisterR8::B, 7));
    opcode!(res_bit_7_c, [ctx] res_bit_r8(ctx, RegisterR8::C, 7));
    opcode!(res_bit_7_d, [ctx] res_bit_r8(ctx, RegisterR8::D, 7));
    opcode!(res_bit_7_e, [ctx] res_bit_r8(ctx, RegisterR8::E, 7));
    opcode!(res_bit_7_h, [ctx] res_bit_r8(ctx, RegisterR8::H, 7));
    opcode!(res_bit_7_l, [ctx] res_bit_r8(ctx, RegisterR8::L, 7));

    opcode!(res_bit_0_hlptr, [ctx] res_bit_r16ptr(ctx, RegisterR16::HL, 0));
    opcode!(res_bit_1_hlptr, [ctx] res_bit_r16ptr(ctx, RegisterR16::HL, 1));
    opcode!(res_bit_2_hlptr, [ctx] res_bit_r16ptr(ctx, RegisterR16::HL, 2));
    opcode!(res_bit_3_hlptr, [ctx] res_bit_r16ptr(ctx, RegisterR16::HL, 3));
    opcode!(res_bit_4_hlptr, [ctx] res_bit_r16ptr(ctx, RegisterR16::HL, 4));
    opcode!(res_bit_5_hlptr, [ctx] res_bit_r16ptr(ctx, RegisterR16::HL, 5));
    opcode!(res_bit_6_hlptr, [ctx] res_bit_r16ptr(ctx, RegisterR16::HL, 6));
    opcode!(res_bit_7_hlptr, [ctx] res_bit_r16ptr(ctx, RegisterR16::HL, 7));
}

////////////////////////////////////////////////
//// Check Bit opcodes
pub mod chk_bit {
    use super::*;

    /// Checks if bit n of a value is set.
    /// Set the Zero flag, if the bit was 0.
    fn check_bit_u8v(ctx: &mut OpCodeContext, value: u8, bit: u8) {
        let result = value & (1 << bit);
        ctx.dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, false);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, true);
    }

    /// Checks if bit n of a register is set.
    /// Set the Zero flag, if the bit was 0.
    fn check_bit_r8(ctx: &mut OpCodeContext, r8: RegisterR8, bit: u8) {
        let value  = ctx.dev.cpu.get_r8(r8);
        check_bit_u8v(ctx, value, bit);
    }

    /// Checks if bit n on a memory address is set.
    /// Set the Zero flag, if the bit was 0.
    fn check_bit_r16ptr(ctx: &mut OpCodeContext, r16_ptr: RegisterR16, bit: u8) {
        let address = ctx.dev.cpu.get_r16(r16_ptr);
        let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
        check_bit_u8v(ctx, value, bit);
    }


    opcode!(check_bit_0_a,     [ctx] check_bit_r8(ctx, RegisterR8::A, 0));
    opcode!(check_bit_0_b,     [ctx] check_bit_r8(ctx, RegisterR8::B, 0));
    opcode!(check_bit_0_c,     [ctx] check_bit_r8(ctx, RegisterR8::C, 0));
    opcode!(check_bit_0_d,     [ctx] check_bit_r8(ctx, RegisterR8::D, 0));
    opcode!(check_bit_0_e,     [ctx] check_bit_r8(ctx, RegisterR8::E, 0));
    opcode!(check_bit_0_h,     [ctx] check_bit_r8(ctx, RegisterR8::H, 0));
    opcode!(check_bit_0_l,     [ctx] check_bit_r8(ctx, RegisterR8::L, 0));
    opcode!(check_bit_0_hlptr, [ctx] check_bit_r16ptr(ctx, RegisterR16::HL, 0));

    opcode!(check_bit_1_a,     [ctx] check_bit_r8(ctx, RegisterR8::A, 1));
    opcode!(check_bit_1_b,     [ctx] check_bit_r8(ctx, RegisterR8::B, 1));
    opcode!(check_bit_1_c,     [ctx] check_bit_r8(ctx, RegisterR8::C, 1));
    opcode!(check_bit_1_d,     [ctx] check_bit_r8(ctx, RegisterR8::D, 1));
    opcode!(check_bit_1_e,     [ctx] check_bit_r8(ctx, RegisterR8::E, 1));
    opcode!(check_bit_1_h,     [ctx] check_bit_r8(ctx, RegisterR8::H, 1));
    opcode!(check_bit_1_l,     [ctx] check_bit_r8(ctx, RegisterR8::L, 1));
    opcode!(check_bit_1_hlptr, [ctx] check_bit_r16ptr(ctx, RegisterR16::HL, 1));

    opcode!(check_bit_2_a,     [ctx] check_bit_r8(ctx, RegisterR8::A, 2));
    opcode!(check_bit_2_b,     [ctx] check_bit_r8(ctx, RegisterR8::B, 2));
    opcode!(check_bit_2_c,     [ctx] check_bit_r8(ctx, RegisterR8::C, 2));
    opcode!(check_bit_2_d,     [ctx] check_bit_r8(ctx, RegisterR8::D, 2));
    opcode!(check_bit_2_e,     [ctx] check_bit_r8(ctx, RegisterR8::E, 2));
    opcode!(check_bit_2_h,     [ctx] check_bit_r8(ctx, RegisterR8::H, 2));
    opcode!(check_bit_2_l,     [ctx] check_bit_r8(ctx, RegisterR8::L, 2));
    opcode!(check_bit_2_hlptr, [ctx] check_bit_r16ptr(ctx, RegisterR16::HL, 2));

    opcode!(check_bit_3_a,     [ctx] check_bit_r8(ctx, RegisterR8::A, 3));
    opcode!(check_bit_3_b,     [ctx] check_bit_r8(ctx, RegisterR8::B, 3));
    opcode!(check_bit_3_c,     [ctx] check_bit_r8(ctx, RegisterR8::C, 3));
    opcode!(check_bit_3_d,     [ctx] check_bit_r8(ctx, RegisterR8::D, 3));
    opcode!(check_bit_3_e,     [ctx] check_bit_r8(ctx, RegisterR8::E, 3));
    opcode!(check_bit_3_h,     [ctx] check_bit_r8(ctx, RegisterR8::H, 3));
    opcode!(check_bit_3_l,     [ctx] check_bit_r8(ctx, RegisterR8::L, 3));
    opcode!(check_bit_3_hlptr, [ctx] check_bit_r16ptr(ctx, RegisterR16::HL, 3));

    opcode!(check_bit_4_a,     [ctx] check_bit_r8(ctx, RegisterR8::A, 4));
    opcode!(check_bit_4_b,     [ctx] check_bit_r8(ctx, RegisterR8::B, 4));
    opcode!(check_bit_4_c,     [ctx] check_bit_r8(ctx, RegisterR8::C, 4));
    opcode!(check_bit_4_d,     [ctx] check_bit_r8(ctx, RegisterR8::D, 4));
    opcode!(check_bit_4_e,     [ctx] check_bit_r8(ctx, RegisterR8::E, 4));
    opcode!(check_bit_4_h,     [ctx] check_bit_r8(ctx, RegisterR8::H, 4));
    opcode!(check_bit_4_l,     [ctx] check_bit_r8(ctx, RegisterR8::L, 4));
    opcode!(check_bit_4_hlptr, [ctx] check_bit_r16ptr(ctx, RegisterR16::HL, 4));

    opcode!(check_bit_5_a,     [ctx] check_bit_r8(ctx, RegisterR8::A, 5));
    opcode!(check_bit_5_b,     [ctx] check_bit_r8(ctx, RegisterR8::B, 5));
    opcode!(check_bit_5_c,     [ctx] check_bit_r8(ctx, RegisterR8::C, 5));
    opcode!(check_bit_5_d,     [ctx] check_bit_r8(ctx, RegisterR8::D, 5));
    opcode!(check_bit_5_e,     [ctx] check_bit_r8(ctx, RegisterR8::E, 5));
    opcode!(check_bit_5_h,     [ctx] check_bit_r8(ctx, RegisterR8::H, 5));
    opcode!(check_bit_5_l,     [ctx] check_bit_r8(ctx, RegisterR8::L, 5));
    opcode!(check_bit_5_hlptr, [ctx] check_bit_r16ptr(ctx, RegisterR16::HL, 5));

    opcode!(check_bit_6_a,     [ctx] check_bit_r8(ctx, RegisterR8::A, 6));
    opcode!(check_bit_6_b,     [ctx] check_bit_r8(ctx, RegisterR8::B, 6));
    opcode!(check_bit_6_c,     [ctx] check_bit_r8(ctx, RegisterR8::C, 6));
    opcode!(check_bit_6_d,     [ctx] check_bit_r8(ctx, RegisterR8::D, 6));
    opcode!(check_bit_6_e,     [ctx] check_bit_r8(ctx, RegisterR8::E, 6));
    opcode!(check_bit_6_h,     [ctx] check_bit_r8(ctx, RegisterR8::H, 6));
    opcode!(check_bit_6_l,     [ctx] check_bit_r8(ctx, RegisterR8::L, 6));
    opcode!(check_bit_6_hlptr, [ctx] check_bit_r16ptr(ctx, RegisterR16::HL, 6));

    opcode!(check_bit_7_a,     [ctx] check_bit_r8(ctx, RegisterR8::A, 7));
    opcode!(check_bit_7_b,     [ctx] check_bit_r8(ctx, RegisterR8::B, 7));
    opcode!(check_bit_7_c,     [ctx] check_bit_r8(ctx, RegisterR8::C, 7));
    opcode!(check_bit_7_d,     [ctx] check_bit_r8(ctx, RegisterR8::D, 7));
    opcode!(check_bit_7_e,     [ctx] check_bit_r8(ctx, RegisterR8::E, 7));
    opcode!(check_bit_7_h,     [ctx] check_bit_r8(ctx, RegisterR8::H, 7));
    opcode!(check_bit_7_l,     [ctx] check_bit_r8(ctx, RegisterR8::L, 7));
    opcode!(check_bit_7_hlptr, [ctx] check_bit_r16ptr(ctx, RegisterR16::HL, 7));
}

////////////////////////////////////////////////
//// CP opcodes
pub mod cp {
    use super::*;

    /// Compares two values.
    fn cp_u8v_u8v(ctx: &mut OpCodeContext, value1: u8, value2: u8) {
        let (_, half_carry, carry) = carrying_sub_u8(value1, value2, false);

        ctx.dev.cpu.set_flag(CpuFlag::Zero, value1 == value2);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, true);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        ctx.dev.cpu.set_flag(CpuFlag::Carry, carry);
    }

    /// Compares two values.
    /// cp r8, u8
    fn cp_r8_u8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        let value1 = ctx.dev.cpu.get_r8(r8);
        let value2 = ctx.dev.cpu.fetch_u8(ctx.ec);
        cp_u8v_u8v(ctx, value1, value2);
    }

    /// Compares two values.
    /// cp dst, src
    fn cp_r8_r8(ctx: &mut OpCodeContext, dst: RegisterR8, src: RegisterR8) {
        let value1 = ctx.dev.cpu.get_r8(dst);
        let value2 = ctx.dev.cpu.get_r8(src);
        cp_u8v_u8v(ctx, value1, value2);
    }

    /// Compares two values.
    /// cp dst, (src_ptr)
    fn cp_r8_r16ptr(ctx: &mut OpCodeContext, dst: RegisterR8, src_ptr: RegisterR16) {
        let value1  = ctx.dev.cpu.get_r8(dst);
        let address = ctx.dev.cpu.get_r16(src_ptr);
        let value2  = ctx.dev.get_mmu().read_u8(ctx.ec, address);
        cp_u8v_u8v(ctx, value1, value2);
    }


    opcode!(cp_a_u8,    [ctx] cp_r8_u8(ctx, RegisterR8::A));
    opcode!(cp_a_a,     [ctx] cp_r8_r8(ctx, RegisterR8::A, RegisterR8::A));
    opcode!(cp_a_b,     [ctx] cp_r8_r8(ctx, RegisterR8::A, RegisterR8::B));
    opcode!(cp_a_c,     [ctx] cp_r8_r8(ctx, RegisterR8::A, RegisterR8::C));
    opcode!(cp_a_d,     [ctx] cp_r8_r8(ctx, RegisterR8::A, RegisterR8::D));
    opcode!(cp_a_e,     [ctx] cp_r8_r8(ctx, RegisterR8::A, RegisterR8::E));
    opcode!(cp_a_h,     [ctx] cp_r8_r8(ctx, RegisterR8::A, RegisterR8::H));
    opcode!(cp_a_l,     [ctx] cp_r8_r8(ctx, RegisterR8::A, RegisterR8::L));
    opcode!(cp_a_hlptr, [ctx] cp_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// AND opcodes
pub mod and {
    use super::*;

    /// Computes a bitwise AND.
    /// r8 <- r8 & value
    fn and_r8_u8v(ctx: &mut OpCodeContext, r8: RegisterR8, value: u8) {
        let old_value = ctx.dev.cpu.get_r8(r8);
        let result    = old_value & value;
        ctx.dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, false);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, true);
        ctx.dev.cpu.set_flag(CpuFlag::Carry, false);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Computes a bitwise AND.
    /// dst <- dst & u8
    fn and_r8_u8(ctx: &mut OpCodeContext, dst: RegisterR8) {
        let value = ctx.dev.cpu.fetch_u8(ctx.ec);
        and_r8_u8v(ctx, dst, value);
    }

    /// Computes a bitwise AND.
    /// dst <- dst & src
    fn and_r8_r8(ctx: &mut OpCodeContext, dst: RegisterR8, src: RegisterR8) {
        let value = ctx.dev.cpu.get_r8(src);
        and_r8_u8v(ctx, dst, value);
    }

    /// Computes a bitwise AND.
    /// dst <- dst & (src_ptr)
    fn and_r8_r16ptr(ctx: &mut OpCodeContext, dst: RegisterR8, src_ptr: RegisterR16) {
        let address = ctx.dev.cpu.get_r16(src_ptr);
        let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
        and_r8_u8v(ctx, dst, value);
    }


    opcode!(and_a_u8,    [ctx] and_r8_u8(ctx, RegisterR8::A));
    opcode!(and_a_a,     [ctx] and_r8_r8(ctx, RegisterR8::A, RegisterR8::A));
    opcode!(and_a_b,     [ctx] and_r8_r8(ctx, RegisterR8::A, RegisterR8::B));
    opcode!(and_a_c,     [ctx] and_r8_r8(ctx, RegisterR8::A, RegisterR8::C));
    opcode!(and_a_d,     [ctx] and_r8_r8(ctx, RegisterR8::A, RegisterR8::D));
    opcode!(and_a_e,     [ctx] and_r8_r8(ctx, RegisterR8::A, RegisterR8::E));
    opcode!(and_a_h,     [ctx] and_r8_r8(ctx, RegisterR8::A, RegisterR8::H));
    opcode!(and_a_l,     [ctx] and_r8_r8(ctx, RegisterR8::A, RegisterR8::L));
    opcode!(and_a_hlptr, [ctx] and_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// OR opcodes
pub mod or {
    use super::*;

    /// Computes a bitwise OR.
    /// r8 <- r8 | value
    fn or_r8_u8v(ctx: &mut OpCodeContext, r8: RegisterR8, value: u8) {
        let old_value = ctx.dev.cpu.get_r8(r8);
        let result    = old_value | value;
        ctx.dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, false);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, false);
        ctx.dev.cpu.set_flag(CpuFlag::Carry, false);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Computes a bitwise OR.
    /// dst <- dst | u8
    fn or_r8_u8(ctx: &mut OpCodeContext, dst: RegisterR8) {
        let value = ctx.dev.cpu.fetch_u8(ctx.ec);
        or_r8_u8v(ctx, dst, value);
    }

    /// Computes a bitwise OR.
    /// dst <- dst | src
    fn or_r8_r8(ctx: &mut OpCodeContext, dst: RegisterR8, src: RegisterR8) {
        let value = ctx.dev.cpu.get_r8(src);
        or_r8_u8v(ctx, dst, value);
    }

    /// Computes a bitwise OR.
    /// dst <- dst | (src_ptr)
    fn or_r8_r16ptr(ctx: &mut OpCodeContext, dst: RegisterR8, src_ptr: RegisterR16) {
        let address = ctx.dev.cpu.get_r16(src_ptr);
        let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
        or_r8_u8v(ctx, dst, value);
    }


    opcode!(or_a_u8,    [ctx] or_r8_u8(ctx, RegisterR8::A));
    opcode!(or_a_a,     [ctx] or_r8_r8(ctx, RegisterR8::A, RegisterR8::A));
    opcode!(or_a_b,     [ctx] or_r8_r8(ctx, RegisterR8::A, RegisterR8::B));
    opcode!(or_a_c,     [ctx] or_r8_r8(ctx, RegisterR8::A, RegisterR8::C));
    opcode!(or_a_d,     [ctx] or_r8_r8(ctx, RegisterR8::A, RegisterR8::D));
    opcode!(or_a_e,     [ctx] or_r8_r8(ctx, RegisterR8::A, RegisterR8::E));
    opcode!(or_a_h,     [ctx] or_r8_r8(ctx, RegisterR8::A, RegisterR8::H));
    opcode!(or_a_l,     [ctx] or_r8_r8(ctx, RegisterR8::A, RegisterR8::L));
    opcode!(or_a_hlptr, [ctx] or_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// XOR opcodes
pub mod xor {
    use super::*;

    /// Computes a bitwise XOR.
    /// r8 <- r8 ^ value
    fn xor_r8_u8v(ctx: &mut OpCodeContext, r8: RegisterR8, value: u8) {
        let old_value = ctx.dev.cpu.get_r8(r8);
        let result    = old_value ^ value;
        ctx.dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        ctx.dev.cpu.set_flag(CpuFlag::Negative, false);
        ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, false);
        ctx.dev.cpu.set_flag(CpuFlag::Carry, false);
        ctx.dev.cpu.set_r8(r8, result);
    }

    /// Computes a bitwise XOR.
    /// dst <- dst ^ src
    fn xor_r8_r8(ctx: &mut OpCodeContext, dst: RegisterR8, src: RegisterR8) {
        let value = ctx.dev.cpu.get_r8(src);
        xor_r8_u8v(ctx, dst, value);
    }

    /// Computes a bitwise XOR.
    /// dst <- dst ^ (src_ptr)
    fn xor_r8_r16ptr(ctx: &mut OpCodeContext, dst: RegisterR8, src_ptr: RegisterR16) {
        let address = ctx.dev.cpu.get_r16(src_ptr);
        let value   = ctx.dev.get_mmu().read_u8(ctx.ec, address);
        xor_r8_u8v(ctx, dst, value);
    }


    fn xor_r8_u8(ctx: &mut OpCodeContext, r8: RegisterR8) {
        let value = ctx.dev.cpu.fetch_u8(ctx.ec);
        xor_r8_u8v(ctx, r8, value);
    }


    opcode!(xor_a_u8,    [ctx] xor_r8_u8(ctx, RegisterR8::A));
    opcode!(xor_a_a,     [ctx] xor_r8_r8(ctx, RegisterR8::A, RegisterR8::A));
    opcode!(xor_a_b,     [ctx] xor_r8_r8(ctx, RegisterR8::A, RegisterR8::B));
    opcode!(xor_a_c,     [ctx] xor_r8_r8(ctx, RegisterR8::A, RegisterR8::C));
    opcode!(xor_a_d,     [ctx] xor_r8_r8(ctx, RegisterR8::A, RegisterR8::D));
    opcode!(xor_a_e,     [ctx] xor_r8_r8(ctx, RegisterR8::A, RegisterR8::E));
    opcode!(xor_a_h,     [ctx] xor_r8_r8(ctx, RegisterR8::A, RegisterR8::H));
    opcode!(xor_a_l,     [ctx] xor_r8_r8(ctx, RegisterR8::A, RegisterR8::L));
    opcode!(xor_a_hlptr, [ctx] xor_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// other

// Convert a BCD Number.
opcode!(daa, [ctx] {
    let mut a     = ctx.dev.cpu.get_r8(RegisterR8::A);
    let mut half  = ctx.dev.cpu.is_flag_set(CpuFlag::HalfCarry);
    let mut carry = ctx.dev.cpu.is_flag_set(CpuFlag::Carry);
    let mut tmp_a = a as u16;

    if ctx.dev.cpu.is_flag_set(CpuFlag::Negative) {
        if half {
            tmp_a = tmp_a.wrapping_sub(0x06);

            if !carry {
                tmp_a &= 0xff;
            }
        }

        if carry {
            tmp_a = tmp_a.wrapping_sub(0x60);
        }
    }
    else {
        if half || ((tmp_a & 0x0f) >= 0x0a) {
            tmp_a = tmp_a.wrapping_add(0x06);
        }

        if carry || (tmp_a >= 0xa0) {
            tmp_a = tmp_a.wrapping_add(0x60);
        }
    }

    a      = (tmp_a & 0x00ff) as u8;
    carry |= (tmp_a & 0x0100) != 0;
    half   = false;

    ctx.dev.cpu.set_flag(CpuFlag::Zero,      a == 0);
    ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, half);
    ctx.dev.cpu.set_flag(CpuFlag::Carry,     carry);

    ctx.dev.cpu.set_r8(RegisterR8::A, a);
});

// Complement
opcode!(cpl_a, [ctx] {
    let value  = ctx.dev.cpu.get_r8(RegisterR8::A);
    let result = !value;
    ctx.dev.cpu.set_r8(RegisterR8::A, result);
    ctx.dev.cpu.set_flag(CpuFlag::Negative,  true);
    ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, true);
});

// Set Carry flag.
opcode!(scf, [ctx] {
    ctx.dev.cpu.set_flag(CpuFlag::Negative,  false);
    ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, false);
    ctx.dev.cpu.set_flag(CpuFlag::Carry,     true);
});

// Change carry flag
opcode!(ccf, [ctx] {
    ctx.dev.cpu.set_flag(CpuFlag::Negative,  false);
    ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, false);
    ctx.dev.cpu.set_flag(CpuFlag::Carry,     !ctx.dev.cpu.is_flag_set(CpuFlag::Carry));
});

