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
use crate::cpu::opcode::{opcode, OpCodeContext};
use crate::utils::signed_overflow_add_u16;

/// Loads the content of a 8bit register into another one.
fn ld_r8_r8(ctx: &mut OpCodeContext, dst: RegisterR8, src: RegisterR8) {
    let value = ctx.dev.cpu.get_r8(src);
    ctx.dev.cpu.set_r8(dst, value);
}

/// Loads a constant 8bit value from the current instruction pointer into a 8bit register.
fn ld_r8_u8(ctx: &mut OpCodeContext, dst: RegisterR8) {
    let value = ctx.dev.cpu.fetch_u8(ctx.ec);
    ctx.dev.cpu.set_r8(dst, value);
}

/// Loads the content of a 8bit register into the device memory.
fn ld_addr_r8(ctx: &mut OpCodeContext, dst_address: u16, src: RegisterR8) {
    let value = ctx.dev.cpu.get_r8(src);
    ctx.dev.get_mmu_mut().write_u8(ctx.ec,  dst_address, value);
}

/// Writes a 8bit value to a given address in the device memory.
fn ld_addr_u8(ctx: &mut OpCodeContext, dst_address: u16, value: u8) {
    ctx.dev.get_mmu_mut().write_u8(ctx.ec,  dst_address, value);
}

/// Writes a 8bit value to a given address in the device memory.
fn ld_addr_u16(ctx: &mut OpCodeContext, dst_address: u16, value: u16) {
    ctx.dev.get_mmu_mut().write_u16(ctx.ec, dst_address, value);
}

/// Loads the value on a given address into a 8bit register.
fn ld_r8_addr(ctx: &mut OpCodeContext, dst: RegisterR8, src_address: u16) {
    let value = ctx.dev.get_mmu().read_u8(ctx.ec, src_address);
    ctx.dev.cpu.set_r8(dst, value);
}

/// Loads the content of a 8bit register into the address stored in the target R16 register.
fn ld_r16ptr_r8(ctx: &mut OpCodeContext, dst: RegisterR16, src: RegisterR8) {
    let address = ctx.dev.cpu.get_r16(dst);
    ld_addr_r8(ctx,  address, src);
}

/// Loads the content of a 8bit register into a constant address of the device memory.
fn ld_u16ptr_r8(ctx: &mut OpCodeContext, src: RegisterR8) {
    let address = ctx.dev.cpu.fetch_u16(ctx.ec);
    ld_addr_r8(ctx,  address, src);
}

/// Loads a constant 8bit value from the current instruction pointer
/// into the address stored in the target R16 register.
fn ld_r16ptr_u8(ctx: &mut OpCodeContext, dst: RegisterR16) {
    let address = ctx.dev.cpu.get_r16(dst);
    let value   = ctx.dev.cpu.fetch_u8(ctx.ec);
    ld_addr_u8(ctx, address, value);
}

/// Loads a 16bit value into a constant address of the device memory.
fn ld_u16ptr_u16v(ctx: &mut OpCodeContext, value: u16) {
    let address = ctx.dev.cpu.fetch_u16(ctx.ec);
    ld_addr_u16(ctx, address, value);
}

/// Loads the value at the address stored in a 16bit register
/// into a 8bit register.
fn ld_r8_r16ptr(ctx: &mut OpCodeContext, dst: RegisterR8, src: RegisterR16) {
    let address = ctx.dev.cpu.get_r16(src);
    ld_r8_addr(ctx, dst, address);
}

/// Loads the value at the address stored in a 16bit constant
/// into a 8bit register.
fn ld_r8_u16ptr(ctx: &mut OpCodeContext, dst: RegisterR8) {
    let address = ctx.dev.cpu.fetch_u16(ctx.ec, );
    ld_r8_addr(ctx, dst, address);
}

/// Loads a constant 16bit value from the current instruction pointer into a 16bit register.
fn ld_r16_u16(ctx: &mut OpCodeContext, dst: RegisterR16) {
    let value = ctx.dev.cpu.fetch_u16(ctx.ec, );
    ctx.dev.cpu.set_r16(dst, value);
}

