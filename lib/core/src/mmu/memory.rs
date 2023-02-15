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

use std::cell::{Ref, RefCell, RefMut};
use std::cmp::max;
use std::io;
use std::rc::Rc;
use crate::boot_rom::BootRom;
use crate::cartridge::Cartridge;
use crate::cpu::cpu::Interrupt;
use crate::gameboy::{DeviceConfig, EmulationType};
use crate::mmu::io_registers::IoRegister;
use crate::mmu::locations::*;
use crate::mmu::mbc::{create_mbc, Mbc};
use crate::mmu::mbc::mbc_none::MbcNone;
use crate::mmu::memory_bus::{memory_map, MemoryBusConnection};
use crate::mmu::memory_data::mapped::MemoryDataMapped;
use crate::mmu::memory_data::{MemoryData, MemoryDataFixedSize};
use crate::utils::{clear_bit, get_bit, set_bit, to_u16, to_u8};


/// Stores the information of an active OAM DMA transfer
/// The DMA transfer copies data from the given address
/// into OAM memory.
/// In total, 160 bytes will be transferred, so it takes
/// 160 cycles to transfer for the transfer to be completed.
pub struct DmaTransferInfo {
    /// The address where to start copying the memory from.
    pub start_address: u16,

    /// The next byte to be copied.
    pub next_byte: u16,
}


/// State of the OAM DMA transfer, whether it be disabled or
/// in progress, including the time remaining.
pub enum DmaTransferState {
    /// No transfer is active.
    Disabled,

    /// A transfer is currently in progress.
    /// The attached struct stores information where
    /// the data should be taken from and how much data
    /// was already transferred.
    Transferring(DmaTransferInfo),
}


/// The memory object as the main owner of the emulator's memory.
/// This object can be used to create additional handles which
/// allow read and write access to the device memory.
pub struct Memory {
    internal: MemoryInternalRef,
}


pub type WRamBank       = MemoryDataFixedSize<4096>;
pub type HRamBank       = MemoryDataFixedSize<127>;
pub type IoRegisterBank = MemoryDataMapped<IoRegister>;


/// Shared internal object for multiple Memory and MemoryReadWrite instances.
struct MemoryInternal {
    /// The configuration of the running device
    device_config: DeviceConfig,

    /// Work RAM banks (DMG = 2 * 4kiB, GBC = 8 * 4kiB)
    wram_banks: Vec<WRamBank>,

    /// Active Work RAM banks.
    /// Bank 0 is fixed, Bank 1 can be switched between 1-7 on GBC.
    wram_active_bank_0: u8,
    wram_active_bank_1: u8,

    /// IO Registers
    io_registers: IoRegisterBank,

    /// Holds a bit for each IO register, which is set once
    /// the according register was written to.
    io_registers_written: [bool; 256],

    /// High RAM
    hram: HRamBank,

    mbc:        Box<dyn Mbc>,
    boot_rom:   Option<BootRom>,
    cartridge:  Option<Cartridge>,
}

/// Helper object to store a shared reference to the internal memory object.
/// The shared reference can be owned by multiple memory objects and therefore
/// granting read or write access to the device memory for multiple components.
#[derive(Clone)]
struct MemoryInternalRef {
    r: Rc<RefCell<MemoryInternal>>,
}

/// A trait for memory objects allowing read access to the memory data.
pub trait MemoryRead {
    /// Read a single byte from the device memory.
    fn read_byte(&self, address: u16) -> u8;

    /// Read an u8 value from the given address in the device memory.
    fn read_u8(&self, address: u16) -> u8 {
        self.read_byte(address)
    }

    /// Read an i8 value from the given address in the device memory.
    fn read_i8(&self, address: u16) -> i8 {
        self.read_byte(address) as i8
    }

    /// Read an u16 value from the given address in the device memory.
    fn read_u16(&self, address: u16) -> u16 {
        let l = self.read_byte(address);
        let h = self.read_byte(address + 1);
        to_u16(h, l)
    }

