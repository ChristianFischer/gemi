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

use std::cell::RefCell;
use std::rc::Rc;
use crate::Cartridge;

/// The memory object as the main owner of the emulator's memory.
/// This object can be used to create additional handles which
/// allow read and write access to the device memory.
pub struct Memory {
    internal: MemoryInternalRef,
}

/// A memory handle to provide read/write access to the device memory.
/// This object can be created from an existing Memory object.
pub struct MemoryReadWrite {
    internal: MemoryInternalRef,
}

/// Shared internal object for multiple Memory and MemoryReadWrite instances.
struct MemoryInternal {
    memory: [u8; 0xffff]
}

type MemoryInternalRef = Rc<RefCell<MemoryInternal>>;


impl Memory {
    /// Create a new Memory object.
    pub fn new() -> Self {
        Self {
            internal: Rc::new(RefCell::new(
                MemoryInternal {
                    memory: [0; 0xffff],
                }
            ))
        }
    }

    /// Creates a MemoryReadWrite object from this Memory object.
    /// This will be used to provide read/write access to the device memory.
    pub fn create_read_write_handle(&self) -> MemoryReadWrite {
        MemoryReadWrite {
            internal: Rc::clone(&self.internal)
        }
    }

    /// Load ROM data from a cartridge into the memory.
    pub fn load_rom_data(&mut self, cartridge: &Cartridge) {
        let ref_cell : &RefCell<MemoryInternal> = &(self.internal);
        let mut mem : &mut MemoryInternal = &mut (ref_cell.borrow_mut());

        for address in 0..0x7fff {
            let value = cartridge.get_rom_data_at(address);
            mem.memory[address as usize] = value;
        }
    }
}

impl MemoryReadWrite {
    /// Read a single byte from the device memory.
    pub fn read(&self, address: u16) -> u8 {
        let ref_cell : &RefCell<MemoryInternal> = &(self.internal);
        ref_cell.borrow().memory[address as usize]
    }

    /// Write a single byte into the device memory.
    pub fn write(&mut self, address: u16, value: u8) {
        let ref_cell : &RefCell<MemoryInternal> = &(self.internal);
        ref_cell.borrow_mut().memory[address as usize] = value;
    }
}
