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

use core::cmp::max;

use crate::cartridge::Cartridge;
use crate::device_type::{DeviceConfig, EmulationType};
use crate::emulator_context::EmulatorContext;
use crate::mmu::locations::*;
use crate::mmu::mbc::{create_mbc, Mbc, MbcImpl, MemoryBankController};
use crate::mmu::memory_bus::{memory_map, MemoryBusConnection};
use crate::mmu::memory_data::{MemoryData, MemoryDataFixedSize};


/// Stores the information of an active OAM DMA transfer
/// The DMA transfer copies data from the given address
/// into OAM memory.
/// In total, 160 bytes will be transferred, so it takes
/// 160 cycles to transfer for the transfer to be completed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DmaTransferInfo {
    /// The address where to start copying the memory from.
    pub start_address: u16,

    /// The next byte to be copied.
    pub next_byte: u16,
}


/// State of the OAM DMA transfer, whether it be disabled or
/// in progress, including the time remaining.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DmaTransferState {
    /// No transfer is active.
    Disabled,

    /// A transfer is currently in progress.
    /// The attached struct stores information where
    /// the data should be taken from and how much data
    /// was already transferred.
    Transferring(DmaTransferInfo),
}


pub type WRamBank = MemoryDataFixedSize<4096>;
pub type HRamBank = MemoryDataFixedSize<127>;


/// The memory object is the owner of the emulator's memory.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Memory {
    /// The configuration of the running device
    #[deprecated(since = "0.1.0", note = "Use the EmulatorContext instead")]
    device_config: DeviceConfig,

    /// Work RAM banks (DMG = 2 * 4kiB, GBC = 8 * 4kiB)
    #[cfg(feature = "cgb")]
    wram_banks: Vec<WRamBank>,

    /// Work RAM banks (DMG only 2 * 4kiB)
    #[cfg(not(feature = "cgb"))]
    wram_banks: [WRamBank; 2],

    /// Active Work RAM banks.
    /// Bank 0 is fixed, Bank 1 can be switched between 1-7 on GBC.
    wram_active_bank_0: u8,
    wram_active_bank_1: u8,

    /// High RAM
    hram: HRamBank,

    /// MemoryBankController implementation.
    mbc: Mbc,

    /// Stores whether the boot rom is currently enabled or disabled.
    boot_rom_enabled: bool,
}


impl Memory {
    /// Create a new Memory object.
    pub fn new(ec: &EmulatorContext) -> Self {
        let mbc = Cartridge::create_from(ec.get_cartridge_rom())
                .map(|info| info.get_mbc().clone())
                .unwrap_or(MemoryBankController::None)
        ;

        let num_wram_banks = match ec.get_device_config().emulation {
            EmulationType::DMG => 2,
            EmulationType::GBC => 8,
        };

        #[cfg(not(feature = "cgb"))]
        {
            _ = num_wram_banks;
        }

        Self {
            device_config: ec.get_device_config().clone(),

            #[cfg(feature = "cgb")]
            wram_banks: core::iter::repeat_with(|| WRamBank::new()).take(num_wram_banks).collect(),

            #[cfg(not(feature = "cgb"))]
            wram_banks: [
                WRamBank::new(),
                WRamBank::new(),
            ],

            wram_active_bank_0: 0,
            wram_active_bank_1: 1,

            hram: HRamBank::new(),

            mbc: create_mbc(ec, &mbc),

            // start with enabled boot rom if one is available
            boot_rom_enabled: ec.get_boot_rom().is_some(),
        }
    }
}


impl Memory {
    /// Reads data from the boot rom, if any, otherwise from the cartridge.
    fn read_boot_rom_or_cartridge(&self, ec: &mut EmulatorContext, address: u16) -> u8 {
        if self.boot_rom_enabled {
            if let Some(boot_rom) = ec.get_boot_rom() {
                return boot_rom.read(address);
            }
        }

        self.read_from_cartridge(ec, address)
    }


