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

use crate::cpu::cpu::CpuFlag;
use crate::cpu::opcode::{opcode, OpCodeContext};
use crate::emulator_client::EmulatorClientMut;
use crate::emulator_device::EmulatorDevice;
use crate::utils::signed_overflow_add_u16;


opcode!(nop, []);

opcode!(stop, [] {
});

opcode!(halt, [dev] {
    dev.cpu.enter_halt_mode();
});

opcode!(enable_interrupts, [dev, _ec, ctx] {
    dev.cpu.enable_interrupts_in(ctx.get_opcode().cycles + 1);
});

opcode!(disable_interrupts, [dev] {
    dev.cpu.disable_interrupts();
});

opcode!(add_sp_i8, [dev, ec] {
    let offset = dev.cpu.fetch_i8(ec);
    let sp     = dev.cpu.get_stack_pointer();
    let (sp_new, _, _) = signed_overflow_add_u16(sp, offset as i16);

    let carry_bits = sp ^ sp_new ^ (offset as u16);
    let half_carry = (carry_bits & 0x0010) != 0;
    let carry      = (carry_bits & 0x0100) != 0;

    dev.cpu.set_flag(CpuFlag::Zero,      false);
    dev.cpu.set_flag(CpuFlag::Negative,  false);
    dev.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
    dev.cpu.set_flag(CpuFlag::Carry,     carry);
    dev.cpu.set_stack_pointer(sp_new);
});
