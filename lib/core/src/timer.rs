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

use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::ops::Sub;
use crate::cpu::Interrupt;
use crate::gameboy::Clock;
use crate::memory::{MEMORY_LOCATION_REGISTER_DIV, MEMORY_LOCATION_REGISTER_TAC, MEMORY_LOCATION_REGISTER_TIMA, MemoryReadWriteHandle};
use crate::utils::{get_bit, get_high};


/// Represents the internal counter which will be incremented with the system clock
/// and triggers incrementing the TIMA counter each time a specific bit, which is
/// configured by TAC falls from 1 to 0.
/// - The usual case which triggers TIMA to be incremented will be when the counter
/// reaches a specific threshold. For example when incrementing from ```0b00001111```
/// to ```0b00010000```, bit 3 will fall from 1 to 0 and trigger the increment.
/// - When the counter register will be reset on writing the DIV register, this may
/// also trigger when the trigger bit was set to 1 before.
/// There are also additional edge cases which may also trigger TIMA to be incremented,
///
/// The upper 8 bits of the counter stores the value which will be accessible via DIV register
/// and therefor automatically be incremented each 256 ticks.
struct InternalCounter {
    /// The value of the counter
    value: u16,

    /// Stores the information whether the timer was enabled via TAC or not
    timer_enabled: bool,

    /// Number of the bit which will trigger the TIMA increment on fall
    fall_bit: u8,

    /// A bitmask to receive the value of the trigger bit
    fall_bit_mask: u16,

    /// A bitmask to receive the counter to the next increment of TIMA
    fall_bit_remainder_mask: u16,

    /// Flag which stores whether the fall bit was triggered on the last operation
    fall_bit_triggered: bool,
}


/// State of the TIMA counter.
/// When TIMA overflows from incrementing at 0xff, it will not immediately reset to TMA
/// and fire the interrupt. Instead, it will remain in an overflow state for 4 cycles.
/// In this state, TIMA will have the value of 0x00. After 4 cycles it will reset into
/// the value stored in TMA as intended and the timer interrupt will be fired.
/// Writing to TIMA during the overflow state will cancel the overflow and let TIMA
/// continue counting with the new value. Writing to TIMA when it's value is reloaded
/// from TMA will ignore the written TIMA value and keep the value from TMA.
enum TimaState {
    /// TIMA is counting normally.
    Normal,

    /// TIMA was wrapping from 0xff into 0x00 and remains in overflow state
    /// until next update.
    Overflow,

    /// The overflow was processed recently and TIMA holds the value of TMA.
    /// If TIMA will be written in this state, the written value has to be ignored.
    OverflowProcessed,
}



/// An object handling the gameboys internal timers,
/// which are controlled by TIMA, TMA, TAC and DIV registers.
pub struct Timer {
    mem: MemoryReadWriteHandle,

    /// The internal counter used to trigger TIMA increments
    internal_counter: InternalCounter,

    /// Internal state of the TIMA counter.
    /// See documentation of ```TimaState```
    tima_state: TimaState,
}


/// Get the bit which triggers the TIMA increment based on the value of the TAC register.
fn get_trigger_bit(tac: u8) -> u8 {
    match tac & 0b11 {
        0b00 => 9,
        0b01 => 3,
        0b10 => 5,
        0b11 => 7,
        _    => unreachable!(),
    }
}


impl InternalCounter {
    /// Creates a new counter with default value.
    pub fn new() -> Self {
        Self::with_value(0x0000, 0x00)
    }


    /// Creates a new counter with a specific initial value.
    pub fn with_value(value: u16, tac: u8) -> Self {
        let mut counter = Self {
            value,

            timer_enabled: false,

            fall_bit: 0,
            fall_bit_mask: 0x0000,
            fall_bit_remainder_mask: 0x0000,

            fall_bit_triggered: false,
        };

        // applies a 'zero' TAC value by default
        counter.apply_tac(tac);

        counter
    }


    /// Applies the configuration of the TAC register.
    pub fn apply_tac(&mut self, tac: u8) {
        let bit_before = self.check_trigger_bit();

        // get the trigger bit from TAC
        let bit = get_trigger_bit(tac);
        let enabled = get_bit(tac, 2);

        // store the bit and mask values
        self.fall_bit                = bit;
        self.fall_bit_mask           = 1u16 << bit;
        self.fall_bit_remainder_mask = (1u16 << bit) - 1;
        self.timer_enabled           = enabled;

        // get the new value of the trigger bit
        let bit_after = self.check_trigger_bit();

        self.fall_bit_triggered = bit_before && !bit_after;
    }


    /// Get the value of the current trigger bit.
    /// This is basically the implementation of the circuit connecting TAC with the internal counter.
    /// The result will be true, if
    /// - The bit of the internal counter, selected by the first two bits in TAC, is 1
    /// - AND The TAC bit 2 (timer enabled) is 1
    pub fn check_trigger_bit(&self) -> bool {
        self.timer_enabled && (self.value & self.fall_bit_mask) != 0
    }


    /// Returns true, when the fall bit was triggered on the last operation.
    pub fn is_fall_bit_triggered(&self) -> bool {
        self.fall_bit_triggered
    }


    /// Reset the internal counter, setting it's value to 0.
    /// This may trigger the fall bit to cause a TIMA increment.
    pub fn reset(&mut self) {
        let bit_before = self.check_trigger_bit();

        self.value = 0;

        // resetting the counter will also trigger the fall bit,
        // if the relevant bit was set to 1 before
        self.fall_bit_triggered = bit_before;
    }


