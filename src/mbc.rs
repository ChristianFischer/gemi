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

use std::fmt::{Display, Formatter};
use crate::Cartridge;
use crate::mbc::mbc1::Mbc1;
use crate::mbc::mbc5::Mbc5;
use crate::mbc::mbc_none::MbcNone;

/// Type of memory bank controller to be used
pub enum MemoryBankController {
    None,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
    MBC6,
    MBC7,
}


/// Trait for objects acting as memory bank controller.
pub trait Mbc {
    /// Read a single byte from the device memory.
    fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8;

    /// Write a single byte into the device memory.
    fn write_byte(&mut self, cartridge: &mut Cartridge, address: u16, value: u8);
}


/// Creates a memory bank controller object based on the type given.
pub fn create_mbc(kind: &MemoryBankController) -> Box<dyn Mbc> {
    match kind {
        MemoryBankController::None => Box::new(MbcNone::new()),
        MemoryBankController::MBC1 => Box::new(Mbc1::new()),
        MemoryBankController::MBC5 => Box::new(Mbc5::new()),
        _                          => panic!("Not implemented {}", kind)
    }
}


impl Display for MemoryBankController {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            MemoryBankController::None => "None",
            MemoryBankController::MBC1 => "MBC1",
            MemoryBankController::MBC2 => "MBC2",
            MemoryBankController::MBC3 => "MBC3",
            MemoryBankController::MBC5 => "MBC5",
            MemoryBankController::MBC6 => "MBC6",
            MemoryBankController::MBC7 => "MBC7",
        };

        write!(f, "{:}", name)
    }
}


pub mod mbc_none {
    use crate::Cartridge;
    use crate::mbc::Mbc;


    /// A default MBC handling ROMs which do not need bank switching.
    pub struct MbcNone {
    }

    impl MbcNone {
        /// Creates a default MBC object.
        pub fn new() -> MbcNone {
            MbcNone {
            }
        }
    }

    impl Mbc for MbcNone {
        fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8 {
            cartridge.get_rom().get_at(address as usize)
        }

        fn write_byte(&mut self, _cartridge: &mut Cartridge, _address: u16, _value: u8) {
        }
    }
}


mod mbc1 {
    use crate::Cartridge;
    use crate::memory_data::MemoryData;
    use crate::mbc::Mbc;

    /// Type 1 Memory Bank Controller:
    /// Supports up to 2MB ROMs or up to 32kB RAM.
    pub struct Mbc1 {
        /// Mode switch between Mode 0 = 128 ROM banks 1 RAM bank and Mode 1 = 32 ROM banks, 4 RAM banks.
        mode: u8,

        /// The value written into the first bank selection register.
        /// Contains the lower 5 bits of the selected ROM bank.
        bank_selection_0: u8,

        /// The value written into the second bank selection register.
        /// Contains either the upper two bits of the ROM bank number
        /// or 2 bits for RAM bank selection, depending on the selected mode.
        bank_selection_1: u8,

        /// The selected ROM bank number.
        rom_bank_selected: u8,

        /// The offset added to the address the game wants to read from,
        /// to get the real address within the ROM file.
        rom_bank_offset:   usize,

        /// The selected RAM bank number.
        ram_bank_selected: u8,

        /// The offset added to the address the game wants to read/write,
        /// to get the real address within the RAM image.
        ram_bank_offset: usize,

        /// Sets if the RAM bank is enabled or not.
        ram_enabled: bool,
    }

    impl Mbc1 {
        pub fn new() -> Mbc1 {
            Mbc1 {
                mode: 0,

                bank_selection_0: 0x00,
                bank_selection_1: 0x00,

                rom_bank_selected: 1,
                rom_bank_offset:   0x0000,

                ram_bank_selected: 0,
                ram_bank_offset:   0x0000,

                ram_enabled: false,
            }
        }


        /// After writing to one of the bank selection registers,
        /// this function is used to calculate the actual RAM and ROM bank numbers
        /// as well as the offsets to read and write inside the ROM and RAM images.
        fn update_selected_banks(&mut self) {
            let mut rom_bank = self.bank_selection_0;
            let mut ram_bank = 0;

            if self.mode == 0 {
                // in mode 0 the 2 bits of the 2nd register will be used as
                // bit 5 and 6 of the rom bank selection
                rom_bank |= self.bank_selection_1 << 4;
            }
            else {
                // in mode 1, the bits of the 2nd register will be used as RAM bank number
                ram_bank = self.bank_selection_1;
            }

            // Bank 0 is only accessible in the 0x0000 - 0x1fff range
            // Banks 0x20, 0x40 and 0x60 are inaccessible too and translated into the next bank
            rom_bank = match rom_bank {
                0x00 | 0x20 | 0x40 | 0x60 => rom_bank + 1,
                _ => rom_bank,
            };

            // store the rom bank and the offset to be added to all
            // requested addresses, beginning with 0x4000
            self.rom_bank_selected = rom_bank;
            self.rom_bank_offset   = ((rom_bank as usize) * 0x4000) - 0x4000;

            // store the ram bank and the offset to be added to all addresses
            self.ram_bank_selected = ram_bank;
            self.ram_bank_offset   = (ram_bank as usize) * 0x2000;
        }
    }

