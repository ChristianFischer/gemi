/*
 * Copyright (C) 2022-2026 by Christian Fischer
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

use crate::cpu::interrupts::{Interrupt, Interrupts};
use crate::emulator_client::EmulatorClientMut;


/// A listener trait for debugging events during emulator execution.
pub trait DebugListener {
    /// Called when the instruction pointer (IP) changes.
    fn on_instruction_pointer_changed(&mut self, address: u16) {
        _ = address;
    }

    /// Called when a memory location is written to.
    ///
    /// If this method returns `Some(value)`, the returned value will be written
    /// to memory instead of the original value.
    fn on_mem_write(&mut self, addr: u16, value: u8) -> Option<u8> {
        _ = (addr, value);
        None
    }

    /// Called if an interrupt was raised.
    fn on_interrupt_triggered(&mut self, interrupt: Interrupt) {
        _ = interrupt
    }

    /// Called when the PPU completes rendering a single scanline.
    fn on_ppu_line_completed(&mut self, line: u8) {
        _ = line;
    }

    /// Called when the PPU completes rendering a full frame.
    fn on_ppu_frame_completed(&mut self) {}

    /// Checks if the debugger wants to suspend execution after hitting breakpoints.
    fn is_halted(&self) -> bool {
        false
    }
}


/// A dummy implementation of the DebugListener trait without functionality.
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NullDebugListener;

impl DebugListener for NullDebugListener {
}


/// Utility to invoke [DebugListener] callbacks from an [EmulatorClientMut] instance.
/// This utility is aware of the `debug_interface` feature and only invokes callback
/// functions if the feature is enabled.
pub struct DebugDelegate<'a, D>
    where D: DebugListener
{
    #[allow(unused)]
    listener: &'a mut D
}


#[cfg(feature = "debug_interface")]
impl<'a, D> DebugDelegate<'a, D>
where D: DebugListener
{
    /// Calls [DebugListener::on_instruction_pointer_changed].
    pub fn invoke_instruction_pointer_changed(&mut self, address: u16) {
        self.listener.on_instruction_pointer_changed(address)
    }

    /// Calls [DebugListener::on_mem_write].
    pub fn invoke_mem_write(&mut self, addr: u16, value: u8) -> Option<u8> {
        self.listener.on_mem_write(addr, value)
    }

    /// Calls [DebugListener::on_interrupt_triggered].
    pub fn invoke_interrupt_triggered(&mut self, interrupts: impl Into<Interrupts>) {
        for interrupt in Into::<Interrupts>::into(interrupts).into_iter() {
            self.listener.on_interrupt_triggered(interrupt)
        }
    }

    /// Calls [DebugListener::on_ppu_line_completed].
    pub fn invoke_ppu_line_completed(&mut self, line: u8) {
        self.listener.on_ppu_line_completed(line)
    }

    /// Calls [DebugListener::on_ppu_frame_completed].
    pub fn invoke_ppu_frame_completed(&mut self) {
        self.listener.on_ppu_frame_completed()
    }
}


#[cfg(not(feature = "debug_interface"))]
impl<'a, D> DebugDelegate<'a, D>
    where D: DebugListener
{
    /// Calls [DebugListener::on_instruction_pointer_changed].
    pub fn invoke_instruction_pointer_changed(&mut self, address: u16) {
        _ = address;
    }

    /// Calls [DebugListener::on_mem_write].
    pub fn invoke_mem_write(&mut self, addr: u16, value: u8) -> Option<u8> {
        _ = (addr, value);
        None
    }

    /// Calls [DebugListener::on_interrupt_triggered].
    pub fn invoke_interrupt_triggered(&mut self, interrupts: impl Into<Interrupts>) {
        _ = interrupts;
    }

    /// Calls [DebugListener::on_ppu_line_completed].
    pub fn invoke_ppu_line_completed(&mut self, line: u8) {
        _ = line;
    }

    /// Calls [DebugListener::on_ppu_frame_completed].
    pub fn invoke_ppu_frame_completed(&mut self) {
    }
}


impl<'a, D> DebugDelegate<'a, D>
where D: DebugListener
{
    /// Creates the [DebugDelegate] for a specific [EmulatorClientMut]
    pub fn with<EC>(ec: &'a mut EC) -> DebugDelegate<'a, EC::DebugInterface>
    where EC: EmulatorClientMut<DebugInterface = D>
    {
        DebugDelegate::<'a, EC::DebugInterface> {
            listener: ec.get_debug_interface_mut()
        }
    }
}
