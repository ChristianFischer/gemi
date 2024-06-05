/*
 * Copyright (C) 2022-2024 by Christian Fischer
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

use flagset::{flags, FlagSet};

use crate::mmu::locations::*;
use crate::mmu::memory_bus::MemoryBusConnection;

flags! {
    /// An enumeration of all interrupts available.
    pub enum Interrupt : u8 {
        VBlank      = 0b_0000_0001,
        LcdStat     = 0b_0000_0010,
        Timer       = 0b_0000_0100,
        Serial      = 0b_0000_1000,
        Input       = 0b_0001_0000,
    }
}

pub type Interrupts = FlagSet<Interrupt>;



/// A data struct holding the interrupt registers used by the CPU
/// and connected via memory bus to other components.
pub struct InterruptRegisters {
    /// IF: pending interrupts
    interrupts_flagged: Interrupts,

    /// IE: interrupts enabled
    interrupts_enabled: Interrupts,

    /// Stores the unused bits of the 'interrupts_enabled' register.
    ie_unused_bits: u8,
}


impl Interrupt {
    /// An array containing all possible interrupts for easier iteration.
    const ALL_INTERRUPTS : [Interrupt; 5] = [
        Interrupt::VBlank,
        Interrupt::LcdStat,
        Interrupt::Timer,
        Interrupt::Serial,
        Interrupt::Input
    ];


    /// Get the address this interrupt will jump to when fired.
    pub fn address(&self) -> u16 {
        match self {
            Interrupt::VBlank   => 0x0040,
            Interrupt::LcdStat  => 0x0048,
            Interrupt::Timer    => 0x0050,
            Interrupt::Serial   => 0x0058,
            Interrupt::Input    => 0x0060,
        }
    }
}


impl InterruptRegisters {
    pub fn new() -> Self {
        Self {
            interrupts_flagged: Interrupts::default(),
            interrupts_enabled: Interrupts::default(),
            ie_unused_bits:     0b_1110_0000,
        }
    }


    /// requests an interrupt to be fired.
    /// This will set the according interrupt bit. If Interrupts
    /// are enabled for the CPU, the instruction pointer will jump
    /// to the according interrupt address, otherwise it will be ignored.
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupts_flagged |= interrupt;
    }


    /// requests multiple interrupts to be fired.
    /// This will set the according interrupt bit. If Interrupts
    /// are enabled for the CPU, the instruction pointer will jump
    /// to the according interrupt address, otherwise it will be ignored.
    pub fn request_interrupts(&mut self, interrupts: impl Into<Interrupts>) {
        self.interrupts_flagged |= interrupts;
    }


    /// Get pending interrupts in form of an integer with each bit representing it's according interrupt.
    pub fn get_interrupts_pending(&self) -> Interrupts {
        self.interrupts_flagged & self.interrupts_enabled
    }


    /// Checks if any interrupts are ready to be handled.
    /// Only interrupts enabled via IE register contribute to this check.
    pub fn has_interrupts_pending(&self) -> bool {
        !self.get_interrupts_pending().is_empty()
    }


    /// Checks the pending interrupts and returns the next pending interrupt,
    /// which was enabled via IE register.
    pub fn take_pending_interrupt(&mut self) -> Option<Interrupt> {
        let interrupts_pending = self.get_interrupts_pending();

        if !interrupts_pending.is_empty() {
            for interrupt in &Interrupt::ALL_INTERRUPTS {
                if interrupts_pending.contains(*interrupt) {
                    // remove the interrupt we're going to handle
                    self.interrupts_flagged -= *interrupt;

                    // found pending interrupt
                    return Some(*interrupt);
                }
            }
        }

        None
    }
}


impl MemoryBusConnection for InterruptRegisters {
    fn on_read(&self, address: u16) -> u8 {
        match address {
            MEMORY_LOCATION_INTERRUPTS_FLAGGED => self.interrupts_flagged.bits() | 0b_1110_0000,
            MEMORY_LOCATION_INTERRUPTS_ENABLED => self.interrupts_enabled.bits() | self.ie_unused_bits,

            _ => 0xff,
        }
    }


    fn on_write(&mut self, address: u16, value: u8) {
        match address {
            MEMORY_LOCATION_INTERRUPTS_FLAGGED => self.interrupts_flagged = Interrupts::new_truncated(value),

            MEMORY_LOCATION_INTERRUPTS_ENABLED => {
                self.interrupts_enabled = Interrupts::new_truncated(value);
                self.ie_unused_bits     = value & 0b_1110_0000;
            }

            _ => { }
        }
    }
}