    /// Read an i16 value from the given address in the device memory.
    fn read_i16(&self, address: u16) -> i16 {
        self.read_u16(address) as i16
    }

    /// Get the n'th bit of a byte on the given address.
    fn get_bit(&self, address: u16, bit: u8) -> bool {
        let byte = self.read_u8(address);
        get_bit(byte, bit)
    }
}

/// A trait for memory objects allowing write access to the memory data.
pub trait MemoryWrite : MemoryRead {
    /// Write a single byte into the device memory.
    fn write_byte(&mut self, address: u16, value: u8);

    /// Writes an u8 value into the given address in the device memory.
    fn write_u8(&mut self, address: u16, value: u8) {
        self.write_byte(address, value);
    }

    /// Writes an i8 value into the given address in the device memory.
    fn write_i8(&mut self, address: u16, value: i8) {
        self.write_byte(address, value as u8);
    }

    /// Writes an u16 value into the given address in the device memory.
    fn write_u16(&mut self, address: u16, value: u16) {
        let (h, l) = to_u8(value);
        self.write_byte(address + 0, l);
        self.write_byte(address + 1, h);
    }

    /// Writes an i16 value into the given address in the device memory.
    fn write_i16(&mut self, address: u16, value: i16) {
        self.write_u16(address, value as u16);
    }

    /// Set the n'th bit of a byte on the given address.
    fn change_bit(&mut self, address: u16, bit: u8, value: bool) {
        if value {
            self.set_bit(address, bit);
        }
        else {
            self.clear_bit(address, bit);
        }
    }

    /// Set the n'th bit of a byte on the given address to 0.
    fn clear_bit(&mut self, address: u16, bit: u8) {
        let byte = self.read_u8(address);
        let result = clear_bit(byte, bit);
        self.write_byte(address, result);
    }

    /// Set the n'th bit of a byte on the given address to 1.
    fn set_bit(&mut self, address: u16, bit: u8) {
        let byte = self.read_u8(address);
        let result = set_bit(byte, bit);
        self.write_byte(address, result);
    }
}


impl Memory {
    /// Create a new Memory object.
    pub fn new(device_config: DeviceConfig) -> Self {
        let num_wram_banks = match device_config.emulation {
            EmulationType::DMG => 2,
            EmulationType::GBC => 8,
        };

        Self {
            internal: MemoryInternalRef::new(
                MemoryInternal {
                    device_config,

                    wram_banks: std::iter::repeat_with(|| WRamBank::new()).take(num_wram_banks).collect(),
                    wram_active_bank_0: 0,
                    wram_active_bank_1: 1,

                    hram: HRamBank::new(),
                    io_registers: IoRegisterBank::new(IoRegister::default()),
                    io_registers_written: [false; 256],

                    mbc:        Box::new(MbcNone::new()),
                    boot_rom:   None,
                    cartridge:  None,
                }
            )
        }
    }


    pub fn new_ref(&self) -> Self {
        Self {
            internal: MemoryInternalRef::clone(&self.internal)
        }
    }


    /// Initializes the values of all IO registers.
    pub fn initialize_io_registers(&mut self, initial_values: [u8; 256]) {
        let mut mem = self.internal.get_mut();
        let io_regs = &mut mem.io_registers;

        for i in 0..256 {
            io_regs.set_at(i, initial_values[i]);
        }
    }


    /// Checks whether a boot rom is active or not.
    pub fn has_boot_rom(&self) -> bool {
        match self.internal.get().boot_rom {
            None    => false,
            Some(_) => true,
        }
    }

    /// Load a boot ROM into the memory.
    pub fn set_boot_rom(&mut self, boot_rom: BootRom) {
        let mut mem = self.internal.get_mut();
        mem.boot_rom = Some(boot_rom)
    }

