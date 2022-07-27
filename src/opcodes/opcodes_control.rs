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

    let sp_new = if offset >= 0 {
        sp + (offset as u16)
    }
    else {
        sp - (-offset as u16)
    };

    gb.cpu.set_flags_by_result(sp as u32, sp_new as u32);
    gb.cpu.set_flag(CpuFlag::Zero,     false);
    gb.cpu.set_flag(CpuFlag::Negative, false);
    gb.cpu.set_stack_pointer(sp_new);
}