/// Loads the value of a 8bit register into the device memory at the address (0xff00 + u8).
fn ldh_u8_r8(ctx: &mut OpCodeContext, src: RegisterR8) {
    let value     = ctx.dev.cpu.get_r8(src);
    let address_h = ctx.dev.cpu.fetch_u8(ctx.ec);
    let address   = 0xff00 | (address_h as u16);
    ctx.dev.get_mmu_mut().write_u8(ctx.ec,  address, value);
}

/// Loads a value from the device memory at the address (0xff00 + u8) into a 8bit register.
fn ldh_r8_u8(ctx: &mut OpCodeContext, dst: RegisterR8) {
    let address_h = ctx.dev.cpu.fetch_u8(ctx.ec);
    let address   = 0xff00 | (address_h as u16);
    let value     = ctx.dev.get_mmu().read_u8(ctx.ec, address);
    ctx.dev.cpu.set_r8(dst, value);
}

/// Loads a value from a 8bit register into the device memory at the address (0xff00 + r8)
fn ldh_r8ptr_r8(ctx: &mut OpCodeContext, dst_ptr: RegisterR8, src: RegisterR8) {
    let address_h = ctx.dev.cpu.get_r8(dst_ptr);
    let address   = 0xff00 | (address_h as u16);
    let value     = ctx.dev.cpu.get_r8(src);
    ctx.dev.get_mmu_mut().write_u8(ctx.ec,  address, value);
}

/// Loads a value from the device memory at the address (0xff00 + r8) into a 8bit register.
fn ldh_r8_r8ptr(ctx: &mut OpCodeContext, dst: RegisterR8, src_ptr: RegisterR8) {
    let address_h = ctx.dev.cpu.get_r8(src_ptr);
    let address   = 0xff00 | (address_h as u16);
    let value     = ctx.dev.get_mmu().read_u8(ctx.ec, address);
    ctx.dev.cpu.set_r8(dst, value);
}

/// Pushes the value of a 16bit register on the stack.
fn push_r16(ctx: &mut OpCodeContext, r16: RegisterR16) {
    let value = ctx.dev.cpu.get_r16(r16);
    ctx.dev.cpu.push_u16(ctx.ec, value);
}

/// Pops a 16bit value from the stack into a 16bit register.
fn pop_r16(ctx: &mut OpCodeContext, r16: RegisterR16) {
    let value = ctx.dev.cpu.pop_u16(ctx.ec);
    ctx.dev.cpu.set_r16(r16, value);
}

/// Pops a 16bit value from the stack into a 16bit register.
/// Applies a bitmask to the value before writing into the register.
fn pop_r16_mask(ctx: &mut OpCodeContext, r16: RegisterR16, mask: u16) {
    let value = ctx.dev.cpu.pop_u16(ctx.ec);
    let value_masked = value & mask;
    ctx.dev.cpu.set_r16(r16, value_masked);
}


// LD r8, r8
opcode!(ld_a_a, [ctx] ld_r8_r8(ctx, RegisterR8::A, RegisterR8::A));
opcode!(ld_a_b, [ctx] ld_r8_r8(ctx, RegisterR8::A, RegisterR8::B));
opcode!(ld_a_c, [ctx] ld_r8_r8(ctx, RegisterR8::A, RegisterR8::C));
opcode!(ld_a_d, [ctx] ld_r8_r8(ctx, RegisterR8::A, RegisterR8::D));
opcode!(ld_a_e, [ctx] ld_r8_r8(ctx, RegisterR8::A, RegisterR8::E));
opcode!(ld_a_l, [ctx] ld_r8_r8(ctx, RegisterR8::A, RegisterR8::L));
opcode!(ld_a_h, [ctx] ld_r8_r8(ctx, RegisterR8::A, RegisterR8::H));

// LD r8, r8
opcode!(ld_b_a, [ctx] ld_r8_r8(ctx, RegisterR8::B, RegisterR8::A));
opcode!(ld_b_b, [ctx] ld_r8_r8(ctx, RegisterR8::B, RegisterR8::B));
opcode!(ld_b_c, [ctx] ld_r8_r8(ctx, RegisterR8::B, RegisterR8::C));
opcode!(ld_b_d, [ctx] ld_r8_r8(ctx, RegisterR8::B, RegisterR8::D));
opcode!(ld_b_e, [ctx] ld_r8_r8(ctx, RegisterR8::B, RegisterR8::E));
opcode!(ld_b_l, [ctx] ld_r8_r8(ctx, RegisterR8::B, RegisterR8::L));
opcode!(ld_b_h, [ctx] ld_r8_r8(ctx, RegisterR8::B, RegisterR8::H));