    /// Load ROM data from a cartridge into the memory.
    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        let mut mem = self.internal.get_mut();
        mem.mbc       = create_mbc(cartridge.get_mbc());
        mem.cartridge = Some(cartridge);
    }

    /// Get a reference to the currently assigned cartridge, if any.
    pub fn get_cartridge(&self) -> Ref<Option<Cartridge>> {
        let mem = self.internal.get();
        Ref::map(mem, |mem| &mem.cartridge)
    }

    /// Save the cartridge RAM, if any.
    pub fn save_cartridge_ram_if_any(&self) -> io::Result<()> {
        let mem = self.internal.get();

        if let Some(cartridge) = &mem.cartridge {
            cartridge.save_ram_if_any()?;
        }

        Ok(())
    }
}

impl MemoryRead for Memory {
    fn read_byte(&self, address: u16) -> u8 {
        self.internal.get().read(address)
    }
}

impl MemoryWrite for Memory {
    fn write_byte(&mut self, address: u16, value: u8) {
        self.internal.get_mut().write(address, value);
    }
}

impl MemoryBusConnection for Memory {
    fn on_read(&self, address: u16) -> u8 {
        self.read_byte(address)
    }

    fn on_write(&mut self, address: u16, value: u8) {
        self.write_byte(address, value);
    }
}

impl Memory {
    /// Get the list of Work RAM banks on this device.
    /// DMG = 2 banks, GBC = 8 banks.
    pub fn get_wram_banks(&self) -> Ref<Vec<WRamBank>> {
        let mem = self.internal.get();
        Ref::map(mem, |mem| &mem.wram_banks)
    }

    /// Get the IO Registers struct.
    pub fn get_io_registers(&self) -> Ref<IoRegister> {
        let mem = self.internal.get();
        Ref::map(mem, |mem| mem.io_registers.get())
    }

    /// Get the IO Registers struct as a mutable reference.
    pub fn get_io_registers_mut(&mut self) -> RefMut<IoRegister> {
        let mem = self.internal.get_mut();
        RefMut::map(mem, |mem| mem.io_registers.get_mut())
    }

    /// Checks whether a specific IO register was written to. The flag will be kept until
    /// acknowledged by calling ```acknowledge_io_register_written```.
    pub fn was_io_register_written(&self, address: u16) -> bool {
        assert!(address >= 0xff00, "Address is not an IO register");
        let mem = self.internal.get();
        mem.io_registers_written[(address & 0xff) as usize]
    }

    /// Acknowledges the 'written to' flag for a specific IO register. This will set the
    /// flag to false until being written again.
    pub fn acknowledge_io_register_written(&mut self, address: u16) {
        assert!(address >= 0xff00, "Address is not an IO register");
        let mut mem = self.internal.get_mut();
        mem.io_registers_written[(address & 0xff) as usize] = false;
    }

    /// A convenience function which combines the functionality of ```was_io_register_written```,
    /// ```acknowledge_io_register_written``` and reading the value of the given register.
    /// If the requested IO register was changed, it acknowledges the writing operation
    /// and returns it's new value. Otherwise, it will return ```None```.
    pub fn take_changed_io_register(&mut self, address: u16) -> Option<u8> {
        assert!(address >= 0xff00, "Address is not an IO register");
        let mut mem = self.internal.get_mut();
        let index = (address & 0xff) as usize;

        if mem.io_registers_written[index] {
            mem.io_registers_written[index] = false;

            Some(mem.io_registers.get_at(index))
        }
        else {
            None
        }
    }

    /// requests an interrupt to be fired.
    /// This will set the according bit in the memory. If Interrupts
    /// are enabled for the CPU, the instruction pointer will jump
    /// to the according interrupt address, otherwise it will be ignored.
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.set_bit(MEMORY_LOCATION_INTERRUPTS_FLAGGED, interrupt.bit());
    }
}

impl MemoryInternalRef {
    /// Creates a new shared reference to a MemoryInternal object.
    /// The given MemoryInternal object will be moved inside the newly created shared reference.
    pub fn new(internal: MemoryInternal) -> MemoryInternalRef {
        MemoryInternalRef {
            r: Rc::new(RefCell::new(internal))
        }
    }

