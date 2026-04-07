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

use crate::cartridge::Cartridge;
use crate::client_data::GameBoyClientData;
use crate::Builder;

use crate::core::apu::Apu;
use crate::core::cpu::cpu::Cpu;
use crate::core::device_type::DeviceConfig;
use crate::core::emulator_device::{Clock, EmulatorDevice, EmulatorUpdateResults};
use crate::core::input::Input;
use crate::core::mmu::memory::Memory;
use crate::core::mmu::mmu::Mmu;
use crate::core::ppu::ppu::Ppu;
use crate::core::serial::SerialPort;


/// The GameBoy object providing access to all it's emulated components.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameBoy {
    pub(crate) client_data: Box<GameBoyClientData>,
    pub(crate) emulator: Box<EmulatorDevice>,
}


impl GameBoy {
    /// Creates a builder to build up the device.
    pub fn build() -> Builder {
        Builder::new()
    }


    /// Boot the device, initializing the Boot ROM program.
    pub fn initialize(&mut self) {
        self.emulator.initialize(self.client_data.as_mut());
    }


    /// Get the number of cycles processed by the emulator since it started.
    pub fn get_total_cycles_processed(&self) -> Clock {
        self.emulator.get_total_cycles_processed()
    }


    /// Get the time in seconds the emulator did run.
    pub fn get_total_seconds_processed(&self) -> f32 {
        self.emulator.get_total_seconds_processed()
    }


    /// Runs the emulator for a single step, either an instruction
    /// or to process a single HALT cycle.
    pub fn run_single_step(&mut self) -> EmulatorUpdateResults {
        self.emulator.run_single_step(self.client_data.as_mut())
    }


    /// Continues running the program located on the cartridge,
    /// until the PPU has completed one single frame.
    pub fn run_frame(&mut self) -> EmulatorUpdateResults {
        self.emulator.run_frame(self.client_data.as_mut())
    }


    /// Reads a single byte from the emulator's memory bus.
    pub fn read_u8(&self, address: u16) -> u8 {
        self.emulator.get_mmu().read_u8(self.client_data.as_ref(), address)
    }


    /// Writes a single byte into the emulator's memory bus.
    pub fn write_u8(&mut self, address: u16, value: u8) {
        self.emulator.get_mmu_mut().write_u8(self.client_data.as_mut(), address, value)
    }


    /// Get the emulator device configuration.
    pub fn get_config(&self) -> &DeviceConfig {
        &self.client_data.device_config
    }


    /// Get a reference to the emulator's cartridge, if any.
    pub fn get_cartridge(&self) -> &Cartridge {
        &self.client_data.cartridge
    }


    /// Get the actual emulator instance.
    pub fn get_emulator(&self) -> &EmulatorDevice {
        &self.emulator
    }


    /// Get the actual emulator instance.
    pub fn get_emulator_mut(&mut self) -> &mut EmulatorDevice {
        &mut self.emulator
    }


    /// Get the device CPU.
    pub fn get_cpu(&self) -> &Cpu {
        &self.emulator.cpu
    }


    /// Get the device CPU.
    pub fn get_cpu_mut(&mut self) -> &mut Cpu {
        &mut self.emulator.cpu
    }


    /// Get the device MMU.
    pub fn get_mmu(&self) -> &Mmu {
        self.emulator.get_mmu()
    }


    /// Get the device MMU.
    pub fn get_mmu_mut(&mut self) -> &mut Mmu {
        self.emulator.get_mmu_mut()
    }


    /// Get the device memory component.
    pub fn get_memory(&self) -> &Memory {
        &self.emulator.get_peripherals().mem
    }


    /// Get the device memory component.
    pub fn get_memory_mut(&mut self) -> &mut Memory {
        &mut self.emulator.get_peripherals_mut().mem
    }


    /// Get the device PPU.
    pub fn get_ppu(&self) -> &Ppu {
        &self.emulator.get_peripherals().ppu
    }


    /// Get the device PPU.
    pub fn get_ppu_mut(&mut self) -> &mut Ppu {
        &mut self.emulator.get_peripherals_mut().ppu
    }


    /// Get the device APU.
    pub fn get_apu(&self) -> &Apu {
        &self.emulator.get_peripherals().apu
    }


    /// Get the device APU.
    pub fn get_apu_mut(&mut self) -> &mut Apu {
        &mut self.emulator.get_peripherals_mut().apu
    }


    /// Get the device input component.
    pub fn get_input(&self) -> &Input {
        &self.emulator.get_peripherals().input
    }


    /// Get the device input component.
    pub fn get_input_mut(&mut self) -> &mut Input {
        &mut self.emulator.get_peripherals_mut().input
    }


    /// Get the device serial port component.
    pub fn get_serial_port(&self) -> &SerialPort {
        &self.emulator.get_peripherals().serial
    }


    /// Get the device serial port component.
    pub fn get_serial_port_mut(&mut self) -> &mut SerialPort {
        &mut self.emulator.get_peripherals_mut().serial
    }
}

