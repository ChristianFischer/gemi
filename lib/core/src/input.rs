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
use crate::memory::{MEMORY_LOCATION_JOYP, MemoryRead, MemoryReadWriteHandle, MemoryWrite};
use crate::utils::{change_bit, get_bit};


/// A list of all buttons available on the GameBoy
pub enum InputButton {
    DPadRight   = 0,
    DPadLeft    = 1,
    DPadUp      = 2,
    DPadDown    = 3,
    A           = 4,
    B           = 5,
    Select      = 6,
    Start       = 7,
}


pub struct Input {
    mem: MemoryReadWriteHandle,

    /// Current state of each button.
    /// bit == 1 means pressed, bit == 0 means released
    button_states: u8,

    /// The pressed state of each button last time;
    /// used to detect changes in the pressed state of each button
    previous_button_states: u8,

    /// The previous state of the JOYP register.
    /// Used to detect changes done by the running ROM.
    previous_joyp: u8,
}


impl Input {
    /// Creates a new Input object.
    pub fn new(mem: MemoryReadWriteHandle) -> Input {
        Input {
            mem,
            button_states:          0x00,
            previous_button_states: 0x00,
            previous_joyp:          0xff,
        }
    }


    /// Updates the JOYP register and fire the input interrupt depending on the current button states.
    pub fn update(&mut self) {
        let joyp = self.mem.read_u8(MEMORY_LOCATION_JOYP);
        let keys_changed = self.button_states != self.previous_button_states;

        if keys_changed {
            let keys_pressed = self.button_states & !self.previous_button_states;

            // when any key was pressed this frame, fire the input interrupt
            if keys_pressed != 0 {
                self.mem.request_interrupt(Interrupt::Input);
            }

            self.previous_button_states = self.button_states;
        }

        if (joyp != self.previous_joyp) || keys_changed {
            let select = joyp & 0x30;
            let value = match select {
                0x00 => 0x00,
                0x10 => (!self.button_states >> 4) & 0x0f,
                0x20 => (!self.button_states >> 0) & 0x0f,
                _    => 0x0f,
            };

            let new_joyp = select | value;

            self.mem.write_u8(MEMORY_LOCATION_JOYP, new_joyp);
            self.previous_joyp = new_joyp;
        }
    }

    /// Changes the 'pressed' state for any button.
    pub fn set_button_pressed(&mut self, button: InputButton, pressed: bool) {
        self.button_states = change_bit(self.button_states, button as u8, pressed);
    }

    /// Checks whether a particular button is currently pressed.
    pub fn is_button_pressed(&self, button: InputButton) -> bool {
        get_bit(self.button_states, button as u8)
    }
}
