/*
 * Copyright (C) 2023 by Christian Fischer
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

use crate::graphic_data::DmgPalette;

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

    _unused_0x08: [u8; 7],

    /// IF: pending interrupts
    interrupts_flagged: u8,

    /// Sound control registers
    _sound0: [u8; 0x10],
    _sound1: [u8; 0x10],
    _sound2: [u8; 0x10],

    /// LCD control
    pub lcdc: u8,

    /// LCD status
    pub lcd_stat: u8,

    /// LCD scroll offset Y
    pub scy: u8,

    /// LCD scroll offset X
    pub scx: u8,

    /// LCD current line
    pub ly: u8,

    /// LCD current line comparison
    pub lyc: u8,

    /// OAM DMA transfer start address
    pub dma_address: u8,

    /// DMG background palette
    pub bgp: DmgPalette,

    /// DMG object palettes
    pub obp: [DmgPalette; 2],

    /// LCD window Y coordinate
    pub wy: u8,

    /// LCD window X coordinate
    pub wx: u8,

    _unused_0x4c: [u8; 3],

    /// GBC: VRAM bank select
    pub vbk: u8,

    pub boot_rom_disable: u8,

    /// CGB VRAM DMA transfer
    vram_dma: [u8; 5],

    _unused_0x56: [u8; 18],

    /// bit 0-6: address of the GBC background palette to read/write
    /// bit 7: auto increment the address on write
    pub bcps: u8,

    /// byte data to be read or written on the background palette memory
    pub bcpd: u8,

    /// bit 0-6: address of the GBC object palette to read/write
    /// bit 7: auto increment the address on write
    pub ocps: u8,

    /// byte data to be read or written on the object palette memory
    pub ocpd: u8,

    /// Object priority flag
    pub opri: u8,

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
    _unused_0xf0: [u8; 0x0f],

    /// IE: interrupts enabled
    pub interrupts_enabled: u8,
}


#[cfg(test)]
mod tests {
    use crate::memory::*;
    use crate::memory_data::mapped::MemoryDataMapped;
    use crate::memory_data::MemoryData;
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
        test_ioreg_struct_elem!(MEMORY_LOCATION_LCD_CONTROL         => lcdc);
        test_ioreg_struct_elem!(MEMORY_LOCATION_LCD_STATUS          => lcd_stat);
        test_ioreg_struct_elem!(MEMORY_LOCATION_SCY                 => scy);
        test_ioreg_struct_elem!(MEMORY_LOCATION_SCX                 => scx);
        test_ioreg_struct_elem!(MEMORY_LOCATION_LY                  => ly);
        test_ioreg_struct_elem!(MEMORY_LOCATION_LYC                 => lyc);
        test_ioreg_struct_elem!(MEMORY_LOCATION_DMA_ADDRESS         => dma_address);
        test_ioreg_struct_elem!(MEMORY_LOCATION_PALETTE_BG          => bgp);
        test_ioreg_struct_elem!(MEMORY_LOCATION_PALETTE_OBP0        => obp[0]);
        test_ioreg_struct_elem!(MEMORY_LOCATION_PALETTE_OBP1        => obp[1]);
        test_ioreg_struct_elem!(MEMORY_LOCATION_WY                  => wy);
        test_ioreg_struct_elem!(MEMORY_LOCATION_WX                  => wx);
        test_ioreg_struct_elem!(MEMORY_LOCATION_VBK                 => vbk);
        test_ioreg_struct_elem!(MEMORY_LOCATION_BCPS                => bcps);
        test_ioreg_struct_elem!(MEMORY_LOCATION_BCPD                => bcpd);
        test_ioreg_struct_elem!(MEMORY_LOCATION_OCPS                => ocps);
        test_ioreg_struct_elem!(MEMORY_LOCATION_OCPD                => ocpd);
        test_ioreg_struct_elem!(MEMORY_LOCATION_OPRI                => opri);
        test_ioreg_struct_elem!(MEMORY_LOCATION_BOOT_ROM_DISABLE    => boot_rom_disable);
        test_ioreg_struct_elem!(MEMORY_LOCATION_SVBK                => svbk);
        test_ioreg_struct_elem!(MEMORY_LOCATION_INTERRUPTS_FLAGGED  => interrupts_flagged);
        test_ioreg_struct_elem!(MEMORY_LOCATION_INTERRUPTS_ENABLED  => interrupts_enabled);
    }
}