// LD r8, r8
opcode!(ld_c_a, [ctx] ld_r8_r8(ctx, RegisterR8::C, RegisterR8::A));
opcode!(ld_c_b, [ctx] ld_r8_r8(ctx, RegisterR8::C, RegisterR8::B));
opcode!(ld_c_c, [ctx] ld_r8_r8(ctx, RegisterR8::C, RegisterR8::C));
opcode!(ld_c_d, [ctx] ld_r8_r8(ctx, RegisterR8::C, RegisterR8::D));
opcode!(ld_c_e, [ctx] ld_r8_r8(ctx, RegisterR8::C, RegisterR8::E));
opcode!(ld_c_l, [ctx] ld_r8_r8(ctx, RegisterR8::C, RegisterR8::L));
opcode!(ld_c_h, [ctx] ld_r8_r8(ctx, RegisterR8::C, RegisterR8::H));

// LD r8, r8
opcode!(ld_d_a, [ctx] ld_r8_r8(ctx, RegisterR8::D, RegisterR8::A));
opcode!(ld_d_b, [ctx] ld_r8_r8(ctx, RegisterR8::D, RegisterR8::B));
opcode!(ld_d_c, [ctx] ld_r8_r8(ctx, RegisterR8::D, RegisterR8::C));
opcode!(ld_d_d, [ctx] ld_r8_r8(ctx, RegisterR8::D, RegisterR8::D));
opcode!(ld_d_e, [ctx] ld_r8_r8(ctx, RegisterR8::D, RegisterR8::E));
opcode!(ld_d_l, [ctx] ld_r8_r8(ctx, RegisterR8::D, RegisterR8::L));
opcode!(ld_d_h, [ctx] ld_r8_r8(ctx, RegisterR8::D, RegisterR8::H));

// LD r8, r8
opcode!(ld_e_a, [ctx] ld_r8_r8(ctx, RegisterR8::E, RegisterR8::A));
opcode!(ld_e_b, [ctx] ld_r8_r8(ctx, RegisterR8::E, RegisterR8::B));
opcode!(ld_e_c, [ctx] ld_r8_r8(ctx, RegisterR8::E, RegisterR8::C));
opcode!(ld_e_d, [ctx] ld_r8_r8(ctx, RegisterR8::E, RegisterR8::D));
opcode!(ld_e_e, [ctx] ld_r8_r8(ctx, RegisterR8::E, RegisterR8::E));
opcode!(ld_e_l, [ctx] ld_r8_r8(ctx, RegisterR8::E, RegisterR8::L));
opcode!(ld_e_h, [ctx] ld_r8_r8(ctx, RegisterR8::E, RegisterR8::H));

// LD r8, r8
opcode!(ld_l_a, [ctx] ld_r8_r8(ctx, RegisterR8::L, RegisterR8::A));
opcode!(ld_l_b, [ctx] ld_r8_r8(ctx, RegisterR8::L, RegisterR8::B));
opcode!(ld_l_c, [ctx] ld_r8_r8(ctx, RegisterR8::L, RegisterR8::C));
opcode!(ld_l_d, [ctx] ld_r8_r8(ctx, RegisterR8::L, RegisterR8::D));
opcode!(ld_l_e, [ctx] ld_r8_r8(ctx, RegisterR8::L, RegisterR8::E));
opcode!(ld_l_l, [ctx] ld_r8_r8(ctx, RegisterR8::L, RegisterR8::L));
opcode!(ld_l_h, [ctx] ld_r8_r8(ctx, RegisterR8::L, RegisterR8::H));

// LD r8, r8
opcode!(ld_h_a, [ctx] ld_r8_r8(ctx, RegisterR8::H, RegisterR8::A));
opcode!(ld_h_b, [ctx] ld_r8_r8(ctx, RegisterR8::H, RegisterR8::B));
opcode!(ld_h_c, [ctx] ld_r8_r8(ctx, RegisterR8::H, RegisterR8::C));
opcode!(ld_h_d, [ctx] ld_r8_r8(ctx, RegisterR8::H, RegisterR8::D));
opcode!(ld_h_e, [ctx] ld_r8_r8(ctx, RegisterR8::H, RegisterR8::E));
opcode!(ld_h_l, [ctx] ld_r8_r8(ctx, RegisterR8::H, RegisterR8::L));
opcode!(ld_h_h, [ctx] ld_r8_r8(ctx, RegisterR8::H, RegisterR8::H));

