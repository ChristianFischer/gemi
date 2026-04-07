/*
 * Copyright (C) 2022-2026 by Christian Fischer
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
use crate::emulator_client::EmulatorClientMut;
use crate::emulator_device::EmulatorDevice;
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
    fn increment_u8v(dev: &mut EmulatorDevice, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        dev.cpu.set_flag(CpuFlag::Negative, false);
        dev.cpu.set_flag(CpuFlag::HalfCarry, (result & 0x0f) == 0);

        result
    }

    /// Increments a 16bit value.
    fn increment_u16v(_dev: &mut EmulatorDevice, value: u16) -> u16 {
        let result = value.wrapping_add(1);
        result
    }

    /// Increments a value
    /// r8 <- r8 + 1
    fn increment_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        let value  = dev.cpu.get_r8(r8);
        let result = increment_u8v(dev, value);
        dev.cpu.set_r8(r8, result);
    }

    /// Increments a value
    /// r16 <- r16 + 1
    fn increment_r16(dev: &mut EmulatorDevice, r16: RegisterR16) {
        let value  = dev.cpu.get_r16(r16);
        let result = increment_u16v(dev, value);
        dev.cpu.set_r16(r16, result);
    }

    /// Increments a value.
    /// (r16) <- (r16) + 1
    fn increment_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16_ptr: RegisterR16) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value   = dev.get_mmu().read_u8(ec, address);
                dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value   = dev.cpu.get_intermediate_value();
                let result  = increment_u8v(dev, value);
                dev.get_mmu_mut().write_u8(ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    // INC r8
    opcode!(inc_a, [dev] increment_r8(dev, RegisterR8::A));
    opcode!(inc_b, [dev] increment_r8(dev, RegisterR8::B));
    opcode!(inc_c, [dev] increment_r8(dev, RegisterR8::C));
    opcode!(inc_d, [dev] increment_r8(dev, RegisterR8::D));
    opcode!(inc_e, [dev] increment_r8(dev, RegisterR8::E));
    opcode!(inc_h, [dev] increment_r8(dev, RegisterR8::H));
    opcode!(inc_l, [dev] increment_r8(dev, RegisterR8::L));

    // INC r16
    opcode!(inc_bc, [dev] increment_r16(dev, RegisterR16::BC));
    opcode!(inc_de, [dev] increment_r16(dev, RegisterR16::DE));
    opcode!(inc_hl, [dev] increment_r16(dev, RegisterR16::HL));

    // INC (r16)
    opcode!(inc_hlptr, [dev, ec, ctx] increment_r16ptr(dev, ec, ctx, RegisterR16::HL));

    // INC SP
    opcode!(inc_sp, [dev] {
        let sp_old = dev.cpu.get_stack_pointer();
        let sp_new = sp_old.wrapping_add(1);
        dev.cpu.set_stack_pointer(sp_new);
    });
}


////////////////////////////////////////////////
//// DEC opcodes
pub mod dec {
    use super::*;

    /// Decrements a 8bit value.
    fn decrement_u8v(dev: &mut EmulatorDevice, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
    
        dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        dev.cpu.set_flag(CpuFlag::Negative, true);
        dev.cpu.set_flag(CpuFlag::HalfCarry, (result & 0x0f) == 0x0f);
    
        result
    }
    
    /// Decrements a 16bit value.
    fn decrement_u16v(_dev: &mut EmulatorDevice, value: u16) -> u16 {
        let result = value.wrapping_sub(1);
        result
    }
    
    /// Decrements a value
    /// r8 <- r8 - 1
    fn decrement_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        let value  = dev.cpu.get_r8(r8);
        let result = decrement_u8v(dev, value);
        dev.cpu.set_r8(r8, result);
    }
    
    /// Decrements a value
    /// r16 <- r16 - 1
    fn decrement_r16(dev: &mut EmulatorDevice, r16: RegisterR16) {
        let value  = dev.cpu.get_r16(r16);
        let result = decrement_u16v(dev, value);
        dev.cpu.set_r16(r16, result);
    }
    
    /// Decrements a value.
    /// (r16) <- (r16) - 1
    fn decrement_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16_ptr: RegisterR16) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value   = dev.get_mmu().read_u8(ec, address);
                dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            },

            1 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value = dev.cpu.get_intermediate_value();
                let result  = decrement_u8v(dev, value);
                dev.get_mmu_mut().write_u8(ec, address, result);

                OpCodeResult::Done
            },

            _ => unreachable!()
        }
    }
    

    // DEC r8
    opcode!(dec_a, [dev] decrement_r8(dev, RegisterR8::A));
    opcode!(dec_b, [dev] decrement_r8(dev, RegisterR8::B));
    opcode!(dec_c, [dev] decrement_r8(dev, RegisterR8::C));
    opcode!(dec_d, [dev] decrement_r8(dev, RegisterR8::D));
    opcode!(dec_e, [dev] decrement_r8(dev, RegisterR8::E));
    opcode!(dec_h, [dev] decrement_r8(dev, RegisterR8::H));
    opcode!(dec_l, [dev] decrement_r8(dev, RegisterR8::L));

    // DEC r16
    opcode!(dec_bc, [dev] decrement_r16(dev, RegisterR16::BC));
    opcode!(dec_de, [dev] decrement_r16(dev, RegisterR16::DE));
    opcode!(dec_hl, [dev] decrement_r16(dev, RegisterR16::HL));

    // DEC (r16)
    opcode!(dec_hlptr, [dev, ec, ctx] decrement_r16ptr(dev, ec, ctx, RegisterR16::HL));

    // DEC SP
    opcode!(dec_sp, [dev] {
        let sp_old = dev.cpu.get_stack_pointer();
        let sp_new = sp_old.wrapping_sub(1);
        dev.cpu.set_stack_pointer(sp_new);
    });
}


////////////////////////////////////////////////
//// ADD / ADC opcodes
pub mod add {
    use super::*;

    /// Adds two values and stores it into a 8bit register.
    /// r8 <- r8 + value + (carry flag, if add_carry)
    fn add_r8_u8v(dev: &mut EmulatorDevice, r8: RegisterR8, value: u8, add_carry: bool) {
        let current_carry = add_carry && dev.cpu.is_flag_set(CpuFlag::Carry);
        let current_value = dev.cpu.get_r8(r8);
        let (result, half_carry, carry) = carrying_add_u8(current_value, value, current_carry);

        dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        dev.cpu.set_flag(CpuFlag::Negative, false);
        dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        dev.cpu.set_flag(CpuFlag::Carry, carry);
        dev.cpu.set_r8(r8, result);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + u8 + (carry flag, if add_carry)
    fn add_r8_u8(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8, add_carry: bool) {
        let value = dev.cpu.fetch_u8(ec);
        add_r8_u8v(dev, dst, value, add_carry);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + src + (carry flag, if add_carry)
    fn add_r8_r8(dev: &mut EmulatorDevice, _ec: &mut impl EmulatorClientMut, dst: RegisterR8, src: RegisterR8, add_carry: bool) {
        let value = dev.cpu.get_r8(src);
        add_r8_u8v(dev, dst, value, add_carry);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + (src_ptr) + (carry flag, if add_carry)
    fn add_r8_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8, src_ptr: RegisterR16, add_carry: bool) {
        let address = dev.cpu.get_r16(src_ptr);
        let value   = dev.get_mmu().read_u8(ec, address);
        add_r8_u8v(dev, dst, value, add_carry);
    }

    /// Adds two values and stores it into a 16bit register.
    /// r16 <- r16 + value
    fn add_r16_u16v(dev: &mut EmulatorDevice, r16: RegisterR16, value: u16) {
        let current_value = dev.cpu.get_r16(r16);
        let (result, half_carry, carry) = carrying_add_u16(current_value, value, false);

        dev.cpu.set_flag(CpuFlag::Negative, false);
        dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        dev.cpu.set_flag(CpuFlag::Carry, carry);
        dev.cpu.set_r16(r16, result);
    }

    /// Adds two values and stores it into a 16bit register.
    /// dst <- dst + src
    fn add_r16_r16(dev: &mut EmulatorDevice, dst: RegisterR16, src: RegisterR16) {
        let value = dev.cpu.get_r16(src);
        add_r16_u16v(dev, dst, value);
    }


    // ADD r8, ?
    opcode!(add_a_u8,    [dev, ec] add_r8_u8(dev, ec, RegisterR8::A, false));
    opcode!(add_a_a,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::A, false));
    opcode!(add_a_b,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::B, false));
    opcode!(add_a_c,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::C, false));
    opcode!(add_a_d,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::D, false));
    opcode!(add_a_e,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::E, false));
    opcode!(add_a_h,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::H, false));
    opcode!(add_a_l,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::L, false));
    opcode!(add_a_hlptr, [dev, ec] add_r8_r16ptr(dev, ec, RegisterR8::A, RegisterR16::HL, false));

    // add with carry flag
    // ADC r8, ?
    opcode!(adc_a_u8,    [dev, ec] add_r8_u8(dev, ec, RegisterR8::A, true));
    opcode!(adc_a_a,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::A, true));
    opcode!(adc_a_b,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::B, true));
    opcode!(adc_a_c,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::C, true));
    opcode!(adc_a_d,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::D, true));
    opcode!(adc_a_e,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::E, true));
    opcode!(adc_a_h,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::H, true));
    opcode!(adc_a_l,     [dev, ec] add_r8_r8(dev, ec, RegisterR8::A, RegisterR8::L, true));
    opcode!(adc_a_hlptr, [dev, ec] add_r8_r16ptr(dev, ec, RegisterR8::A, RegisterR16::HL, true));

    // ADD r16, r16
    opcode!(add_hl_bc, [dev] add_r16_r16(dev, RegisterR16::HL, RegisterR16::BC));
    opcode!(add_hl_de, [dev] add_r16_r16(dev, RegisterR16::HL, RegisterR16::DE));
    opcode!(add_hl_hl, [dev] add_r16_r16(dev, RegisterR16::HL, RegisterR16::HL));
    opcode!(add_hl_sp, [dev] add_r16_u16v(dev, RegisterR16::HL, dev.cpu.get_stack_pointer()));
}


////////////////////////////////////////////////
//// SUB / SBC opcodes
pub mod sub {
    use super::*;

    /// Subtracts a value from another one and stores the result into a 8bit register.
    /// r8 <- r8 - value - (carry flag, if sub_carry)
    fn sub_r8_u8v(dev: &mut EmulatorDevice, r8: RegisterR8, value: u8, sub_carry: bool) {
        let current_carry = sub_carry && dev.cpu.is_flag_set(CpuFlag::Carry);
        let current_value = dev.cpu.get_r8(r8);
        let (result, half_carry, carry) = carrying_sub_u8(current_value, value, current_carry);

        dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        dev.cpu.set_flag(CpuFlag::Negative, true);
        dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        dev.cpu.set_flag(CpuFlag::Carry, carry);
        dev.cpu.set_r8(r8, result);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + u8 + (carry flag, if add_carry)
    fn sub_r8_u8(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8, add_carry: bool) {
        let value = dev.cpu.fetch_u8(ec);
        sub_r8_u8v(dev, dst, value, add_carry);
    }

    /// Subtracts a value from another one and stores the result into a 8bit register.
    /// dst <- dst - src - (carry flag, if sub_carry)
    fn sub_r8_r8(dev: &mut EmulatorDevice, _ec: &mut impl EmulatorClientMut, dst: RegisterR8, src: RegisterR8, sub_carry: bool) {
        let value = dev.cpu.get_r8(src);
        sub_r8_u8v(dev, dst, value, sub_carry);
    }

    /// Subtracts a value from another one and stores the result into a 8bit register.
    /// dst <- dst - (src_ptr) - (carry flag, if sub_carry)
    fn sub_r8_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8, src_ptr: RegisterR16, sub_carry: bool) {
        let address = dev.cpu.get_r16(src_ptr);
        let value   = dev.get_mmu().read_u8(ec, address);
        sub_r8_u8v(dev, dst, value, sub_carry);
    }


    // SUB r8, ?
    opcode!(sub_a_u8,    [dev, ec] sub_r8_u8(dev, ec, RegisterR8::A, false));
    opcode!(sub_a_a,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::A, false));
    opcode!(sub_a_b,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::B, false));
    opcode!(sub_a_c,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::C, false));
    opcode!(sub_a_d,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::D, false));
    opcode!(sub_a_e,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::E, false));
    opcode!(sub_a_h,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::H, false));
    opcode!(sub_a_l,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::L, false));
    opcode!(sub_a_hlptr, [dev, ec] sub_r8_r16ptr(dev, ec, RegisterR8::A, RegisterR16::HL, false));

    // subtract with carry flag
    // SUB r8, ?
    opcode!(sbc_a_u8,    [dev, ec] sub_r8_u8(dev, ec, RegisterR8::A, true));
    opcode!(sbc_a_a,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::A, true));
    opcode!(sbc_a_b,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::B, true));
    opcode!(sbc_a_c,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::C, true));
    opcode!(sbc_a_d,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::D, true));
    opcode!(sbc_a_e,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::E, true));
    opcode!(sbc_a_h,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::H, true));
    opcode!(sbc_a_l,     [dev, ec] sub_r8_r8(dev, ec, RegisterR8::A, RegisterR8::L, true));
    opcode!(sbc_a_hlptr, [dev, ec] sub_r8_r16ptr(dev, ec, RegisterR8::A, RegisterR16::HL, true));
}


////////////////////////////////////////////////
//// RL / RLC opcodes
pub mod rl {
    use super::*;

    /// Shifts or rotates a value to the left.
    fn shift_left_u8v(dev: &mut EmulatorDevice, value: u8, op: ShiftOp) -> u8 {
        shift_left_u8v_nc(dev, value, op, NullCheck::Check)
    }

    /// Shifts or rotates a value to the left.
    fn shift_left_u8v_nc(dev: &mut EmulatorDevice, value: u8, op: ShiftOp, nullcheck: NullCheck) -> u8 {
        let carry    = dev.cpu.is_flag_set(CpuFlag::Carry) as u8;
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

        dev.cpu.set_flag(CpuFlag::Zero, null_bit);
        dev.cpu.set_flag(CpuFlag::Negative, false);
        dev.cpu.set_flag(CpuFlag::HalfCarry, false);
        dev.cpu.set_flag(CpuFlag::Carry, left_bit != 0);

        result
    }

    /// Shifts or rotates a value on a 16bit pointer to the left.
    fn shift_left_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16ptr: RegisterR16, op: ShiftOp) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = dev.cpu.get_r16(r16ptr);
                let value   = dev.get_mmu().read_u8(ec, address);
                dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = dev.cpu.get_r16(r16ptr);
                let value   = dev.cpu.get_intermediate_value();
                let result  = shift_left_u8v(dev, value, op);
                dev.get_mmu_mut().write_u8(ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }

    /// Performs an arithmetic shift left of the value of a register.
    fn sla_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        let value  = dev.cpu.get_r8(r8);
        let result = shift_left_u8v(dev, value, ShiftOp::ShiftArithmetic);
        dev.cpu.set_r8(r8, result);
    }

    /// Performs an arithmetic shift left of the value on a memory location.
    fn sla_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_left_r16ptr(dev, ec, ctx, r16ptr, ShiftOp::ShiftArithmetic)
    }

    /// Rotates the value of a register to the left through the carry flag.
    fn rl_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        rl_r8_nc(dev, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the left through the carry flag.
    fn rl_r8_nc(dev: &mut EmulatorDevice, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = dev.cpu.get_r8(r8);
        let result = shift_left_u8v_nc(dev, value, ShiftOp::RotateThroughCarry, nullcheck);
        dev.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the left through the carry flag.
    fn rl_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_left_r16ptr(dev, ec, ctx, r16ptr, ShiftOp::RotateThroughCarry)
    }

    /// Rotates the value of a register to the left.
    fn rlc_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        rlc_r8_nc(dev, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the left.
    fn rlc_r8_nc(dev: &mut EmulatorDevice, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = dev.cpu.get_r8(r8);
        let result = shift_left_u8v_nc(dev, value, ShiftOp::Rotate, nullcheck);
        dev.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the left.
    fn rlc_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_left_r16ptr(dev, ec, ctx, r16ptr, ShiftOp::Rotate)
    }


    // arithmetic shift left
    opcode!(sla_a,     [dev] sla_r8(dev, RegisterR8::A));
    opcode!(sla_b,     [dev] sla_r8(dev, RegisterR8::B));
    opcode!(sla_c,     [dev] sla_r8(dev, RegisterR8::C));
    opcode!(sla_d,     [dev] sla_r8(dev, RegisterR8::D));
    opcode!(sla_e,     [dev] sla_r8(dev, RegisterR8::E));
    opcode!(sla_h,     [dev] sla_r8(dev, RegisterR8::H));
    opcode!(sla_l,     [dev] sla_r8(dev, RegisterR8::L));

    // rotate left through carry flag
    opcode!(rla,       [dev] rl_r8_nc(dev, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rl_a,      [dev] rl_r8(dev, RegisterR8::A));
    opcode!(rl_b,      [dev] rl_r8(dev, RegisterR8::B));
    opcode!(rl_c,      [dev] rl_r8(dev, RegisterR8::C));
    opcode!(rl_d,      [dev] rl_r8(dev, RegisterR8::D));
    opcode!(rl_e,      [dev] rl_r8(dev, RegisterR8::E));
    opcode!(rl_h,      [dev] rl_r8(dev, RegisterR8::H));
    opcode!(rl_l,      [dev] rl_r8(dev, RegisterR8::L));

    // rotate left (carry flag just set)
    opcode!(rlca,      [dev] rlc_r8_nc(dev, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rlc_a,     [dev] rlc_r8(dev, RegisterR8::A));
    opcode!(rlc_b,     [dev] rlc_r8(dev, RegisterR8::B));
    opcode!(rlc_c,     [dev] rlc_r8(dev, RegisterR8::C));
    opcode!(rlc_d,     [dev] rlc_r8(dev, RegisterR8::D));
    opcode!(rlc_e,     [dev] rlc_r8(dev, RegisterR8::E));
    opcode!(rlc_h,     [dev] rlc_r8(dev, RegisterR8::H));
    opcode!(rlc_l,     [dev] rlc_r8(dev, RegisterR8::L));

    opcode!(sla_hlptr, [dev, ec, ctx] sla_r16ptr(dev, ec, ctx, RegisterR16::HL));
    opcode!(rl_hlptr,  [dev, ec, ctx] rl_r16ptr (dev, ec, ctx, RegisterR16::HL));
    opcode!(rlc_hlptr, [dev, ec, ctx] rlc_r16ptr(dev, ec, ctx, RegisterR16::HL));
}

////////////////////////////////////////////////
//// RR / RRC opcodes
pub mod rr {
    use super::*;

    /// Shifts or rotates a value to the right.
    fn shift_right_u8v(dev: &mut EmulatorDevice, value: u8, op: ShiftOp) -> u8 {
        shift_right_u8v_nc(dev, value, op, NullCheck::Check)
    }

    /// Shifts or rotates a value to the right.
    fn shift_right_u8v_nc(dev: &mut EmulatorDevice, value: u8, op: ShiftOp, nullcheck: NullCheck) -> u8 {
        let carry    = dev.cpu.is_flag_set(CpuFlag::Carry) as u8;
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

        dev.cpu.set_flag(CpuFlag::Zero, null_bit);
        dev.cpu.set_flag(CpuFlag::Negative, false);
        dev.cpu.set_flag(CpuFlag::HalfCarry, false);
        dev.cpu.set_flag(CpuFlag::Carry, right_bit != 0);

        result
    }

    /// Shifts or rotates a value on a 16bit pointer to the right.
    fn shift_right_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16ptr: RegisterR16, op: ShiftOp) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = dev.cpu.get_r16(r16ptr);
                let value   = dev.get_mmu().read_u8(ec, address);
                dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = dev.cpu.get_r16(r16ptr);
                let value   = dev.cpu.get_intermediate_value();
                let result  = shift_right_u8v(dev, value, op);
                dev.get_mmu_mut().write_u8(ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    /// Performs an arithmetic shift right of the value of a register.
    fn sra_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        let value  = dev.cpu.get_r8(r8);
        let result = shift_right_u8v(dev, value, ShiftOp::ShiftArithmetic);
        dev.cpu.set_r8(r8, result);
    }

    /// Performs an arithmetic shift right of the value on a memory location.
    fn sra_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(dev, ec, ctx, r16ptr, ShiftOp::ShiftArithmetic)
    }

    /// Performs an arithmetic shift right of the value of a register.
    fn srl_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        let value  = dev.cpu.get_r8(r8);
        let result = shift_right_u8v(dev, value, ShiftOp::ShiftLogical);
        dev.cpu.set_r8(r8, result);
    }

    /// Performs an arithmetic shift right of the value on a memory location.
    fn srl_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(dev, ec, ctx, r16ptr, ShiftOp::ShiftLogical)
    }

    /// Rotates the value of a register to the right through the carry flag.
    fn rr_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        rr_r8_nc(dev, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the right through the carry flag.
    fn rr_r8_nc(dev: &mut EmulatorDevice, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = dev.cpu.get_r8(r8);
        let result = shift_right_u8v_nc(dev, value, ShiftOp::RotateThroughCarry, nullcheck);
        dev.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the right through the carry flag.
    fn rr_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(dev, ec, ctx, r16ptr, ShiftOp::RotateThroughCarry)
    }

    /// Rotates the value of a register to the right.
    fn rrc_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        rrc_r8_nc(dev, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the right.
    fn rrc_r8_nc(dev: &mut EmulatorDevice, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = dev.cpu.get_r8(r8);
        let result = shift_right_u8v_nc(dev, value, ShiftOp::Rotate, nullcheck);
        dev.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the right.
    fn rrc_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(dev, ec, ctx, r16ptr, ShiftOp::Rotate)
    }


    // arithmetic shift right
    opcode!(sra_a,     [dev] sra_r8(dev, RegisterR8::A));
    opcode!(sra_b,     [dev] sra_r8(dev, RegisterR8::B));
    opcode!(sra_c,     [dev] sra_r8(dev, RegisterR8::C));
    opcode!(sra_d,     [dev] sra_r8(dev, RegisterR8::D));
    opcode!(sra_e,     [dev] sra_r8(dev, RegisterR8::E));
    opcode!(sra_h,     [dev] sra_r8(dev, RegisterR8::H));
    opcode!(sra_l,     [dev] sra_r8(dev, RegisterR8::L));

    // logical shift right
    opcode!(srl_a,     [dev] srl_r8(dev, RegisterR8::A));
    opcode!(srl_b,     [dev] srl_r8(dev, RegisterR8::B));
    opcode!(srl_c,     [dev] srl_r8(dev, RegisterR8::C));
    opcode!(srl_d,     [dev] srl_r8(dev, RegisterR8::D));
    opcode!(srl_e,     [dev] srl_r8(dev, RegisterR8::E));
    opcode!(srl_h,     [dev] srl_r8(dev, RegisterR8::H));
    opcode!(srl_l,     [dev] srl_r8(dev, RegisterR8::L));

    // rotate right through carry flag
    opcode!(rra,       [dev] rr_r8_nc(dev, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rr_a,      [dev] rr_r8(dev, RegisterR8::A));
    opcode!(rr_b,      [dev] rr_r8(dev, RegisterR8::B));
    opcode!(rr_c,      [dev] rr_r8(dev, RegisterR8::C));
    opcode!(rr_d,      [dev] rr_r8(dev, RegisterR8::D));
    opcode!(rr_e,      [dev] rr_r8(dev, RegisterR8::E));
    opcode!(rr_h,      [dev] rr_r8(dev, RegisterR8::H));
    opcode!(rr_l,      [dev] rr_r8(dev, RegisterR8::L));

    // rotate right (carry flag just set)
    opcode!(rrca,      [dev] rrc_r8_nc(dev, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rrc_a,     [dev] rrc_r8(dev, RegisterR8::A));
    opcode!(rrc_b,     [dev] rrc_r8(dev, RegisterR8::B));
    opcode!(rrc_c,     [dev] rrc_r8(dev, RegisterR8::C));
    opcode!(rrc_d,     [dev] rrc_r8(dev, RegisterR8::D));
    opcode!(rrc_e,     [dev] rrc_r8(dev, RegisterR8::E));
    opcode!(rrc_h,     [dev] rrc_r8(dev, RegisterR8::H));
    opcode!(rrc_l,     [dev] rrc_r8(dev, RegisterR8::L));

    opcode!(sra_hlptr, [dev, ec, ctx] sra_r16ptr(dev, ec, ctx, RegisterR16::HL));
    opcode!(srl_hlptr, [dev, ec, ctx] srl_r16ptr(dev, ec, ctx, RegisterR16::HL));
    opcode!(rr_hlptr,  [dev, ec, ctx] rr_r16ptr (dev, ec, ctx, RegisterR16::HL));
    opcode!(rrc_hlptr, [dev, ec, ctx] rrc_r16ptr(dev, ec, ctx, RegisterR16::HL));
}

////////////////////////////////////////////////
//// SWAP opcodes
pub mod swap {
    use super::*;

    /// Swaps the low and high nibble of a byte.
    fn swap_nibbles_u8v(dev: &mut EmulatorDevice, value: u8) -> u8 {
        let low   = (value >> 0) & 0x0f;
        let high  = (value >> 4) & 0x0f;
        let result = (low << 4) | (high);

        dev.cpu.clear_flags();
        dev.cpu.set_flag(CpuFlag::Zero, result == 0);

        result
    }

    /// Swaps the low and high nibble of a 8bit register.
    fn swap_r8(dev: &mut EmulatorDevice, r8: RegisterR8) {
        let value  = dev.cpu.get_r8(r8);
        let result = swap_nibbles_u8v(dev, value);
        dev.cpu.set_r8(r8, result);
    }

    /// Swaps the low and high nibble of a byte at the address of a 16bit register pointer.
    fn swap_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16_ptr: RegisterR16) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value   = dev.get_mmu().read_u8(ec, address);
                dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value   = dev.cpu.get_intermediate_value();
                let result  = swap_nibbles_u8v(dev, value);
                dev.get_mmu_mut().write_u8(ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    // swap low and high nibbles of registers
    opcode!(swap_a,     [dev] swap_r8(dev, RegisterR8::A));
    opcode!(swap_b,     [dev] swap_r8(dev, RegisterR8::B));
    opcode!(swap_c,     [dev] swap_r8(dev, RegisterR8::C));
    opcode!(swap_d,     [dev] swap_r8(dev, RegisterR8::D));
    opcode!(swap_e,     [dev] swap_r8(dev, RegisterR8::E));
    opcode!(swap_h,     [dev] swap_r8(dev, RegisterR8::H));
    opcode!(swap_l,     [dev] swap_r8(dev, RegisterR8::L));

    opcode!(swap_hlptr, [dev, ec, ctx] swap_r16ptr(dev, ec, ctx, RegisterR16::HL));
}

////////////////////////////////////////////////
//// Set Bit opcodes
pub mod set_bit {
    use super::*;

    /// Set bit n of a given 8bit value.
    /// value | (1 << bit)
    fn set_bit_u8v(_dev: &mut EmulatorDevice, value: u8, bit: u8) -> u8 {
        let result = value | (1 << bit);
        result
    }

    /// Set bit n of in the given register.
    /// r8 <- r8 | (1 << bit)
    fn set_bit_r8(dev: &mut EmulatorDevice, r8: RegisterR8, bit: u8) {
        let value  = dev.cpu.get_r8(r8);
        let result = set_bit_u8v(dev, value, bit);
        dev.cpu.set_r8(r8, result);
    }

    /// Set bit n on a memory address.
    /// (r16) <- (r16) | (1 << bit)
    fn set_bit_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16_ptr: RegisterR16, bit: u8) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value   = dev.get_mmu().read_u8(ec, address);
                dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value   = dev.cpu.get_intermediate_value();
                let result  = set_bit_u8v(dev, value, bit);
                dev.get_mmu_mut().write_u8(ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    opcode!(set_bit_0_a, [dev] set_bit_r8(dev, RegisterR8::A, 0));
    opcode!(set_bit_0_b, [dev] set_bit_r8(dev, RegisterR8::B, 0));
    opcode!(set_bit_0_c, [dev] set_bit_r8(dev, RegisterR8::C, 0));
    opcode!(set_bit_0_d, [dev] set_bit_r8(dev, RegisterR8::D, 0));
    opcode!(set_bit_0_e, [dev] set_bit_r8(dev, RegisterR8::E, 0));
    opcode!(set_bit_0_h, [dev] set_bit_r8(dev, RegisterR8::H, 0));
    opcode!(set_bit_0_l, [dev] set_bit_r8(dev, RegisterR8::L, 0));

    opcode!(set_bit_1_a, [dev] set_bit_r8(dev, RegisterR8::A, 1));
    opcode!(set_bit_1_b, [dev] set_bit_r8(dev, RegisterR8::B, 1));
    opcode!(set_bit_1_c, [dev] set_bit_r8(dev, RegisterR8::C, 1));
    opcode!(set_bit_1_d, [dev] set_bit_r8(dev, RegisterR8::D, 1));
    opcode!(set_bit_1_e, [dev] set_bit_r8(dev, RegisterR8::E, 1));
    opcode!(set_bit_1_h, [dev] set_bit_r8(dev, RegisterR8::H, 1));
    opcode!(set_bit_1_l, [dev] set_bit_r8(dev, RegisterR8::L, 1));

    opcode!(set_bit_2_a, [dev] set_bit_r8(dev, RegisterR8::A, 2));
    opcode!(set_bit_2_b, [dev] set_bit_r8(dev, RegisterR8::B, 2));
    opcode!(set_bit_2_c, [dev] set_bit_r8(dev, RegisterR8::C, 2));
    opcode!(set_bit_2_d, [dev] set_bit_r8(dev, RegisterR8::D, 2));
    opcode!(set_bit_2_e, [dev] set_bit_r8(dev, RegisterR8::E, 2));
    opcode!(set_bit_2_h, [dev] set_bit_r8(dev, RegisterR8::H, 2));
    opcode!(set_bit_2_l, [dev] set_bit_r8(dev, RegisterR8::L, 2));

    opcode!(set_bit_3_a, [dev] set_bit_r8(dev, RegisterR8::A, 3));
    opcode!(set_bit_3_b, [dev] set_bit_r8(dev, RegisterR8::B, 3));
    opcode!(set_bit_3_c, [dev] set_bit_r8(dev, RegisterR8::C, 3));
    opcode!(set_bit_3_d, [dev] set_bit_r8(dev, RegisterR8::D, 3));
    opcode!(set_bit_3_e, [dev] set_bit_r8(dev, RegisterR8::E, 3));
    opcode!(set_bit_3_h, [dev] set_bit_r8(dev, RegisterR8::H, 3));
    opcode!(set_bit_3_l, [dev] set_bit_r8(dev, RegisterR8::L, 3));

    opcode!(set_bit_4_a, [dev] set_bit_r8(dev, RegisterR8::A, 4));
    opcode!(set_bit_4_b, [dev] set_bit_r8(dev, RegisterR8::B, 4));
    opcode!(set_bit_4_c, [dev] set_bit_r8(dev, RegisterR8::C, 4));
    opcode!(set_bit_4_d, [dev] set_bit_r8(dev, RegisterR8::D, 4));
    opcode!(set_bit_4_e, [dev] set_bit_r8(dev, RegisterR8::E, 4));
    opcode!(set_bit_4_h, [dev] set_bit_r8(dev, RegisterR8::H, 4));
    opcode!(set_bit_4_l, [dev] set_bit_r8(dev, RegisterR8::L, 4));

    opcode!(set_bit_5_a, [dev] set_bit_r8(dev, RegisterR8::A, 5));
    opcode!(set_bit_5_b, [dev] set_bit_r8(dev, RegisterR8::B, 5));
    opcode!(set_bit_5_c, [dev] set_bit_r8(dev, RegisterR8::C, 5));
    opcode!(set_bit_5_d, [dev] set_bit_r8(dev, RegisterR8::D, 5));
    opcode!(set_bit_5_e, [dev] set_bit_r8(dev, RegisterR8::E, 5));
    opcode!(set_bit_5_h, [dev] set_bit_r8(dev, RegisterR8::H, 5));
    opcode!(set_bit_5_l, [dev] set_bit_r8(dev, RegisterR8::L, 5));

    opcode!(set_bit_6_a, [dev] set_bit_r8(dev, RegisterR8::A, 6));
    opcode!(set_bit_6_b, [dev] set_bit_r8(dev, RegisterR8::B, 6));
    opcode!(set_bit_6_c, [dev] set_bit_r8(dev, RegisterR8::C, 6));
    opcode!(set_bit_6_d, [dev] set_bit_r8(dev, RegisterR8::D, 6));
    opcode!(set_bit_6_e, [dev] set_bit_r8(dev, RegisterR8::E, 6));
    opcode!(set_bit_6_h, [dev] set_bit_r8(dev, RegisterR8::H, 6));
    opcode!(set_bit_6_l, [dev] set_bit_r8(dev, RegisterR8::L, 6));

    opcode!(set_bit_7_a, [dev] set_bit_r8(dev, RegisterR8::A, 7));
    opcode!(set_bit_7_b, [dev] set_bit_r8(dev, RegisterR8::B, 7));
    opcode!(set_bit_7_c, [dev] set_bit_r8(dev, RegisterR8::C, 7));
    opcode!(set_bit_7_d, [dev] set_bit_r8(dev, RegisterR8::D, 7));
    opcode!(set_bit_7_e, [dev] set_bit_r8(dev, RegisterR8::E, 7));
    opcode!(set_bit_7_h, [dev] set_bit_r8(dev, RegisterR8::H, 7));
    opcode!(set_bit_7_l, [dev] set_bit_r8(dev, RegisterR8::L, 7));

    opcode!(set_bit_0_hlptr, [dev, ec, ctx] set_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 0));
    opcode!(set_bit_1_hlptr, [dev, ec, ctx] set_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 1));
    opcode!(set_bit_2_hlptr, [dev, ec, ctx] set_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 2));
    opcode!(set_bit_3_hlptr, [dev, ec, ctx] set_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 3));
    opcode!(set_bit_4_hlptr, [dev, ec, ctx] set_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 4));
    opcode!(set_bit_5_hlptr, [dev, ec, ctx] set_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 5));
    opcode!(set_bit_6_hlptr, [dev, ec, ctx] set_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 6));
    opcode!(set_bit_7_hlptr, [dev, ec, ctx] set_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 7));
}

////////////////////////////////////////////////
//// Reset Bit opcodes
pub mod res_bit {
    use super::*;

    /// Resets bit n of a given 8bit value.
    /// value & !(1 << bit)
    fn res_bit_u8v(_dev: &mut EmulatorDevice, value: u8, bit: u8) -> u8 {
        let result = value & !(1 << bit);
        result
    }

    /// Resets bit n of in the given register.
    /// r8 <- r8 & !(1 << bit)
    fn res_bit_r8(dev: &mut EmulatorDevice, r8: RegisterR8, bit: u8) {
        let value  = dev.cpu.get_r8(r8);
        let result = res_bit_u8v(dev, value, bit);
        dev.cpu.set_r8(r8, result);
    }

    /// Resets bit n on a memory address.
    /// (r16) <- (r16) & !(1 << bit)
    fn res_bit_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, ctx: &mut OpCodeContext, r16_ptr: RegisterR16, bit: u8) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value   = dev.get_mmu().read_u8(ec, address);
                dev.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = dev.cpu.get_r16(r16_ptr);
                let value   = dev.cpu.get_intermediate_value();
                let result  = res_bit_u8v(dev, value, bit);
                dev.get_mmu_mut().write_u8(ec, address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    opcode!(res_bit_0_a, [dev] res_bit_r8(dev, RegisterR8::A, 0));
    opcode!(res_bit_0_b, [dev] res_bit_r8(dev, RegisterR8::B, 0));
    opcode!(res_bit_0_c, [dev] res_bit_r8(dev, RegisterR8::C, 0));
    opcode!(res_bit_0_d, [dev] res_bit_r8(dev, RegisterR8::D, 0));
    opcode!(res_bit_0_e, [dev] res_bit_r8(dev, RegisterR8::E, 0));
    opcode!(res_bit_0_h, [dev] res_bit_r8(dev, RegisterR8::H, 0));
    opcode!(res_bit_0_l, [dev] res_bit_r8(dev, RegisterR8::L, 0));

    opcode!(res_bit_1_a, [dev] res_bit_r8(dev, RegisterR8::A, 1));
    opcode!(res_bit_1_b, [dev] res_bit_r8(dev, RegisterR8::B, 1));
    opcode!(res_bit_1_c, [dev] res_bit_r8(dev, RegisterR8::C, 1));
    opcode!(res_bit_1_d, [dev] res_bit_r8(dev, RegisterR8::D, 1));
    opcode!(res_bit_1_e, [dev] res_bit_r8(dev, RegisterR8::E, 1));
    opcode!(res_bit_1_h, [dev] res_bit_r8(dev, RegisterR8::H, 1));
    opcode!(res_bit_1_l, [dev] res_bit_r8(dev, RegisterR8::L, 1));

    opcode!(res_bit_2_a, [dev] res_bit_r8(dev, RegisterR8::A, 2));
    opcode!(res_bit_2_b, [dev] res_bit_r8(dev, RegisterR8::B, 2));
    opcode!(res_bit_2_c, [dev] res_bit_r8(dev, RegisterR8::C, 2));
    opcode!(res_bit_2_d, [dev] res_bit_r8(dev, RegisterR8::D, 2));
    opcode!(res_bit_2_e, [dev] res_bit_r8(dev, RegisterR8::E, 2));
    opcode!(res_bit_2_h, [dev] res_bit_r8(dev, RegisterR8::H, 2));
    opcode!(res_bit_2_l, [dev] res_bit_r8(dev, RegisterR8::L, 2));

    opcode!(res_bit_3_a, [dev] res_bit_r8(dev, RegisterR8::A, 3));
    opcode!(res_bit_3_b, [dev] res_bit_r8(dev, RegisterR8::B, 3));
    opcode!(res_bit_3_c, [dev] res_bit_r8(dev, RegisterR8::C, 3));
    opcode!(res_bit_3_d, [dev] res_bit_r8(dev, RegisterR8::D, 3));
    opcode!(res_bit_3_e, [dev] res_bit_r8(dev, RegisterR8::E, 3));
    opcode!(res_bit_3_h, [dev] res_bit_r8(dev, RegisterR8::H, 3));
    opcode!(res_bit_3_l, [dev] res_bit_r8(dev, RegisterR8::L, 3));

    opcode!(res_bit_4_a, [dev] res_bit_r8(dev, RegisterR8::A, 4));
    opcode!(res_bit_4_b, [dev] res_bit_r8(dev, RegisterR8::B, 4));
    opcode!(res_bit_4_c, [dev] res_bit_r8(dev, RegisterR8::C, 4));
    opcode!(res_bit_4_d, [dev] res_bit_r8(dev, RegisterR8::D, 4));
    opcode!(res_bit_4_e, [dev] res_bit_r8(dev, RegisterR8::E, 4));
    opcode!(res_bit_4_h, [dev] res_bit_r8(dev, RegisterR8::H, 4));
    opcode!(res_bit_4_l, [dev] res_bit_r8(dev, RegisterR8::L, 4));

    opcode!(res_bit_5_a, [dev] res_bit_r8(dev, RegisterR8::A, 5));
    opcode!(res_bit_5_b, [dev] res_bit_r8(dev, RegisterR8::B, 5));
    opcode!(res_bit_5_c, [dev] res_bit_r8(dev, RegisterR8::C, 5));
    opcode!(res_bit_5_d, [dev] res_bit_r8(dev, RegisterR8::D, 5));
    opcode!(res_bit_5_e, [dev] res_bit_r8(dev, RegisterR8::E, 5));
    opcode!(res_bit_5_h, [dev] res_bit_r8(dev, RegisterR8::H, 5));
    opcode!(res_bit_5_l, [dev] res_bit_r8(dev, RegisterR8::L, 5));

    opcode!(res_bit_6_a, [dev] res_bit_r8(dev, RegisterR8::A, 6));
    opcode!(res_bit_6_b, [dev] res_bit_r8(dev, RegisterR8::B, 6));
    opcode!(res_bit_6_c, [dev] res_bit_r8(dev, RegisterR8::C, 6));
    opcode!(res_bit_6_d, [dev] res_bit_r8(dev, RegisterR8::D, 6));
    opcode!(res_bit_6_e, [dev] res_bit_r8(dev, RegisterR8::E, 6));
    opcode!(res_bit_6_h, [dev] res_bit_r8(dev, RegisterR8::H, 6));
    opcode!(res_bit_6_l, [dev] res_bit_r8(dev, RegisterR8::L, 6));

    opcode!(res_bit_7_a, [dev] res_bit_r8(dev, RegisterR8::A, 7));
    opcode!(res_bit_7_b, [dev] res_bit_r8(dev, RegisterR8::B, 7));
    opcode!(res_bit_7_c, [dev] res_bit_r8(dev, RegisterR8::C, 7));
    opcode!(res_bit_7_d, [dev] res_bit_r8(dev, RegisterR8::D, 7));
    opcode!(res_bit_7_e, [dev] res_bit_r8(dev, RegisterR8::E, 7));
    opcode!(res_bit_7_h, [dev] res_bit_r8(dev, RegisterR8::H, 7));
    opcode!(res_bit_7_l, [dev] res_bit_r8(dev, RegisterR8::L, 7));

    opcode!(res_bit_0_hlptr, [dev, ec, ctx] res_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 0));
    opcode!(res_bit_1_hlptr, [dev, ec, ctx] res_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 1));
    opcode!(res_bit_2_hlptr, [dev, ec, ctx] res_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 2));
    opcode!(res_bit_3_hlptr, [dev, ec, ctx] res_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 3));
    opcode!(res_bit_4_hlptr, [dev, ec, ctx] res_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 4));
    opcode!(res_bit_5_hlptr, [dev, ec, ctx] res_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 5));
    opcode!(res_bit_6_hlptr, [dev, ec, ctx] res_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 6));
    opcode!(res_bit_7_hlptr, [dev, ec, ctx] res_bit_r16ptr(dev, ec, ctx, RegisterR16::HL, 7));
}

////////////////////////////////////////////////
//// Check Bit opcodes
pub mod chk_bit {
    use super::*;

    /// Checks if bit n of a value is set.
    /// Set the Zero flag, if the bit was 0.
    fn check_bit_u8v(dev: &mut EmulatorDevice, value: u8, bit: u8) {
        let result = value & (1 << bit);
        dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        dev.cpu.set_flag(CpuFlag::Negative, false);
        dev.cpu.set_flag(CpuFlag::HalfCarry, true);
    }

    /// Checks if bit n of a register is set.
    /// Set the Zero flag, if the bit was 0.
    fn check_bit_r8(dev: &mut EmulatorDevice, r8: RegisterR8, bit: u8) {
        let value  = dev.cpu.get_r8(r8);
        check_bit_u8v(dev, value, bit);
    }

    /// Checks if bit n on a memory address is set.
    /// Set the Zero flag, if the bit was 0.
    fn check_bit_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, r16_ptr: RegisterR16, bit: u8) {
        let address = dev.cpu.get_r16(r16_ptr);
        let value   = dev.get_mmu().read_u8(ec, address);
        check_bit_u8v(dev, value, bit);
    }


    opcode!(check_bit_0_a,     [dev] check_bit_r8(dev, RegisterR8::A, 0));
    opcode!(check_bit_0_b,     [dev] check_bit_r8(dev, RegisterR8::B, 0));
    opcode!(check_bit_0_c,     [dev] check_bit_r8(dev, RegisterR8::C, 0));
    opcode!(check_bit_0_d,     [dev] check_bit_r8(dev, RegisterR8::D, 0));
    opcode!(check_bit_0_e,     [dev] check_bit_r8(dev, RegisterR8::E, 0));
    opcode!(check_bit_0_h,     [dev] check_bit_r8(dev, RegisterR8::H, 0));
    opcode!(check_bit_0_l,     [dev] check_bit_r8(dev, RegisterR8::L, 0));
    opcode!(check_bit_0_hlptr, [dev, ec] check_bit_r16ptr(dev, ec, RegisterR16::HL, 0));

    opcode!(check_bit_1_a,     [dev] check_bit_r8(dev, RegisterR8::A, 1));
    opcode!(check_bit_1_b,     [dev] check_bit_r8(dev, RegisterR8::B, 1));
    opcode!(check_bit_1_c,     [dev] check_bit_r8(dev, RegisterR8::C, 1));
    opcode!(check_bit_1_d,     [dev] check_bit_r8(dev, RegisterR8::D, 1));
    opcode!(check_bit_1_e,     [dev] check_bit_r8(dev, RegisterR8::E, 1));
    opcode!(check_bit_1_h,     [dev] check_bit_r8(dev, RegisterR8::H, 1));
    opcode!(check_bit_1_l,     [dev] check_bit_r8(dev, RegisterR8::L, 1));
    opcode!(check_bit_1_hlptr, [dev, ec] check_bit_r16ptr(dev, ec, RegisterR16::HL, 1));

    opcode!(check_bit_2_a,     [dev] check_bit_r8(dev, RegisterR8::A, 2));
    opcode!(check_bit_2_b,     [dev] check_bit_r8(dev, RegisterR8::B, 2));
    opcode!(check_bit_2_c,     [dev] check_bit_r8(dev, RegisterR8::C, 2));
    opcode!(check_bit_2_d,     [dev] check_bit_r8(dev, RegisterR8::D, 2));
    opcode!(check_bit_2_e,     [dev] check_bit_r8(dev, RegisterR8::E, 2));
    opcode!(check_bit_2_h,     [dev] check_bit_r8(dev, RegisterR8::H, 2));
    opcode!(check_bit_2_l,     [dev] check_bit_r8(dev, RegisterR8::L, 2));
    opcode!(check_bit_2_hlptr, [dev, ec] check_bit_r16ptr(dev, ec, RegisterR16::HL, 2));

    opcode!(check_bit_3_a,     [dev] check_bit_r8(dev, RegisterR8::A, 3));
    opcode!(check_bit_3_b,     [dev] check_bit_r8(dev, RegisterR8::B, 3));
    opcode!(check_bit_3_c,     [dev] check_bit_r8(dev, RegisterR8::C, 3));
    opcode!(check_bit_3_d,     [dev] check_bit_r8(dev, RegisterR8::D, 3));
    opcode!(check_bit_3_e,     [dev] check_bit_r8(dev, RegisterR8::E, 3));
    opcode!(check_bit_3_h,     [dev] check_bit_r8(dev, RegisterR8::H, 3));
    opcode!(check_bit_3_l,     [dev] check_bit_r8(dev, RegisterR8::L, 3));
    opcode!(check_bit_3_hlptr, [dev, ec] check_bit_r16ptr(dev, ec, RegisterR16::HL, 3));

    opcode!(check_bit_4_a,     [dev] check_bit_r8(dev, RegisterR8::A, 4));
    opcode!(check_bit_4_b,     [dev] check_bit_r8(dev, RegisterR8::B, 4));
    opcode!(check_bit_4_c,     [dev] check_bit_r8(dev, RegisterR8::C, 4));
    opcode!(check_bit_4_d,     [dev] check_bit_r8(dev, RegisterR8::D, 4));
    opcode!(check_bit_4_e,     [dev] check_bit_r8(dev, RegisterR8::E, 4));
    opcode!(check_bit_4_h,     [dev] check_bit_r8(dev, RegisterR8::H, 4));
    opcode!(check_bit_4_l,     [dev] check_bit_r8(dev, RegisterR8::L, 4));
    opcode!(check_bit_4_hlptr, [dev, ec] check_bit_r16ptr(dev, ec, RegisterR16::HL, 4));

    opcode!(check_bit_5_a,     [dev] check_bit_r8(dev, RegisterR8::A, 5));
    opcode!(check_bit_5_b,     [dev] check_bit_r8(dev, RegisterR8::B, 5));
    opcode!(check_bit_5_c,     [dev] check_bit_r8(dev, RegisterR8::C, 5));
    opcode!(check_bit_5_d,     [dev] check_bit_r8(dev, RegisterR8::D, 5));
    opcode!(check_bit_5_e,     [dev] check_bit_r8(dev, RegisterR8::E, 5));
    opcode!(check_bit_5_h,     [dev] check_bit_r8(dev, RegisterR8::H, 5));
    opcode!(check_bit_5_l,     [dev] check_bit_r8(dev, RegisterR8::L, 5));
    opcode!(check_bit_5_hlptr, [dev, ec] check_bit_r16ptr(dev, ec, RegisterR16::HL, 5));

    opcode!(check_bit_6_a,     [dev] check_bit_r8(dev, RegisterR8::A, 6));
    opcode!(check_bit_6_b,     [dev] check_bit_r8(dev, RegisterR8::B, 6));
    opcode!(check_bit_6_c,     [dev] check_bit_r8(dev, RegisterR8::C, 6));
    opcode!(check_bit_6_d,     [dev] check_bit_r8(dev, RegisterR8::D, 6));
    opcode!(check_bit_6_e,     [dev] check_bit_r8(dev, RegisterR8::E, 6));
    opcode!(check_bit_6_h,     [dev] check_bit_r8(dev, RegisterR8::H, 6));
    opcode!(check_bit_6_l,     [dev] check_bit_r8(dev, RegisterR8::L, 6));
    opcode!(check_bit_6_hlptr, [dev, ec] check_bit_r16ptr(dev, ec, RegisterR16::HL, 6));

    opcode!(check_bit_7_a,     [dev] check_bit_r8(dev, RegisterR8::A, 7));
    opcode!(check_bit_7_b,     [dev] check_bit_r8(dev, RegisterR8::B, 7));
    opcode!(check_bit_7_c,     [dev] check_bit_r8(dev, RegisterR8::C, 7));
    opcode!(check_bit_7_d,     [dev] check_bit_r8(dev, RegisterR8::D, 7));
    opcode!(check_bit_7_e,     [dev] check_bit_r8(dev, RegisterR8::E, 7));
    opcode!(check_bit_7_h,     [dev] check_bit_r8(dev, RegisterR8::H, 7));
    opcode!(check_bit_7_l,     [dev] check_bit_r8(dev, RegisterR8::L, 7));
    opcode!(check_bit_7_hlptr, [dev, ec] check_bit_r16ptr(dev, ec, RegisterR16::HL, 7));
}

////////////////////////////////////////////////
//// CP opcodes
pub mod cp {
    use super::*;

    /// Compares two values.
    fn cp_u8v_u8v(dev: &mut EmulatorDevice, value1: u8, value2: u8) {
        let (_, half_carry, carry) = carrying_sub_u8(value1, value2, false);

        dev.cpu.set_flag(CpuFlag::Zero, value1 == value2);
        dev.cpu.set_flag(CpuFlag::Negative, true);
        dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        dev.cpu.set_flag(CpuFlag::Carry, carry);
    }

    /// Compares two values.
    /// cp r8, u8
    fn cp_r8_u8(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, r8: RegisterR8) {
        let value1 = dev.cpu.get_r8(r8);
        let value2 = dev.cpu.fetch_u8(ec);
        cp_u8v_u8v(dev, value1, value2);
    }

    /// Compares two values.
    /// cp dst, src
    fn cp_r8_r8(dev: &mut EmulatorDevice, _ec: &mut impl EmulatorClientMut, dst: RegisterR8, src: RegisterR8) {
        let value1 = dev.cpu.get_r8(dst);
        let value2 = dev.cpu.get_r8(src);
        cp_u8v_u8v(dev, value1, value2);
    }

    /// Compares two values.
    /// cp dst, (src_ptr)
    fn cp_r8_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8, src_ptr: RegisterR16) {
        let value1  = dev.cpu.get_r8(dst);
        let address = dev.cpu.get_r16(src_ptr);
        let value2  = dev.get_mmu().read_u8(ec, address);
        cp_u8v_u8v(dev, value1, value2);
    }


    opcode!(cp_a_u8,    [dev, ec] cp_r8_u8(dev, ec, RegisterR8::A));
    opcode!(cp_a_a,     [dev, ec] cp_r8_r8(dev, ec, RegisterR8::A, RegisterR8::A));
    opcode!(cp_a_b,     [dev, ec] cp_r8_r8(dev, ec, RegisterR8::A, RegisterR8::B));
    opcode!(cp_a_c,     [dev, ec] cp_r8_r8(dev, ec, RegisterR8::A, RegisterR8::C));
    opcode!(cp_a_d,     [dev, ec] cp_r8_r8(dev, ec, RegisterR8::A, RegisterR8::D));
    opcode!(cp_a_e,     [dev, ec] cp_r8_r8(dev, ec, RegisterR8::A, RegisterR8::E));
    opcode!(cp_a_h,     [dev, ec] cp_r8_r8(dev, ec, RegisterR8::A, RegisterR8::H));
    opcode!(cp_a_l,     [dev, ec] cp_r8_r8(dev, ec, RegisterR8::A, RegisterR8::L));
    opcode!(cp_a_hlptr, [dev, ec] cp_r8_r16ptr(dev, ec, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// AND opcodes
pub mod and {
    use super::*;

    /// Computes a bitwise AND.
    /// r8 <- r8 & value
    fn and_r8_u8v(dev: &mut EmulatorDevice, r8: RegisterR8, value: u8) {
        let old_value = dev.cpu.get_r8(r8);
        let result    = old_value & value;
        dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        dev.cpu.set_flag(CpuFlag::Negative, false);
        dev.cpu.set_flag(CpuFlag::HalfCarry, true);
        dev.cpu.set_flag(CpuFlag::Carry, false);
        dev.cpu.set_r8(r8, result);
    }

    /// Computes a bitwise AND.
    /// dst <- dst & u8
    fn and_r8_u8(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8) {
        let value = dev.cpu.fetch_u8(ec);
        and_r8_u8v(dev, dst, value);
    }

    /// Computes a bitwise AND.
    /// dst <- dst & src
    fn and_r8_r8(dev: &mut EmulatorDevice, _ec: &mut impl EmulatorClientMut, dst: RegisterR8, src: RegisterR8) {
        let value = dev.cpu.get_r8(src);
        and_r8_u8v(dev, dst, value);
    }

    /// Computes a bitwise AND.
    /// dst <- dst & (src_ptr)
    fn and_r8_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8, src_ptr: RegisterR16) {
        let address = dev.cpu.get_r16(src_ptr);
        let value   = dev.get_mmu().read_u8(ec, address);
        and_r8_u8v(dev, dst, value);
    }


    opcode!(and_a_u8,    [dev, ec] and_r8_u8(dev, ec, RegisterR8::A));
    opcode!(and_a_a,     [dev, ec] and_r8_r8(dev, ec, RegisterR8::A, RegisterR8::A));
    opcode!(and_a_b,     [dev, ec] and_r8_r8(dev, ec, RegisterR8::A, RegisterR8::B));
    opcode!(and_a_c,     [dev, ec] and_r8_r8(dev, ec, RegisterR8::A, RegisterR8::C));
    opcode!(and_a_d,     [dev, ec] and_r8_r8(dev, ec, RegisterR8::A, RegisterR8::D));
    opcode!(and_a_e,     [dev, ec] and_r8_r8(dev, ec, RegisterR8::A, RegisterR8::E));
    opcode!(and_a_h,     [dev, ec] and_r8_r8(dev, ec, RegisterR8::A, RegisterR8::H));
    opcode!(and_a_l,     [dev, ec] and_r8_r8(dev, ec, RegisterR8::A, RegisterR8::L));
    opcode!(and_a_hlptr, [dev, ec] and_r8_r16ptr(dev, ec, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// OR opcodes
pub mod or {
    use super::*;

    /// Computes a bitwise OR.
    /// r8 <- r8 | value
    fn or_r8_u8v(dev: &mut EmulatorDevice, r8: RegisterR8, value: u8) {
        let old_value = dev.cpu.get_r8(r8);
        let result    = old_value | value;
        dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        dev.cpu.set_flag(CpuFlag::Negative, false);
        dev.cpu.set_flag(CpuFlag::HalfCarry, false);
        dev.cpu.set_flag(CpuFlag::Carry, false);
        dev.cpu.set_r8(r8, result);
    }

    /// Computes a bitwise OR.
    /// dst <- dst | u8
    fn or_r8_u8(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8) {
        let value = dev.cpu.fetch_u8(ec);
        or_r8_u8v(dev, dst, value);
    }

    /// Computes a bitwise OR.
    /// dst <- dst | src
    fn or_r8_r8(dev: &mut EmulatorDevice, _ec: &mut impl EmulatorClientMut, dst: RegisterR8, src: RegisterR8) {
        let value = dev.cpu.get_r8(src);
        or_r8_u8v(dev, dst, value);
    }

    /// Computes a bitwise OR.
    /// dst <- dst | (src_ptr)
    fn or_r8_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8, src_ptr: RegisterR16) {
        let address = dev.cpu.get_r16(src_ptr);
        let value   = dev.get_mmu().read_u8(ec, address);
        or_r8_u8v(dev, dst, value);
    }


    opcode!(or_a_u8,    [dev, ec] or_r8_u8(dev, ec, RegisterR8::A));
    opcode!(or_a_a,     [dev, ec] or_r8_r8(dev, ec, RegisterR8::A, RegisterR8::A));
    opcode!(or_a_b,     [dev, ec] or_r8_r8(dev, ec, RegisterR8::A, RegisterR8::B));
    opcode!(or_a_c,     [dev, ec] or_r8_r8(dev, ec, RegisterR8::A, RegisterR8::C));
    opcode!(or_a_d,     [dev, ec] or_r8_r8(dev, ec, RegisterR8::A, RegisterR8::D));
    opcode!(or_a_e,     [dev, ec] or_r8_r8(dev, ec, RegisterR8::A, RegisterR8::E));
    opcode!(or_a_h,     [dev, ec] or_r8_r8(dev, ec, RegisterR8::A, RegisterR8::H));
    opcode!(or_a_l,     [dev, ec] or_r8_r8(dev, ec, RegisterR8::A, RegisterR8::L));
    opcode!(or_a_hlptr, [dev, ec] or_r8_r16ptr(dev, ec, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// XOR opcodes
pub mod xor {
    use super::*;

    /// Computes a bitwise XOR.
    /// r8 <- r8 ^ value
    fn xor_r8_u8v(dev: &mut EmulatorDevice, r8: RegisterR8, value: u8) {
        let old_value = dev.cpu.get_r8(r8);
        let result    = old_value ^ value;
        dev.cpu.set_flag(CpuFlag::Zero, result == 0);
        dev.cpu.set_flag(CpuFlag::Negative, false);
        dev.cpu.set_flag(CpuFlag::HalfCarry, false);
        dev.cpu.set_flag(CpuFlag::Carry, false);
        dev.cpu.set_r8(r8, result);
    }

    /// Computes a bitwise XOR.
    /// dst <- dst ^ src
    fn xor_r8_r8(dev: &mut EmulatorDevice, _ec: &mut impl EmulatorClientMut, dst: RegisterR8, src: RegisterR8) {
        let value = dev.cpu.get_r8(src);
        xor_r8_u8v(dev, dst, value);
    }

    /// Computes a bitwise XOR.
    /// dst <- dst ^ (src_ptr)
    fn xor_r8_r16ptr(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, dst: RegisterR8, src_ptr: RegisterR16) {
        let address = dev.cpu.get_r16(src_ptr);
        let value   = dev.get_mmu().read_u8(ec, address);
        xor_r8_u8v(dev, dst, value);
    }


    fn xor_r8_u8(dev: &mut EmulatorDevice, ec: &mut impl EmulatorClientMut, r8: RegisterR8) {
        let value = dev.cpu.fetch_u8(ec);
        xor_r8_u8v(dev, r8, value);
    }


    opcode!(xor_a_u8,    [dev, ec] xor_r8_u8(dev, ec, RegisterR8::A));
    opcode!(xor_a_a,     [dev, ec] xor_r8_r8(dev, ec, RegisterR8::A, RegisterR8::A));
    opcode!(xor_a_b,     [dev, ec] xor_r8_r8(dev, ec, RegisterR8::A, RegisterR8::B));
    opcode!(xor_a_c,     [dev, ec] xor_r8_r8(dev, ec, RegisterR8::A, RegisterR8::C));
    opcode!(xor_a_d,     [dev, ec] xor_r8_r8(dev, ec, RegisterR8::A, RegisterR8::D));
    opcode!(xor_a_e,     [dev, ec] xor_r8_r8(dev, ec, RegisterR8::A, RegisterR8::E));
    opcode!(xor_a_h,     [dev, ec] xor_r8_r8(dev, ec, RegisterR8::A, RegisterR8::H));
    opcode!(xor_a_l,     [dev, ec] xor_r8_r8(dev, ec, RegisterR8::A, RegisterR8::L));
    opcode!(xor_a_hlptr, [dev, ec] xor_r8_r16ptr(dev, ec, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// other

// Convert a BCD Number.
opcode!(daa, [dev] {
    let mut a     = dev.cpu.get_r8(RegisterR8::A);
    let mut half  = dev.cpu.is_flag_set(CpuFlag::HalfCarry);
    let mut carry = dev.cpu.is_flag_set(CpuFlag::Carry);
    let mut tmp_a = a as u16;

    if dev.cpu.is_flag_set(CpuFlag::Negative) {
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

    dev.cpu.set_flag(CpuFlag::Zero,      a == 0);
    dev.cpu.set_flag(CpuFlag::HalfCarry, half);
    dev.cpu.set_flag(CpuFlag::Carry,     carry);

    dev.cpu.set_r8(RegisterR8::A, a);
});

// Complement
opcode!(cpl_a, [dev] {
    let value  = dev.cpu.get_r8(RegisterR8::A);
    let result = !value;
    dev.cpu.set_r8(RegisterR8::A, result);
    dev.cpu.set_flag(CpuFlag::Negative,  true);
    dev.cpu.set_flag(CpuFlag::HalfCarry, true);
});

// Set Carry flag.
opcode!(scf, [dev] {
    dev.cpu.set_flag(CpuFlag::Negative,  false);
    dev.cpu.set_flag(CpuFlag::HalfCarry, false);
    dev.cpu.set_flag(CpuFlag::Carry,     true);
});

// Change carry flag
opcode!(ccf, [dev] {
    dev.cpu.set_flag(CpuFlag::Negative,  false);
    dev.cpu.set_flag(CpuFlag::HalfCarry, false);
    dev.cpu.set_flag(CpuFlag::Carry,     !dev.cpu.is_flag_set(CpuFlag::Carry));
});

