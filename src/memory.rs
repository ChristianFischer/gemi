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

use std::cell::{Ref, RefCell, RefMut};
use std::cmp::min;
use std::rc::Rc;
use crate::boot_rom::BootRom;
use crate::Cartridge;
use crate::cpu::Interrupt;
use crate::utils::{clear_bit, get_bit, set_bit, to_u16, to_u8};

pub const MEMORY_LOCATION_SPRITES_BEGIN:            u16 = 0x8000;
pub const MEMORY_LOCATION_BACKGROUND_MAP_BEGIN:     u16 = 0x9800;
pub const MEMORY_LOCATION_OAM_BEGIN:                u16 = 0xfe00;
pub const MEMORY_LOCATION_OAM_END:                  u16 = 0xfe9f;
pub const MEMORY_LOCATION_REGISTER_DIV:             u16 = 0xff04;
pub const MEMORY_LOCATION_REGISTER_TIMA:            u16 = 0xff05;
pub const MEMORY_LOCATION_REGISTER_TMA:             u16 = 0xff06;
pub const MEMORY_LOCATION_REGISTER_TAC:             u16 = 0xff07;
pub const MEMORY_LOCATION_LCD_CONTROL:              u16 = 0xff40;
pub const MEMORY_LOCATION_LCD_STATUS:               u16 = 0xff41;
pub const MEMORY_LOCATION_SCY:                      u16 = 0xff42;
pub const MEMORY_LOCATION_SCX:                      u16 = 0xff43;
pub const MEMORY_LOCATION_LY:                       u16 = 0xff44;
pub const MEMORY_LOCATION_LYC:                      u16 = 0xff45;
pub const MEMORY_LOCATION_DMA_ADDRESS:              u16 = 0xff46;
pub const MEMORY_LOCATION_WY:                       u16 = 0xff4a;
pub const MEMORY_LOCATION_WX:                       u16 = 0xff4b;
pub const MEMORY_LOCATION_BOOT_ROM_DISABLE:         u16 = 0xff50;
pub const MEMORY_LOCATION_INTERRUPTS_PENDING:       u16 = 0xff0f;
pub const MEMORY_LOCATION_INTERRUPTS_ENABLED:       u16 = 0xffff;


/// Stores the information of an active OAM DMA transfer
/// The DMA transfer copies data from the given address
/// into OAM memory.
/// In total, 160 bytes will be transferred, so it takes 
/// 160 cycles to transfer for the transfer to be completed.
pub struct DmaTransferInfo {
    /// The address where to start copying the memory from.
    start_address: u16,

    /// The next byte to be copied.
    next_byte: u16,
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

/// A memory handle to provide readonly access to the device memory.
/// This object can be created from an existing Memory object.
#[derive(Clone)]
pub struct MemoryReadOnlyHandle {
    internal: MemoryInternalRef,
}

/// A memory handle to provide read/write access to the device memory.
/// This object can be created from an existing Memory object.
#[derive(Clone)]
pub struct MemoryReadWriteHandle {
    internal: MemoryInternalRef,
}

/// Shared internal object for multiple Memory and MemoryReadWrite instances.
struct MemoryInternal {
    memory:     Box<[u8; 0x10000]>,
    dma:        DmaTransferState,
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
    pub fn new() -> Self {
        Self {
            internal: MemoryInternalRef::new(
                MemoryInternal {
                    memory:     Box::new([0; 0x10000]),
                    dma:        DmaTransferState::Disabled,
                    boot_rom:   None,
                    cartridge:  None,
                }
            )
        }
    }

    /// Let the memory controller handle it's tasks.
    /// 'cycles' gives the number of ticks passed since 
    /// the last call.
    pub fn update(&mut self, cycles: u32) {
        self.internal.get_mut().handle_dma_transfer(cycles);
    }


    /// Creates a MemoryReadOnlyHandle from this Memory object.
    /// This will be used to provide read/write access to the device memory.
    pub fn create_readonly_handle(&self) -> MemoryReadOnlyHandle {
        MemoryReadOnlyHandle {
            internal: MemoryInternalRef::clone(&self.internal)
        }
    }