// LD r8, u8
opcode!(ld_a_u8, [ctx] ld_r8_u8(ctx, RegisterR8::A));
opcode!(ld_b_u8, [ctx] ld_r8_u8(ctx, RegisterR8::B));
opcode!(ld_c_u8, [ctx] ld_r8_u8(ctx, RegisterR8::C));
opcode!(ld_d_u8, [ctx] ld_r8_u8(ctx, RegisterR8::D));
opcode!(ld_e_u8, [ctx] ld_r8_u8(ctx, RegisterR8::E));
opcode!(ld_h_u8, [ctx] ld_r8_u8(ctx, RegisterR8::H));
opcode!(ld_l_u8, [ctx] ld_r8_u8(ctx, RegisterR8::L));

// LD r16, u16
opcode!(ld_bc_u16, [ctx] ld_r16_u16(ctx, RegisterR16::BC));
opcode!(ld_de_u16, [ctx] ld_r16_u16(ctx, RegisterR16::DE));
opcode!(ld_hl_u16, [ctx] ld_r16_u16(ctx, RegisterR16::HL));

// LD r8, (u16)
opcode!(ld_a_u16ptr, [ctx] ld_r8_u16ptr(ctx, RegisterR8::A));

// LD r8, (r16)
opcode!(ld_a_bcptr, [ctx] ld_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::BC));
opcode!(ld_a_deptr, [ctx] ld_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::DE));
opcode!(ld_a_hlptr, [ctx] ld_r8_r16ptr(ctx, RegisterR8::A, RegisterR16::HL));
opcode!(ld_b_hlptr, [ctx] ld_r8_r16ptr(ctx, RegisterR8::B, RegisterR16::HL));
opcode!(ld_c_hlptr, [ctx] ld_r8_r16ptr(ctx, RegisterR8::C, RegisterR16::HL));
opcode!(ld_d_hlptr, [ctx] ld_r8_r16ptr(ctx, RegisterR8::D, RegisterR16::HL));
opcode!(ld_e_hlptr, [ctx] ld_r8_r16ptr(ctx, RegisterR8::E, RegisterR16::HL));
opcode!(ld_h_hlptr, [ctx] ld_r8_r16ptr(ctx, RegisterR8::H, RegisterR16::HL));
opcode!(ld_l_hlptr, [ctx] ld_r8_r16ptr(ctx, RegisterR8::L, RegisterR16::HL));

// LD (r16), r8
opcode!(ld_bcptr_a, [ctx] ld_r16ptr_r8(ctx, RegisterR16::BC, RegisterR8::A));
opcode!(ld_deptr_a, [ctx] ld_r16ptr_r8(ctx, RegisterR16::DE, RegisterR8::A));
opcode!(ld_hlptr_a, [ctx] ld_r16ptr_r8(ctx, RegisterR16::HL, RegisterR8::A));
opcode!(ld_hlptr_b, [ctx] ld_r16ptr_r8(ctx, RegisterR16::HL, RegisterR8::B));
opcode!(ld_hlptr_c, [ctx] ld_r16ptr_r8(ctx, RegisterR16::HL, RegisterR8::C));
opcode!(ld_hlptr_d, [ctx] ld_r16ptr_r8(ctx, RegisterR16::HL, RegisterR8::D));
opcode!(ld_hlptr_e, [ctx] ld_r16ptr_r8(ctx, RegisterR16::HL, RegisterR8::E));
opcode!(ld_hlptr_h, [ctx] ld_r16ptr_r8(ctx, RegisterR16::HL, RegisterR8::H));
opcode!(ld_hlptr_l, [ctx] ld_r16ptr_r8(ctx, RegisterR16::HL, RegisterR8::L));

// LD A, (HL+)
opcode!(ld_a_hlptri, [ctx] {
    let hl = ctx.dev.cpu.get_r16(RegisterR16::HL);
    ld_r8_addr(ctx, RegisterR8::A, hl);
    ctx.dev.cpu.set_r16(RegisterR16::HL, hl + 1);
});

