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
use crate::apu::generators::envelope::Envelope;
use crate::apu::generators::length_timer::LengthTimer;
use crate::apu::generators::SoundGenerator;
use crate::apu::registers::{ApuChannelRegisters, ApuRegisters};
use crate::gameboy::Clock;
use crate::utils::get_bit;


/// A sound generator to generate random noise using a Linear Feedback Shift Register (LFSR)
/// This will generate a random sequence of 0 and 1, modified with the volume from
/// the volume envelope function.
pub struct NoiseGenerator {
    enabled: bool,

    /// A length timer controlling how long the sound generator will run.
    length_timer: LengthTimer<6>,

    /// An envelope function to provide the volume for the generated wave.
    envelope: Envelope,

    /// A Linear Feedback Shift Register used to generate a random sequence of 0 an 1.
    lfsr: u16,

    /// Width of the LFSR in bits.
    lfsr_width: u16,

    /// The time left until the next LFSR iteration.
    frequency_timer: Clock,
}


impl NoiseGenerator {
    pub fn new() -> Self {
        Self {
            enabled:            false,
            envelope:           Envelope::default(),
            length_timer:       LengthTimer::default(),
            lfsr:               0,
            lfsr_width:         15,
            frequency_timer:    0,
        }
    }


    /// Compute the time until next LFSR iteration.
    pub fn compute_timer(&self, registers: &ApuChannelRegisters) -> Clock {
        let shift        = (registers.nr3 >> 4) & 0x0f;
        let divider_code = (registers.nr3 >> 0) & 0x07;

        let divider = match divider_code {
            0 => 8,
            _ => (divider_code as Clock) << 4,
        };

        divider << shift
    }
}


impl SoundGenerator for NoiseGenerator {
    fn on_trigger_event(&mut self, registers: &ApuChannelRegisters) {
        self.length_timer = LengthTimer::from_registers(registers);
        self.envelope     = Envelope::from_registers(registers);

        // bit 3 determines the length of the LFSR, either 7 or 15 bits.
        let lfsr_is_short = get_bit(registers.nr3, 3);
        self.lfsr_width = match lfsr_is_short {
            false => 15,
            true  => 7,
        };

        // reset lfsr to zero
        self.lfsr = 0;

        // channel is enabled when DAC is enabled as well
        self.enabled = self.envelope.get_dac_enabled();
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
        self.envelope.tick();
    }


    fn update(&mut self, registers: &ApuChannelRegisters, cycles: Clock) {
        let mut remaining_cycles = cycles;

        while remaining_cycles > 0 {
            let run_cycles = min(self.frequency_timer, remaining_cycles);

            self.frequency_timer  = self.frequency_timer.saturating_sub(run_cycles);

            // when the timer expires
            if self.frequency_timer == 0 {
                // timer will be restarted
                self.frequency_timer = self.compute_timer(registers);

                // determine the new bit to insert by XOR of bit 0 and 1
                let insert_bit = (self.lfsr ^ (self.lfsr >> 1) ^ 1) & 0x01;

                // the XOR bit will be inserted at the position left of the lfsr width;
                // the following shift will move the new bit in it's desired position
                self.lfsr |= insert_bit << self.lfsr_width;

                // the whole register gets shifted to the right
                self.lfsr >>= 1;
            }

            remaining_cycles = remaining_cycles.saturating_sub(run_cycles);
        }
    }


    fn is_dac_enabled(&self) -> bool {
        self.envelope.get_dac_enabled()
    }


    fn get_sample(&self, _registers: &ApuRegisters) -> u8 {
        if self.enabled {
            // take bit 0 to determine whether a tone is generated or not
            let amp    = (self.lfsr & 0x01) as u8;
            let volume = self.envelope.get_current_volume();

            amp * volume
        }
        else {
            0
        }
    }
}