    impl Mbc for Mbc1 {
        fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8 {
            match address {
                // read from fixed ROM bank, which is always bank 0.
                0x0000 ..= 0x3fff => {
                    let rom_address = address as usize;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from the switchable ROM bank
                0x4000 ..= 0x7fff => {
                    let rom_address = (address as usize) + self.rom_bank_offset;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from switchable RAM bank.
                0xa000 ..= 0xbfff => {
                    if cartridge.has_ram() && self.ram_enabled {
                        let ram_address = (address as usize) - 0xa000 + self.ram_bank_offset;
                        cartridge.get_ram().get_at(ram_address)
                    }
                    else {
                        0xff
                    }
                }

                _ => unreachable!("Unexpected read from address {}", address),
            }
        }

        fn write_byte(&mut self, cartridge: &mut Cartridge, address: u16, value: u8) {
            match (address >> 13) & 0x000f {
                // 0x0000 - 0x1fff: enable or disable RAM
                0x00 => {
                    self.ram_enabled = (value & 0x0f) == 0x0a;
                },

                // 0x2000 - 0x3fff: select ROM bank
                0x01 => {
                    self.bank_selection_0 = value & 0x1f;
                    self.update_selected_banks();
                },

                // 0x4000 - 0x5fff: select RAM bank
                0x02 => {
                    self.bank_selection_1 = value & 0x03;
                    self.update_selected_banks();
                },

                // 0x6000 - 0x7fff: switch ROM / RAM mode
                0x03 => {
                    self.mode = value & 0x01;
                    self.update_selected_banks();
                },

                // 0xa000 - 0xbfff: Cartridge RAM
                0x05 => {
                    if cartridge.has_ram() && self.ram_enabled {
                        let ram_address = (address as usize) - 0xa000 + self.ram_bank_offset;
                        cartridge.get_mut_ram().set_at(ram_address, value);
                    }
                },

                _ => unreachable!("Unexpected write to address {}", address),
            }
        }
    }
}


mod mbc5 {
    use crate::Cartridge;
    use crate::memory_data::MemoryData;
    use crate::mbc::Mbc;

    /// Type 5 Memory Bank Controller:
    /// Supports up to 8MB ROMs and up to 128kB RAM.
    pub struct Mbc5 {
        /// The value written into the first bank selection register.
        /// Contains the lower 8 bits of the selected ROM bank.
        rom_bank_selection_0: u8,

        /// The value written into the second bank selection register.
        /// Contains the 9th bit of the selected ROM bank.
        rom_bank_selection_1: u8,

        /// The value written into the RAM bank selection register.
        ram_bank_selection_0: u8,

        /// The selected ROM bank number.
        rom_bank_selected: u16,

        /// The offset added to the address the game wants to read from,
        /// to get the real address within the ROM file.
        rom_bank_offset:   usize,

        /// The selected RAM bank number.
        ram_bank_selected: u16,

        /// The offset added to the address the game wants to read/write,
        /// to get the real address within the RAM image.
        ram_bank_offset: usize,

        /// Sets if the RAM bank is enabled or not.
        ram_enabled: bool,
    }

    impl Mbc5 {
        pub fn new() -> Self {
            Self {
                rom_bank_selection_0: 0x00,
                rom_bank_selection_1: 0x00,
                ram_bank_selection_0: 0x00,

                rom_bank_selected: 1,
                rom_bank_offset:   0x0000,

                ram_bank_selected: 0,
                ram_bank_offset:   0x0000,

                ram_enabled: false,
            }
        }


        /// After writing to one of the bank selection registers,
        /// this function is used to calculate the actual RAM and ROM bank numbers
        /// as well as the offsets to read and write inside the ROM and RAM images.
        fn update_selected_banks(&mut self) {
            let rom_bank =
                (self.rom_bank_selection_0 as u16)
              | ((self.rom_bank_selection_1 as u16 & 0x01) << 8)
            ;

            let ram_bank =
                self.ram_bank_selection_0 as u16
            ;

            // store the rom bank and the offset to be added to all
            // requested addresses, beginning with 0x4000
            self.rom_bank_selected = rom_bank;
            self.rom_bank_offset   = (rom_bank as usize) * 0x4000;

            // store the ram bank and the offset to be added to all addresses
            self.ram_bank_selected = ram_bank;
            self.ram_bank_offset   = (ram_bank as usize) * 0x2000;
        }
    }

    impl Mbc for Mbc5 {
        fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8 {
            match address {
                // read from fixed ROM bank, which is always bank 0.
                0x0000 ..= 0x3fff => {
                    let rom_address = address as usize;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from the switchable ROM bank
                0x4000 ..= 0x7fff => {
                    let rom_address = (address as usize) + self.rom_bank_offset - 0x4000;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from switchable RAM bank.
                0xa000 ..= 0xbfff => {
                    if cartridge.has_ram() && self.ram_enabled {
                        let ram_address = (address as usize) - 0xa000 + self.ram_bank_offset;
                        cartridge.get_ram().get_at(ram_address)
                    }
                    else {
                        0xff
                    }
                }

                _ => unreachable!("Unexpected read from address {}", address),
            }
        }

        fn write_byte(&mut self, cartridge: &mut Cartridge, address: u16, value: u8) {
            match address {
                // enable or disable RAM
                0x0000 ..= 0x1fff => {
                    self.ram_enabled = (value & 0x0f) == 0x0a;
                },

                // ROM bank selection #0
                0x2000 ..= 0x2fff => {
                    self.rom_bank_selection_0 = value;
                    self.update_selected_banks();
                },

                // ROM bank selection #1
                0x3000 ..= 0x3fff => {
                    self.rom_bank_selection_1 = value;
                    self.update_selected_banks();
                },

                // RAM bank selection
                0x4000 ..= 0x5fff => {
                    self.ram_bank_selection_0 = value & 0x0f;
                    self.update_selected_banks();
                },

                // Cartridge RAM
                0xa000 ..= 0xbfff => {
                    if cartridge.has_ram() && self.ram_enabled {
                        let ram_address = (address as usize) - 0xa000 + self.ram_bank_offset;
                        cartridge.get_mut_ram().set_at(ram_address, value);
                    }
                },

                _ => unreachable!("Unexpected write to address {}", address),
            }
        }
    }
}