// LD A, (HL-)
opcode!(ld_a_hlptrd, [ctx] {
    let hl = ctx.dev.cpu.get_r16(RegisterR16::HL);
    ld_r8_addr(ctx, RegisterR8::A, hl);
    ctx.dev.cpu.set_r16(RegisterR16::HL, hl - 1);
});

// LD (HL+), A
opcode!(ld_hlptri_a, [ctx] {
    let hl = ctx.dev.cpu.get_r16(RegisterR16::HL);
    ld_addr_r8(ctx,  hl, RegisterR8::A);
    ctx.dev.cpu.set_r16(RegisterR16::HL, hl + 1);
});

// LD (HL-), A
opcode!(ld_hlptrd_a, [ctx] {
    let hl = ctx.dev.cpu.get_r16(RegisterR16::HL);
    ld_addr_r8(ctx,  hl, RegisterR8::A);
    ctx.dev.cpu.set_r16(RegisterR16::HL, hl - 1);
});

// LD (r16), u8
opcode!(ld_hlptr_u8,  [ctx] ld_r16ptr_u8(ctx, RegisterR16::HL));

// LD (u16), A
opcode!(ld_u16ptr_a,  [ctx] ld_u16ptr_r8(ctx, RegisterR8::A));

// LD (u16), SP
opcode!(ld_u16ptr_sp, [ctx] ld_u16ptr_u16v(ctx, ctx.dev.cpu.get_stack_pointer()));

// LDH (0xff00 + u8), A // LDH A, (0xff00 + u8)
opcode!(ldh_u8_a, [ctx] ldh_u8_r8(ctx, RegisterR8::A));
opcode!(ldh_a_u8, [ctx] ldh_r8_u8(ctx, RegisterR8::A));

// LDH (0xff00 + C), A // LDH A, (0xff00 + C)
opcode!(ldh_cptr_a, [ctx] ldh_r8ptr_r8(ctx, RegisterR8::C, RegisterR8::A));
opcode!(ldh_a_cptr, [ctx] ldh_r8_r8ptr(ctx, RegisterR8::A, RegisterR8::C));

// LD HL, SP+i8
opcode!(ld_hl_sp_i8, [ctx] {
    let offset = ctx.dev.cpu.fetch_i8(ctx.ec);
    let sp     = ctx.dev.cpu.get_stack_pointer();
    let (sp_new, _, _) = signed_overflow_add_u16(sp, offset as i16);

    let carry_bits = sp ^ sp_new ^ (offset as u16);
    let half_carry = (carry_bits & 0x0010) != 0;
    let carry      = (carry_bits & 0x0100) != 0;

    ctx.dev.cpu.set_flag(CpuFlag::Zero,      false);
    ctx.dev.cpu.set_flag(CpuFlag::Negative,  false);
    ctx.dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
    ctx.dev.cpu.set_flag(CpuFlag::Carry,     carry);
    ctx.dev.cpu.set_r16(RegisterR16::HL, sp_new);
});

// LD SP, u16
opcode!(ld_sp_u16, [ctx] {
    let value = ctx.dev.cpu.fetch_u16(ctx.ec, );
    ctx.dev.cpu.set_stack_pointer(value);
});

// LD SP, HL
opcode!(ld_sp_hl, [ctx] {
    let value = ctx.dev.cpu.get_r16(RegisterR16::HL);
    ctx.dev.cpu.set_stack_pointer(value);
});

// PUSH r16
opcode!(push_af, [ctx] push_r16(ctx, RegisterR16::AF));
opcode!(push_bc, [ctx] push_r16(ctx, RegisterR16::BC));
opcode!(push_de, [ctx] push_r16(ctx, RegisterR16::DE));
opcode!(push_hl, [ctx] push_r16(ctx, RegisterR16::HL));

// POP r16
opcode!(pop_af, [ctx] pop_r16_mask(ctx, RegisterR16::AF, 0xfff0));
opcode!(pop_bc, [ctx] pop_r16(ctx, RegisterR16::BC));
opcode!(pop_de, [ctx] pop_r16(ctx, RegisterR16::DE));
opcode!(pop_hl, [ctx] pop_r16(ctx, RegisterR16::HL));
