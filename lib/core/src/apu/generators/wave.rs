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

use std::cmp::min;
use crate::apu::generators::length_timer::LengthTimer;
use crate::apu::generators::SoundGenerator;
use crate::apu::registers::{ApuChannelRegisters, ApuRegisters};
use crate::gameboy::Clock;
use crate::utils::get_bit;


pub struct WaveGenerator {
    /// Stores whether the sound generator is enabled or not.
    enabled: bool,

    /// Stores whether the DAC of this channel is enabled or not.
    dac_enabled: bool,

    /// A length timer controlling how long the sound generator will run.
    length_timer: LengthTimer<8>,

    /// The number of bits to shift the value of the sound sample to lower it's volume.
    volume_shift: u8,

    /// The length controlling how fast the wave will be played.
    /// The value will be read from NRx3 and NRx4 register of the channel.
    wave_length: Clock,

    /// Current value of the wave timer. The value is decreased with each CPU T-Cycle
    /// and restarts when reaching zero.
    wave_timer: Clock,

    /// The current position where to read the value from the wave duty.
    wave_step: u8,
}


impl WaveGenerator {
    pub fn new() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            length_timer: LengthTimer::default(),

            volume_shift: 0,

            wave_length: 0,
            wave_timer: 0,
            wave_step: 0,
        }
    }
}


impl SoundGenerator for WaveGenerator {
    fn on_trigger_event(&mut self, registers: &ApuChannelRegisters) {
        self.dac_enabled  = get_bit(registers.nr0, 7);
        self.length_timer = LengthTimer::from_registers(registers);

        let output_level = (registers.nr2 >> 5) & 0x03;
        self.volume_shift = match output_level {
            0b00 => 4, // mute
            0b01 => 0, // 100% (no change)
            0b10 => 1, //  50%
            0b11 => 2, //  25%
            _ => unreachable!()
        };

        let wave_length_register_value = (registers.nr3 as Clock) | (((registers.nr4 as Clock) & 0x07) << 8);
        let wave_length = (2048 - wave_length_register_value) * 4;

        self.wave_length = wave_length;
        self.wave_timer  = wave_length;

        self.enabled = true;
    }


    fn tick_sound_length(&mut self, registers: &ApuChannelRegisters) {
        let expired = self.length_timer.tick(registers);

        // when the timer expires, the generator will be disabled
        if expired {
            self.enabled = false;
        }
    }


    fn tick_freq_sweep(&mut self, _registers: &ApuChannelRegisters) {
    }


    fn tick_envelope_sweep(&mut self, _registers: &ApuChannelRegisters) {
    }


    fn update(&mut self, _registers: &ApuChannelRegisters, cycles: Clock) {
        if self.enabled {
            let mut remaining_cycles = cycles;

            while remaining_cycles > 0 {
                let run_cycles = min(self.wave_timer, remaining_cycles);

                self.wave_timer  = self.wave_timer.saturating_sub(run_cycles);

                // when the wave timer expires, it will be restarted and the
                // position inside the wave ram proceeds
                if self.wave_timer == 0 {
                    self.wave_timer = self.wave_length;
                    self.wave_step = self.wave_step.wrapping_add(1);
                }

                remaining_cycles = remaining_cycles.saturating_sub(run_cycles);
            }
        }
    }


    fn is_dac_enabled(&self) -> bool {
        self.dac_enabled
    }


    fn get_sample(&self, registers: &ApuRegisters) -> u8 {
        // since wave ram contains two samples per byte,
        // the index within the wave ram is wave_step / 2.
        let index = (self.wave_step >> 1) & 0x0f;

        // read the byte from wave RAM
        let value = registers.wave_ram.0[index as usize];

        // depending on bit 1, either take the high or low nibble
        let amp = match self.wave_step & 0x01 {
            0 => (value >> 4) & 0x0f,
            _ => (value >> 0) & 0x0f,
        };

        // apply the volume shift to the amplitude to get the final sample
        let sample = amp >> self.volume_shift;

        sample
    }
}