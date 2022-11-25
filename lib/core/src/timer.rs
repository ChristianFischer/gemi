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

use crate::cpu::Interrupt;
use crate::gameboy::Clock;
use crate::memory::{MEMORY_LOCATION_REGISTER_DIV, MEMORY_LOCATION_REGISTER_TAC, MEMORY_LOCATION_REGISTER_TIMA, MEMORY_LOCATION_REGISTER_TMA, MemoryRead, MemoryReadWriteHandle, MemoryWrite};
use crate::utils::get_bit;


const UPDATE_TIME_DIV:     Clock =    256;
const UPDATE_TIME_TIMA_00: Clock =   1024;
const UPDATE_TIME_TIMA_01: Clock =     16;
const UPDATE_TIME_TIMA_02: Clock =     64;
const UPDATE_TIME_TIMA_03: Clock =    256;


/// An object handling the gameboys internal timers,
/// which are controlled by TIMA, TMA, TAC and DIV registers.
pub struct Timer {
    mem: MemoryReadWriteHandle,

    /// Stores the previous value of the DIV register (0xff04)
    /// to check if it was written to and therefor need to be reset.
    div_previous: u8,

    /// The clock which counts the cycles to increment the DIV register.
    /// DIV will be updated after the clock reaches UPDATE_TIME_DIV.
    div_clock: Clock,

    /// Stores the previous value of the TAC register (0xff07)
    /// to check if it was written to.
    tac_previous: u8,

    /// Flag to store whether the TIMA timer is enabled.
    /// This flag will be set by changing bit 2 of the TAC register.
    tima_enabled: bool,

    /// The update clock selection defined by the value of the TAC register.
    /// Should be either 4kHz, 16kHz, 64kHz or 256kHz.
    tima_update_time: Clock,

    /// The clock which counts the cycles to increment the TIMA register.
    tima_clock: Clock,
}


impl Timer {
    /// Creates an empty CPU object.
    pub fn new(mem: MemoryReadWriteHandle) -> Timer {
        Timer {
            mem,

            div_previous: 0x00,
            tac_previous: 0x00,

            tima_enabled: false,
            tima_update_time: UPDATE_TIME_TIMA_00,

            div_clock: 0,
            tima_clock: 0,
        }
    }


    /// Update timers for n CPU cycles.
    pub fn update(&mut self, cycles: Clock) {
        self.check_for_changed_registers();
        self.update_div(cycles);
        self.update_tima(cycles);
    }


    /// check for changed values (should be moved into a callback instead)
    fn check_for_changed_registers(&mut self) {
        let div = self.mem.read_u8(MEMORY_LOCATION_REGISTER_DIV);
        let tac = self.mem.read_u8(MEMORY_LOCATION_REGISTER_TAC);

        // check if DIV changed
        if self.div_previous != div {
            // if DIV was written to, it always resets to zero
            self.mem.write_u8(MEMORY_LOCATION_REGISTER_DIV, 0);

            self.div_previous = 0;
            self.div_clock = 0;
        }

        // check if TAC changed
        if self.tac_previous != tac {
            self.tac_previous = tac;

            // check whether the timer is enabled
            self.tima_enabled = get_bit(tac, 2);

            // select the desired update frequency
            self.tima_update_time = match tac & 0x03 {
                0 => UPDATE_TIME_TIMA_00,
                1 => UPDATE_TIME_TIMA_01,
                2 => UPDATE_TIME_TIMA_02,
                3 => UPDATE_TIME_TIMA_03,
                _ => panic!("Unexpected value")
            };

            // reset clock on change
            self.tima_clock = 0;
        }
    }


    /// Update the DIV timer
    fn update_div(&mut self, cycles: Clock) {
        self.div_clock = self.div_clock.wrapping_add(cycles);

        while self.div_clock >= UPDATE_TIME_DIV {
            let mut div = self.mem.read_u8(MEMORY_LOCATION_REGISTER_DIV);
            div = div.wrapping_add(1);

            self.mem.write_u8(MEMORY_LOCATION_REGISTER_DIV, div);

            self.div_clock = self.div_clock.saturating_sub(UPDATE_TIME_DIV);
            self.div_previous = div;
        }
    }


    /// Update the TIMA timer and handles the timer interrupt
    fn update_tima(&mut self, cycles: Clock) {
        if self.tima_enabled {
            self.tima_clock = self.tima_clock.wrapping_add(cycles);

            while self.tima_clock >= self.tima_update_time {
                let mut tima = self.mem.read_u8(MEMORY_LOCATION_REGISTER_TIMA);
                tima = match tima.checked_add(1) {
                    Some(v) => v,
                    None => {
                        // overflow, raise interrupt
                        self.mem.request_interrupt(Interrupt::Timer);

                        // reset TIMA to the value of TMA
                        self.mem.read_u8(MEMORY_LOCATION_REGISTER_TMA)
                    }
                };

                self.mem.write_u8(MEMORY_LOCATION_REGISTER_TIMA, tima);

                self.tima_clock = self.tima_clock.saturating_sub(self.tima_update_time);
            }
        }

    }

}