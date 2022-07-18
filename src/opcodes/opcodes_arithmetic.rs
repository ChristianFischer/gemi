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

fn cp_r8_u8(gb: &mut GameBoy, r8: RegisterR8) {
    //let value1 = gb.cpu.get_r8(r8);
    let _ = gb.cpu.fetch_u8();
}

fn xor_r8_u8(gb: &mut GameBoy, r8: RegisterR8) {
    let value1 = gb.cpu.get_r8(r8);
    let value2 = gb.cpu.fetch_u8();
    let result = value1 ^ value2;
    gb.cpu.set_r8(r8, result);
}

pub fn cp_a_u8(gb: &mut GameBoy) {
    cp_r8_u8(gb, RegisterR8::A);
}

pub fn inc_a(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::A);
}

pub fn dec_a(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::A);
}

pub fn inc_b(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::B);
}

pub fn dec_b(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::B);
}

pub fn inc_c(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::C);
}

pub fn dec_c(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::C);
}

pub fn inc_d(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::D);
}

pub fn dec_d(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::D);
}

pub fn inc_e(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::E);
}

pub fn dec_e(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::E);
}

pub fn inc_h(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::H);
}

pub fn dec_h(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::H);
}

pub fn inc_l(gb: &mut GameBoy) {
    gb.cpu.increment_r8(RegisterR8::L);
}

pub fn dec_l(gb: &mut GameBoy) {
    gb.cpu.decrement_r8(RegisterR8::L);
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

pub fn inc_sp(gb: &mut GameBoy) {
    gb.cpu.increment_sp();
}

pub fn xor_a_u8(gb: &mut GameBoy) {
    xor_r8_u8(gb, RegisterR8::A);
}
