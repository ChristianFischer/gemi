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
use std::rc::Rc;
use crate::Cartridge;
use crate::cpu::{to_u16, to_u8};

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
    memory: [u8; 0x10000]
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
}

/// A trait for memory objects allowing write access to the memory data.
pub trait MemoryWrite {
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
}


impl Memory {
    /// Create a new Memory object.
    pub fn new() -> Self {
        Self {
            internal: MemoryInternalRef::new(
                MemoryInternal {
                    memory: [0; 0x10000],
                }
            )
        }
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

    /// Load ROM data from a cartridge into the memory.
    pub fn load_rom_data(&mut self, cartridge: &Cartridge) {
        let mut mem = self.internal.get_mut();

        for address in 0..0x7fff {
            let value = cartridge.get_rom_data_at(address);
            mem.memory[address as usize] = value;
        }
    }
}

impl MemoryRead for Memory {
    fn read_byte(&self, address: u16) -> u8 {
        self.internal.get().memory[address as usize]
    }
}

impl MemoryWrite for Memory {
    fn write_byte(&mut self, address: u16, value: u8) {
        self.internal.get_mut().memory[address as usize] = value;
    }
}

impl MemoryRead for MemoryReadOnlyHandle {
    fn read_byte(&self, address: u16) -> u8 {
        self.internal.get().memory[address as usize]
    }
}

impl MemoryRead for MemoryReadWriteHandle {
    fn read_byte(&self, address: u16) -> u8 {
        self.internal.get().memory[address as usize]
    }
}

impl MemoryWrite for MemoryReadWriteHandle {
    fn write_byte(&mut self, address: u16, value: u8) {
        self.internal.get_mut().memory[address as usize] = value;
    }
}

impl MemoryReadWriteHandle {
    pub fn clone_readonly(&self) -> MemoryReadOnlyHandle {
        MemoryReadOnlyHandle {
            internal: self.internal.clone()
        }
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
