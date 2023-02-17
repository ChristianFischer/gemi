/*
 * Copyright (C) 2022-2023 by Christian Fischer
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

use crate::apu::registers::ApuRegisters;

/// Data struct containing raw data of all IO Registers in memory range 0xff00 to 0xffff.
/// This allows read/write operations without involving the memory bus. In real hardware,
/// most of those registers are no storage units, but read/write operations would directly
/// handled on their corresponding hardware unit.
/// Since our memory bus implementation does not send callbacks into PPU, APU, etc this
/// struct represents the data which would be available on the next read operation.
#[derive(Default)]
#[allow(dead_code)]
#[repr(packed(1))]
pub struct IoRegister {
    /// JoyPad input
    pub joyp: u8,

    /// Serial transfer data
    pub sb: u8,

    /// Serial transfer control
    pub sc: u8,

    _unused_0x03: u8,

    /// Divider register
    pub div: u8,

    /// Timer counter
    pub tima: u8,

    /// Timer modulo
    pub tma: u8,

    /// Timer control
    pub tac: u8,

    _unused_0x08: [u8; 8],

    /// Sound control registers
    pub apu: ApuRegisters,

    _unused_ppu_0x40: [u8; 12],

    _unused_0x4c: [u8; 3],

    _unused_vbk: u8,

    pub boot_rom_disable: u8,

    /// CGB VRAM DMA transfer
    vram_dma: [u8; 5],

    _unused_0x56: [u8; 18],

    _unused_ppu_0x68: [u8; 5],

    _unused_0x6d: [u8; 3],

    /// CGB WRAM bank select
    pub svbk: u8,

    _unused_0x71: [u8; 0x0f],
    _unused_0x80: [u8; 0x10],
    _unused_0x90: [u8; 0x10],
    _unused_0xa0: [u8; 0x10],
    _unused_0xb0: [u8; 0x10],
    _unused_0xc0: [u8; 0x10],
    _unused_0xd0: [u8; 0x10],
    _unused_0xe0: [u8; 0x10],
    _unused_0xf0: [u8; 0x10],
}


#[cfg(test)]
mod tests {
    use crate::mmu::locations::*;
    use crate::mmu::memory_data::mapped::MemoryDataMapped;
    use crate::mmu::memory_data::MemoryData;
    use super::*;

    macro_rules! test_ioreg_struct_elem {
        ($offset:expr => $($tokens:tt)+) => {
            {
                let mut bank = MemoryDataMapped::new(IoRegister::default());
                let offset = ($offset - 0xff00) as usize;
                let value = 0x12;

                bank.set_at(offset, value);
                assert_eq!(value, bank.get_at(offset));
                assert_eq!(value, bank.get().$($tokens)+.into());
            }
        }
    }

    #[test]
    fn test_ioreg_struct_size() {
        assert_eq!(256, std::mem::size_of::<IoRegister>());
    }

    #[test]
    fn test_ioreg_struct_locations() {
        test_ioreg_struct_elem!(MEMORY_LOCATION_JOYP                => joyp);
        test_ioreg_struct_elem!(MEMORY_LOCATION_SB                  => sb);
        test_ioreg_struct_elem!(MEMORY_LOCATION_SC                  => sc);
        test_ioreg_struct_elem!(MEMORY_LOCATION_REGISTER_DIV        => div);
        test_ioreg_struct_elem!(MEMORY_LOCATION_REGISTER_TIMA       => tima);
        test_ioreg_struct_elem!(MEMORY_LOCATION_REGISTER_TMA        => tma);
        test_ioreg_struct_elem!(MEMORY_LOCATION_REGISTER_TAC        => tac);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR10            => apu.channels[0].nr0);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR11            => apu.channels[0].nr1);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR12            => apu.channels[0].nr2);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR13            => apu.channels[0].nr3);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR14            => apu.channels[0].nr4);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR21            => apu.channels[1].nr1);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR22            => apu.channels[1].nr2);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR23            => apu.channels[1].nr3);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR24            => apu.channels[1].nr4);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR30            => apu.channels[2].nr0);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR31            => apu.channels[2].nr1);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR32            => apu.channels[2].nr2);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR33            => apu.channels[2].nr3);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR34            => apu.channels[2].nr4);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR41            => apu.channels[3].nr1);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR42            => apu.channels[3].nr2);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR43            => apu.channels[3].nr3);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR44            => apu.channels[3].nr4);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR50            => apu.nr50);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR51            => apu.nr51);
        test_ioreg_struct_elem!(MEMORY_LOCATION_APU_NR52            => apu.nr52);
        test_ioreg_struct_elem!(MEMORY_LOCATION_BOOT_ROM_DISABLE    => boot_rom_disable);
        test_ioreg_struct_elem!(MEMORY_LOCATION_SVBK                => svbk);
    }
}
