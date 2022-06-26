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

use crate::{Cartridge, Cpu};
use crate::cpu::RegisterR8;
use crate::opcodes::ParamType::{None, U8};

type ProcessOpCode = fn(cpu: &mut Cpu, cartridge: &Cartridge);

/// Data types for opcode parameters.
#[derive(Copy, Clone)]
pub enum ParamType {
    None,
    U8,
    U16,
}

/// Data struct describing a single opcode.
#[derive(Copy, Clone)]
pub struct OpCode {
    pub name: &'static str,
    pub bytes: usize,
    pub cycles: usize,
    pub proc: ProcessOpCode,
    pub param1: ParamType,
}


/// Represents an invalid opcode.
/// This is intended as a placeholder for opcodes not yet implemented.
pub static OPCODE_INVALID: OpCode = OpCode { name: "???", bytes: 1, cycles: 0, proc: nop, param1: None };


/// The table of all supported opcodes.
/// The array's index is the opcodes numerical value.
pub static OPCODE_TABLE: [OpCode; 256] = [
    /* 0x00*/ OpCode { name: "NOP", bytes: 1, cycles: 0, proc: nop, param1: None },
    /* 0x01*/ OPCODE_INVALID,
    /* 0x02*/ OPCODE_INVALID,
    /* 0x03*/ OPCODE_INVALID,
    /* 0x04*/ OPCODE_INVALID,
    /* 0x05*/ OPCODE_INVALID,
    /* 0x06*/ OPCODE_INVALID,
    /* 0x07*/ OPCODE_INVALID,
    /* 0x08*/ OPCODE_INVALID,
    /* 0x09*/ OPCODE_INVALID,
    /* 0x0A*/ OPCODE_INVALID,
    /* 0x0B*/ OPCODE_INVALID,
    /* 0x0C*/ OPCODE_INVALID,
    /* 0x0D*/ OPCODE_INVALID,
    /* 0x0E*/ OPCODE_INVALID,
    /* 0x0F*/ OPCODE_INVALID,

    /* 0x10*/ OPCODE_INVALID,
    /* 0x11*/ OPCODE_INVALID,
    /* 0x12*/ OPCODE_INVALID,
    /* 0x13*/ OPCODE_INVALID,
    /* 0x14*/ OPCODE_INVALID,
    /* 0x15*/ OPCODE_INVALID,
    /* 0x16*/ OPCODE_INVALID,
    /* 0x17*/ OPCODE_INVALID,
    /* 0x18*/ OpCode { name: "JR", bytes: 2, cycles: 0, proc: jr_i8, param1: U8 },
    /* 0x19*/ OPCODE_INVALID,
    /* 0x1A*/ OPCODE_INVALID,
    /* 0x1B*/ OPCODE_INVALID,
    /* 0x1C*/ OPCODE_INVALID,
    /* 0x1D*/ OPCODE_INVALID,
    /* 0x1E*/ OPCODE_INVALID,
    /* 0x1F*/ OPCODE_INVALID,

    /* 0x20*/ OPCODE_INVALID,
    /* 0x21*/ OPCODE_INVALID,
    /* 0x22*/ OPCODE_INVALID,
    /* 0x23*/ OPCODE_INVALID,
    /* 0x24*/ OPCODE_INVALID,
    /* 0x25*/ OPCODE_INVALID,
    /* 0x26*/ OPCODE_INVALID,
    /* 0x27*/ OPCODE_INVALID,
    /* 0x28*/ OPCODE_INVALID,
    /* 0x29*/ OPCODE_INVALID,
    /* 0x2A*/ OPCODE_INVALID,
    /* 0x2B*/ OPCODE_INVALID,
    /* 0x2C*/ OPCODE_INVALID,
    /* 0x2D*/ OPCODE_INVALID,
    /* 0x2E*/ OPCODE_INVALID,
    /* 0x2F*/ OPCODE_INVALID,

    /* 0x30*/ OPCODE_INVALID,
    /* 0x31*/ OPCODE_INVALID,
    /* 0x32*/ OPCODE_INVALID,
    /* 0x33*/ OPCODE_INVALID,
    /* 0x34*/ OPCODE_INVALID,
    /* 0x35*/ OPCODE_INVALID,
    /* 0x36*/ OPCODE_INVALID,
    /* 0x37*/ OPCODE_INVALID,
    /* 0x38*/ OPCODE_INVALID,
    /* 0x39*/ OPCODE_INVALID,
    /* 0x3A*/ OPCODE_INVALID,
    /* 0x3B*/ OPCODE_INVALID,
    /* 0x3C*/ OPCODE_INVALID,
    /* 0x3D*/ OPCODE_INVALID,
    /* 0x3E*/ OPCODE_INVALID,
    /* 0x3F*/ OPCODE_INVALID,

    /* 0x40*/ OPCODE_INVALID,
    /* 0x41*/ OPCODE_INVALID,
    /* 0x42*/ OPCODE_INVALID,
    /* 0x43*/ OPCODE_INVALID,
    /* 0x44*/ OPCODE_INVALID,
    /* 0x45*/ OPCODE_INVALID,
    /* 0x46*/ OPCODE_INVALID,
    /* 0x47*/ OPCODE_INVALID,
    /* 0x48*/ OPCODE_INVALID,
    /* 0x49*/ OPCODE_INVALID,
    /* 0x4A*/ OPCODE_INVALID,
    /* 0x4B*/ OPCODE_INVALID,
    /* 0x4C*/ OPCODE_INVALID,
    /* 0x4D*/ OPCODE_INVALID,
    /* 0x4E*/ OPCODE_INVALID,
    /* 0x4F*/ OPCODE_INVALID,

    /* 0x50*/ OPCODE_INVALID,
    /* 0x51*/ OPCODE_INVALID,
    /* 0x52*/ OPCODE_INVALID,
    /* 0x53*/ OPCODE_INVALID,
    /* 0x54*/ OPCODE_INVALID,
    /* 0x55*/ OPCODE_INVALID,
    /* 0x56*/ OPCODE_INVALID,
    /* 0x57*/ OpCode { name: "LD D,A ", bytes: 1, cycles: 0, proc: ld_d_a, param1: None },
    /* 0x58*/ OPCODE_INVALID,
    /* 0x59*/ OPCODE_INVALID,
    /* 0x5A*/ OPCODE_INVALID,
    /* 0x5B*/ OPCODE_INVALID,
    /* 0x5C*/ OPCODE_INVALID,
    /* 0x5D*/ OPCODE_INVALID,
    /* 0x5E*/ OPCODE_INVALID,
    /* 0x5F*/ OPCODE_INVALID,

    /* 0x60*/ OPCODE_INVALID,
    /* 0x61*/ OPCODE_INVALID,
    /* 0x62*/ OPCODE_INVALID,
    /* 0x63*/ OPCODE_INVALID,
    /* 0x64*/ OPCODE_INVALID,
    /* 0x65*/ OPCODE_INVALID,
    /* 0x66*/ OPCODE_INVALID,
    /* 0x67*/ OPCODE_INVALID,
    /* 0x68*/ OPCODE_INVALID,
    /* 0x69*/ OPCODE_INVALID,
    /* 0x6A*/ OPCODE_INVALID,
    /* 0x6B*/ OPCODE_INVALID,
    /* 0x6C*/ OPCODE_INVALID,
    /* 0x6D*/ OPCODE_INVALID,
    /* 0x6E*/ OPCODE_INVALID,
    /* 0x6F*/ OPCODE_INVALID,

    /* 0x70*/ OPCODE_INVALID,
    /* 0x71*/ OPCODE_INVALID,
    /* 0x72*/ OPCODE_INVALID,
    /* 0x73*/ OPCODE_INVALID,
    /* 0x74*/ OPCODE_INVALID,
    /* 0x75*/ OPCODE_INVALID,
    /* 0x76*/ OPCODE_INVALID,
    /* 0x77*/ OPCODE_INVALID,
    /* 0x78*/ OPCODE_INVALID,
    /* 0x79*/ OPCODE_INVALID,
    /* 0x7A*/ OPCODE_INVALID,
    /* 0x7B*/ OPCODE_INVALID,
    /* 0x7C*/ OPCODE_INVALID,
    /* 0x7D*/ OPCODE_INVALID,
    /* 0x7E*/ OPCODE_INVALID,
    /* 0x7F*/ OPCODE_INVALID,

    /* 0x80*/ OPCODE_INVALID,
    /* 0x81*/ OPCODE_INVALID,
    /* 0x82*/ OPCODE_INVALID,
    /* 0x83*/ OPCODE_INVALID,
    /* 0x84*/ OPCODE_INVALID,
    /* 0x85*/ OPCODE_INVALID,
    /* 0x86*/ OPCODE_INVALID,
    /* 0x87*/ OPCODE_INVALID,
    /* 0x88*/ OPCODE_INVALID,
    /* 0x89*/ OPCODE_INVALID,
    /* 0x8A*/ OPCODE_INVALID,
    /* 0x8B*/ OPCODE_INVALID,
    /* 0x8C*/ OPCODE_INVALID,
    /* 0x8D*/ OPCODE_INVALID,
    /* 0x8E*/ OPCODE_INVALID,
    /* 0x8F*/ OPCODE_INVALID,

    /* 0x90*/ OPCODE_INVALID,
    /* 0x91*/ OPCODE_INVALID,
    /* 0x92*/ OPCODE_INVALID,
    /* 0x93*/ OPCODE_INVALID,
    /* 0x94*/ OPCODE_INVALID,
    /* 0x95*/ OPCODE_INVALID,
    /* 0x96*/ OPCODE_INVALID,
    /* 0x97*/ OPCODE_INVALID,
    /* 0x98*/ OPCODE_INVALID,
    /* 0x99*/ OPCODE_INVALID,
    /* 0x9A*/ OPCODE_INVALID,
    /* 0x9B*/ OPCODE_INVALID,
    /* 0x9C*/ OPCODE_INVALID,
    /* 0x9D*/ OPCODE_INVALID,
    /* 0x9E*/ OPCODE_INVALID,
    /* 0x9F*/ OPCODE_INVALID,

    /* 0xA0*/ OPCODE_INVALID,
    /* 0xA1*/ OPCODE_INVALID,
    /* 0xA2*/ OPCODE_INVALID,
    /* 0xA3*/ OPCODE_INVALID,
    /* 0xA4*/ OPCODE_INVALID,
    /* 0xA5*/ OPCODE_INVALID,
    /* 0xA6*/ OPCODE_INVALID,
    /* 0xA7*/ OPCODE_INVALID,
    /* 0xA8*/ OPCODE_INVALID,
    /* 0xA9*/ OPCODE_INVALID,
    /* 0xAA*/ OPCODE_INVALID,
    /* 0xAB*/ OPCODE_INVALID,
    /* 0xAC*/ OPCODE_INVALID,
    /* 0xAD*/ OPCODE_INVALID,
    /* 0xAE*/ OPCODE_INVALID,
    /* 0xAF*/ OPCODE_INVALID,

    /* 0xB0*/ OPCODE_INVALID,
    /* 0xB1*/ OPCODE_INVALID,
    /* 0xB2*/ OPCODE_INVALID,
    /* 0xB3*/ OPCODE_INVALID,
    /* 0xB4*/ OPCODE_INVALID,
    /* 0xB5*/ OPCODE_INVALID,
    /* 0xB6*/ OPCODE_INVALID,
    /* 0xB7*/ OPCODE_INVALID,
    /* 0xB8*/ OPCODE_INVALID,
    /* 0xB9*/ OPCODE_INVALID,
    /* 0xBA*/ OPCODE_INVALID,
    /* 0xBB*/ OPCODE_INVALID,
    /* 0xBC*/ OPCODE_INVALID,
    /* 0xBD*/ OPCODE_INVALID,
    /* 0xBE*/ OPCODE_INVALID,
    /* 0xBF*/ OPCODE_INVALID,

    /* 0xC0*/ OPCODE_INVALID,
    /* 0xC1*/ OPCODE_INVALID,
    /* 0xC2*/ OPCODE_INVALID,
    /* 0xC3*/ OPCODE_INVALID,
    /* 0xC4*/ OPCODE_INVALID,
    /* 0xC5*/ OPCODE_INVALID,
    /* 0xC6*/ OPCODE_INVALID,
    /* 0xC7*/ OPCODE_INVALID,
    /* 0xC8*/ OPCODE_INVALID,
    /* 0xC9*/ OPCODE_INVALID,
    /* 0xCA*/ OPCODE_INVALID,
    /* 0xCB*/ OPCODE_INVALID,
    /* 0xCC*/ OPCODE_INVALID,
    /* 0xCD*/ OPCODE_INVALID,
    /* 0xCE*/ OPCODE_INVALID,
    /* 0xCF*/ OPCODE_INVALID,

    /* 0xD0*/ OPCODE_INVALID,
    /* 0xD1*/ OPCODE_INVALID,
    /* 0xD2*/ OPCODE_INVALID,
    /* 0xD3*/ OPCODE_INVALID,
    /* 0xD4*/ OPCODE_INVALID,
    /* 0xD5*/ OPCODE_INVALID,
    /* 0xD6*/ OPCODE_INVALID,
    /* 0xD7*/ OPCODE_INVALID,
    /* 0xD8*/ OPCODE_INVALID,
    /* 0xD9*/ OPCODE_INVALID,
    /* 0xDA*/ OPCODE_INVALID,
    /* 0xDB*/ OPCODE_INVALID,
    /* 0xDC*/ OPCODE_INVALID,
    /* 0xDD*/ OPCODE_INVALID,
    /* 0xDE*/ OPCODE_INVALID,
    /* 0xDF*/ OPCODE_INVALID,

    /* 0xE0*/ OPCODE_INVALID,
    /* 0xE1*/ OPCODE_INVALID,
    /* 0xE2*/ OPCODE_INVALID,
    /* 0xE3*/ OPCODE_INVALID,
    /* 0xE4*/ OPCODE_INVALID,
    /* 0xE5*/ OPCODE_INVALID,
    /* 0xE6*/ OPCODE_INVALID,
    /* 0xE7*/ OPCODE_INVALID,
    /* 0xE8*/ OPCODE_INVALID,
    /* 0xE9*/ OPCODE_INVALID,
    /* 0xEA*/ OPCODE_INVALID,
    /* 0xEB*/ OPCODE_INVALID,
    /* 0xEC*/ OPCODE_INVALID,
    /* 0xED*/ OPCODE_INVALID,
    /* 0xEE*/ OPCODE_INVALID,
    /* 0xEF*/ OPCODE_INVALID,

    /* 0xF0*/ OPCODE_INVALID,
    /* 0xF1*/ OPCODE_INVALID,
    /* 0xF2*/ OPCODE_INVALID,
    /* 0xF3*/ OpCode { name: "DI", bytes: 1, cycles: 4, proc: disable_interrupts, param1: None },
    /* 0xF4*/ OPCODE_INVALID,
    /* 0xF5*/ OPCODE_INVALID,
    /* 0xF6*/ OPCODE_INVALID,
    /* 0xF7*/ OPCODE_INVALID,
    /* 0xF8*/ OPCODE_INVALID,
    /* 0xF9*/ OPCODE_INVALID,
    /* 0xFA*/ OPCODE_INVALID,
    /* 0xFB*/ OPCODE_INVALID,
    /* 0xFC*/ OPCODE_INVALID,
    /* 0xFD*/ OPCODE_INVALID,
    /* 0xFE*/ OPCODE_INVALID,
    /* 0xFF*/ OPCODE_INVALID,
];


fn nop(cpu: &mut Cpu, cartridge: &Cartridge) {}

fn jr_i8(cpu: &mut Cpu, cartridge: &Cartridge) {
    let offset = cpu.fetch_u8(cartridge);
    cpu.jump_relative(offset as u16);
}

fn ld_r8_r8(cpu: &mut Cpu, dst: RegisterR8, src: RegisterR8) {
    let value = cpu.get_r8(src);
    cpu.set_r8(dst, value);
}

fn ld_d_a(cpu: &mut Cpu, cartridge: &Cartridge) {
    ld_r8_r8(cpu, RegisterR8::D, RegisterR8::A);
}

fn disable_interrupts(cpu: &mut Cpu, cartridge: &Cartridge) {
}

fn enable_interrupts(cpu: &mut Cpu, cartridge: &Cartridge) {
}