    /// Increments the counter by a given amount.
    /// This may trigger the fall bit to cause a TIMA increment.
    pub fn increment(&mut self, count: u16) {
        // get the trigger bit before incrementing
        let bit_before = self.check_trigger_bit();

        // increment
        self.value = self.value.wrapping_add(count);

        // get the new value of the trigger bit
        let bit_after = self.check_trigger_bit();

        // fall bit will be triggered, when the trigger bit switched from 1 to 0
        self.fall_bit_triggered = bit_before && !bit_after;
    }


    /// Get the value of the counter.
    pub fn get_value(&self) -> u16 {
        self.value
    }


    /// Get the number of cycles remaining until the fall bit will be triggered.
    pub fn get_remaining_cycles_to_trigger(&self) -> u16 {
        // take the complement of the timer value and leave out bits
        // not part of the value range of the trigger counter
        ((!self.get_value()) & self.fall_bit_remainder_mask) + 1
    }


    /// Get the value of the DIV register.
    pub fn get_div(&self) -> u8 {
        get_high(self.value)
    }
}


impl Display for InternalCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04x}", self.value)
    }
}


impl Timer {
    /// Creates an empty CPU object.
    pub fn new(mem: MemoryReadWriteHandle) -> Timer {
        Timer {
            mem,

            internal_counter: InternalCounter::new(),
            tima_state: TimaState::Normal,
        }
    }


    /// (Re)initializes the internal counter with specific initial values.
    pub fn initialize_counter(&mut self, counter: u16, tac: u8) {
        self.internal_counter = InternalCounter::with_value(counter, tac);
    }


    /// Update timers for n CPU cycles.
    pub fn update(&mut self, cycles: Clock) {
        self.check_for_changed_registers();
        self.handle_overflow();
        self.increment_counter(cycles);
    }


    /// check for changed values (should be moved into a callback instead)
    fn check_for_changed_registers(&mut self) {
        // check if DIV changed
        if let Some(_) = self.mem.take_changed_io_register(MEMORY_LOCATION_REGISTER_DIV) {
            // if DIV was written to, it always resets to zero
            self.mem.get_io_registers_mut().div = 0;

            // writing to DIV will reset the counter
            self.internal_counter.reset();

            // resetting the counter may trigger an increment on TIMA,
            // if the trigger bit was falling from 1 to 0
            if self.internal_counter.is_fall_bit_triggered() {
                self.increment_tima();
            }
        }

        // handle TIMA writing during overflow state
        if let Some(_) = self.mem.take_changed_io_register(MEMORY_LOCATION_REGISTER_TIMA) {
            match self.tima_state {
                // TIMA was written during the overflow state, which will
                // cancel the overflow state and let TIMA continue counting as usual
                TimaState::Overflow => {
                    self.tima_state = TimaState::Normal;
                }

                _ => {}
            }
        }

        // check if TAC changed
        if let Some(tac) = self.mem.take_changed_io_register(MEMORY_LOCATION_REGISTER_TAC) {
            // apply the new value of TAC
            self.internal_counter.apply_tac(tac);

            // changing the value may cause the fall bit detection to trigger
            if self.internal_counter.is_fall_bit_triggered() {
                self.increment_tima();
            }
        }
    }


    /// Increment the internal counter by the clock ticks passed since last call.
    /// Also increments TIMA when triggered.
    fn increment_counter(&mut self, cycles: Clock) {
        let div_before = self.internal_counter.get_div();
        let mut cycles_remaining = cycles;

        while cycles_remaining > 0 {
            // compute how much we can increment, until we hit the trigger value
            let cycles_until_trigger = self.internal_counter.get_remaining_cycles_to_trigger();
            let increment            = min(cycles_remaining, cycles_until_trigger as Clock);

            cycles_remaining = cycles_remaining.sub(increment);

            // do the increment
            self.internal_counter.increment(increment as u16);

            // check if we hit the fall bit trigger
            if self.internal_counter.is_fall_bit_triggered() {
                self.increment_tima();
            }
        }

        // Check if DIV changed
        let div_after = self.internal_counter.get_div();
        if div_before != div_after {
            self.mem.get_io_registers_mut().div = div_after;
        }
    }


    /// Increments the TIMA counter.
    /// On overflow, TIMA will remain in it's overflow state, which means it's value stays on
    /// zero for 4 cycles and the interrupt is delayed until the overflow state ends.
    fn increment_tima(&mut self) {
        // handle pending overflow, if any
        self.handle_overflow();

        // perform the increment
        {
            let mut io_regs = self.mem.get_io_registers_mut();
            let (tima, overflow) = io_regs.tima.overflowing_add(1);

            // store the incremented value
            io_regs.tima = tima;

            if overflow {
                self.tima_state = TimaState::Overflow;
            }
        }
    }


    /// Handle the TIMA overflow state.
    fn handle_overflow(&mut self) {
        match self.tima_state {
            TimaState::Overflow => {
                // overflow, raise interrupt
                self.mem.request_interrupt(Interrupt::Timer);

                // reset value
                self.reset_tima_to_tma();

                // switch into processed state to handle the situation of
                // simultaneously writing TIMA on overflow
                self.tima_state = TimaState::OverflowProcessed;
            },

            TimaState::OverflowProcessed => {
                // reset TIMA to the value of TMA
                self.reset_tima_to_tma();

                // switch back into normal counting state
                self.tima_state = TimaState::Normal;
            },

            _ => {}
        }
    }


    /// Reset TIMA by loading the value of TMA
    fn reset_tima_to_tma(&mut self) {
        let mut io_regs = self.mem.get_io_registers_mut();

        let tma = io_regs.tma;
        io_regs.tima = tma;
    }

}