    /// Creates a clone of a reference to a MemoryInternal object.
    /// This just clones the reference, not the MemoryInternal object itself.
    pub fn clone(other: &MemoryInternalRef) -> MemoryInternalRef {
        MemoryInternalRef {
            r: Rc::clone(&other.r)
        }
    }

    /// Get a readonly reference to the MemoryInternal object.
    pub fn get(&self) -> Ref<MemoryInternal> {
        let ref_cell : &RefCell<MemoryInternal> = &(self.r);
        ref_cell.borrow()
    }

    /// Get a mutable reference to the MemoryInternal object.
    pub fn get_mut(&mut self) -> RefMut<MemoryInternal> {
        let ref_cell : &RefCell<MemoryInternal> = &(self.r);
        ref_cell.borrow_mut()
    }
}

impl MemoryInternal {
    /// Reads data from any memory location.
    /// The request will be forwarded to the according device, depending
    /// on the physical location of the data (like cartridge, ppu, etc)
    pub fn read(&self, address: u16) -> u8 {
        memory_map!(
            address => {
                0x0000 ..= 0x00ff => [] self.read_boot_rom_or_cartridge(address),
                0x0100 ..= 0x7fff => [] self.read_from_cartridge(address),
                0xa000 ..= 0xbfff => [] self.read_from_cartridge(address),

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
                    self.read((mapped_address + 0xc000) as u16)
                },

                0xfea0 ..= 0xfeff => []               unreachable!(), // unusable ram area
                0xff00 ..= 0xff7f => [mapped_address] self.io_registers.get_at(mapped_address),
                0xff80 ..= 0xfffe => [mapped_address] self.hram.get_at(mapped_address),
                0xffff            => []               self.io_registers.get().interrupts_enabled
            }
        )
    }

    /// Reads data from the boot rom, if any, otherwise from the cartridge.
    fn read_boot_rom_or_cartridge(&self, address: u16) -> u8 {
        if let Some(boot_rom) = &self.boot_rom {
            return boot_rom.read(address);
        }

        self.read_from_cartridge(address)
    }

    /// Reads data from the cartridge.
    fn read_from_cartridge(&self, address: u16) -> u8 {
        if let Some(cartridge) = &self.cartridge {
            return self.mbc.read_byte(cartridge, address);
        }

        0xff
    }

    /// Writes data to any memory location.
    /// The request will be forwarded to the according device, depending
    /// on the physical location of the data (like cartridge, ppu, etc)
    pub fn write(&mut self, address: u16, value: u8) {
        memory_map!(
            address => {
                0x0000 ..= 0x7fff => [] self.write_to_cartridge(address, value),
                0xa000 ..= 0xbfff => [] self.write_to_cartridge(address, value),

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
                    self.write((mapped_address + 0xc000) as u16, value)
                },

                0xfea0 ..= 0xfeff => []               unreachable!(), // unusable ram area

                0xff00 ..= 0xff7f => [mapped_address] {
                    let old = self.io_registers.get_at(mapped_address);
                    self.io_registers.set_at(mapped_address, value);
                    self.io_registers_written[mapped_address] = true;
                    self.on_io_registers_changed(address, old, value);
                },

                0xff80 ..= 0xfffe => [mapped_address] self.hram.set_at(mapped_address, value),

                0xffff => [] {
                    let ioreg = self.io_registers.get_mut();
                    let old = ioreg.interrupts_enabled;
                    ioreg.interrupts_enabled = value;

                    self.io_registers_written[0xff] = true;

                    self.on_io_registers_changed(address, old, value);
                }
            }
        )
    }

    /// Writes data to the cartridge.
    fn write_to_cartridge(&mut self, address: u16, value: u8) {
        if let Some(cartridge) = &mut self.cartridge {
            self.mbc.write_byte(cartridge, address, value);
        }
    }

    /// Writes data into IO registers
    fn on_io_registers_changed(&mut self, address: u16, _old_value: u8, value: u8) {
        match address {
            MEMORY_LOCATION_BOOT_ROM_DISABLE => {
                if (value & 0x01) != 0 {
                    self.boot_rom = None;
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