    /// Creates a MemoryReadWriteHandle from this Memory object.
    /// This will be used to provide read/write access to the device memory.
    pub fn create_read_write_handle(&self) -> MemoryReadWriteHandle {
        MemoryReadWriteHandle {
            internal: MemoryInternalRef::clone(&self.internal)
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
        mem.cartridge = Some(cartridge);
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

impl MemoryRead for MemoryReadOnlyHandle {
    fn read_byte(&self, address: u16) -> u8 {
        self.internal.get().read(address)
    }
}

impl MemoryRead for MemoryReadWriteHandle {
    fn read_byte(&self, address: u16) -> u8 {
        self.internal.get().read(address)
    }
}

impl MemoryWrite for MemoryReadWriteHandle {
    fn write_byte(&mut self, address: u16, value: u8) {
        self.internal.get_mut().write(address, value);
    }
}

impl MemoryReadWriteHandle {
    pub fn clone_readonly(&self) -> MemoryReadOnlyHandle {
        MemoryReadOnlyHandle {
            internal: self.internal.clone()
        }
    }

    /// requests an interrupt to be fired.
    /// This will set the according bit in the memory. If Interrupts
    /// are enabled for the CPU, the instruction pointer will jump
    /// to the according interrupt address, otherwise it will be ignored.
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.set_bit(MEMORY_LOCATION_INTERRUPTS_PENDING, interrupt.bit());
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
        match address {
            0x0000 ..= 0x00ff => self.read_boot_rom_or_cartridge(address),
            0x0100 ..= 0x7fff => self.read_from_cartridge(address),
            _                 => self.read_internal_memory(address),
        }
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
            return cartridge.get_rom_data_at(address);
        }

        self.read_internal_memory(address)
    }

    /// Reads data from the internal memory.
    fn read_internal_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    /// Writes data to any memory location.
    /// The request will be forwarded to the according device, depending
    /// on the physical location of the data (like cartridge, ppu, etc)
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xff00 ..= 0xffff => self.write_io_registers(address, value),
            _                 => self.write_internal_memory(address, value),
        }
    }

    /// Writes data into IO registers
    fn write_io_registers(&mut self, address: u16, value: u8) {
        self.write_internal_memory(address, value);

        match address {
            MEMORY_LOCATION_DMA_ADDRESS => {
                let start_address = (value as u16) << 8;

                self.dma = DmaTransferState::Transferring(DmaTransferInfo {
                    start_address,
                    next_byte: 0,
                });
            },

            MEMORY_LOCATION_BOOT_ROM_DISABLE => {
                if (value & 0x01) != 0 {
                    self.boot_rom = None;
                }
            },

            _ => { }
        }
    }

    /// Writes data into the internal memory.
    fn write_internal_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    /// Handles an OAM DMA transfer, if any active.
    fn handle_dma_transfer(&mut self, cycles: u32) {
        match self.dma {
            DmaTransferState::Disabled => {}

            DmaTransferState::Transferring(ref transfer) => {
                let oam_size       = MEMORY_LOCATION_OAM_END - MEMORY_LOCATION_OAM_BEGIN + 1;
                let transfer_begin = transfer.next_byte;
                let transfer_end   = min(transfer_begin.saturating_add(cycles as u16), oam_size);
                
                // Copy the amount of data the memory controller was able to handle  
                for b in transfer_begin .. transfer_end {
                    let src = transfer.start_address.saturating_add(b);
                    let dst = MEMORY_LOCATION_OAM_BEGIN.saturating_add(b);
                    self.memory[dst as usize] = self.memory[src as usize];
                }

                // store the current state or set the transfer state to 'Disabled'
                if transfer_end == oam_size {
                    self.dma = DmaTransferState::Disabled;
                }
                else {
                    self.dma = DmaTransferState::Transferring(DmaTransferInfo {
                        start_address: transfer.start_address,
                        next_byte:     transfer_end,
                    });
                }
            }
        }
    }
}