    /// Reads data from the cartridge.
    fn read_from_cartridge(&self, ec: &mut EmulatorContext, address: u16) -> u8 {
        self.mbc.read_byte(ec, address)
    }


    /// Writes data to the cartridge.
    fn write_to_cartridge(&mut self, ec: &mut EmulatorContext, address: u16, value: u8) {
        self.mbc.write_byte(ec, address, value);
    }
}


impl MemoryBusConnection for Memory {
    fn on_read(&self, ec: &mut EmulatorContext, address: u16) -> u8 {
        memory_map!(
            address => {
                0x0000 ..= 0x00ff => [] self.read_boot_rom_or_cartridge(ec, address),
                0x0100 ..= 0x7fff => [] self.read_from_cartridge(ec, address),
                0xa000 ..= 0xbfff => [] self.read_from_cartridge(ec, address),

                0xc000 ..= 0xcfff => [mapped_address] {
                    let bank = &self.wram_banks[self.wram_active_bank_0 as usize];
                    bank.get_at(mapped_address)
                },

                0xd000 ..= 0xdfff => [mapped_address] {
                    let bank = &self.wram_banks[self.wram_active_bank_1 as usize];
                    bank.get_at(mapped_address)
                },

                0xe000 ..= 0xfdff => [mapped_address] {
                    // echo RAM; mapped into WRAM (0xc000 - 0xddff)
                    self.on_read(ec, (mapped_address + 0xc000) as u16)
                },

                0xfea0 ..= 0xfeff => [] {
                    // unusable ram area
                    0xff
                },

                0xff80 ..= 0xfffe => [mapped_address] {
                    self.hram.get_at(mapped_address)
                },

                // io registers
                0xff00 ..= 0xff7f => [] {
                    match address {
                        MEMORY_LOCATION_BOOT_ROM_DISABLE => {
                            if self.boot_rom_enabled {
                                0x00
                            }
                            else {
                                0xff
                            }
                        },

                        MEMORY_LOCATION_SVBK => {
                            // on GBC: WRAM bank #1
                            if let EmulationType::GBC = self.device_config.emulation {
                                self.wram_active_bank_1 | 0b_1111_1000
                            }
                            else {
                                0xff
                            }
                        },

                        _ => 0xff
                    }
                }
            }
        )
    }


    fn on_write(&mut self, ec: &mut EmulatorContext, address: u16, value: u8) {
        memory_map!(
            address => {
                0x0000 ..= 0x7fff => [] self.write_to_cartridge(ec, address, value),
                0xa000 ..= 0xbfff => [] self.write_to_cartridge(ec, address, value),

                0xc000 ..= 0xcfff => [mapped_address] {
                    let bank = &mut self.wram_banks[self.wram_active_bank_0 as usize];
                    bank.set_at(mapped_address, value)
                },

                0xd000 ..= 0xdfff => [mapped_address] {
                    let bank = &mut self.wram_banks[self.wram_active_bank_1 as usize];
                    bank.set_at(mapped_address, value)
                },

                0xe000 ..= 0xfdff => [mapped_address] {
                    // echo RAM; mapped into WRAM (0xc000 - 0xddff)
                    self.on_write(ec, (mapped_address + 0xc000) as u16, value)
                },

                0xfea0 ..= 0xfeff => [] { /* unusable ram area */ },

                0xff80 ..= 0xfffe => [mapped_address] {
                    self.hram.set_at(mapped_address, value)
                },

                // io registers
                0xff00 ..= 0xff7f => [] {
                    match address {
                        MEMORY_LOCATION_BOOT_ROM_DISABLE => {
                            // Boot ROM can only be disabled but never enabled
                            if (value & 0x01) != 0 {
                                self.boot_rom_enabled = false;
                            }
                        },

                        MEMORY_LOCATION_SVBK => {
                            // on GBC: switch WRAM bank #1
                            if let EmulationType::GBC = self.device_config.emulation {
                                let bank = value & 0x07;
                                self.wram_active_bank_1 = max(1, bank);
                            }
                        },

                        _ => { }
                    }
                }
            }
        )
    }
}
