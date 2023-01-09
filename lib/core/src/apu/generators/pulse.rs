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
use crate::apu::generators::freq_sweep::{FrequencySweep, FrequencySweepResult};
use crate::apu::generators::length_timer::LengthTimer;
use crate::apu::generators::SoundGenerator;
use crate::apu::generators::wave_duty::WaveDuty;
use crate::apu::registers::{ApuChannelRegisters, ApuRegisters};
use crate::gameboy::Clock;


/// A sound generator to generate a pulse wave. The wave is based is based on a wave duty value
/// and a volume computed by an envelope function.
pub struct PulseGenerator {
    /// Stores whether the sound generator is enabled or not.
    enabled: bool,

    /// A length timer controlling how long the sound generator will run.
    length_timer: LengthTimer<6>,

    /// An envelope function to provide the volume for the generated wave.
    envelope: Envelope,

    /// The base of the sound wave to be played, represented by a square wave.
    wave_duty: WaveDuty,

    /// The length controlling how fast the wave will be played.
    /// The value will be read from NRx3 and NRx4 register of the channel.
    wave_length: Clock,

    /// Current value of the wave timer. The value is decreased with each CPU T-Cycle
    /// and restarts when reaching zero.
    wave_timer: Clock,

    /// The current position where to read the value from the wave duty.
    wave_duty_step: u8,
}


/// Variation of the PulseGenerator with an additional frequency sweep to change
/// the wave's frequency over time.
pub struct PulseSweepGenerator {
    pulse_generator: PulseGenerator,
    freq_sweep: FrequencySweep,
}


impl PulseGenerator {
    pub fn new() -> Self {
        Self {
            enabled:      false,
            wave_duty:    WaveDuty::default(),
            envelope:     Envelope::default(),
            length_timer: LengthTimer::default(),
            wave_length: 0,
            wave_timer: 0,
            wave_duty_step: 0,
        }
    }
}


impl SoundGenerator for PulseGenerator {
    fn on_trigger_event(&mut self, registers: &ApuChannelRegisters) {
        let wave_duty_value = (registers.nr1 >> 6) & 0x03;

        self.length_timer = LengthTimer::from_registers(registers);
        self.envelope     = Envelope::from_registers(registers);
        self.wave_duty    = WaveDuty::by_index(wave_duty_value);

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
        self.envelope.tick();
    }


    fn update(&mut self, _registers: &ApuChannelRegisters, cycles: Clock) {
        if self.enabled {
            let mut remaining_cycles = cycles;

            while remaining_cycles > 0 {
                let run_cycles = min(self.wave_timer, remaining_cycles);

                self.wave_timer  = self.wave_timer.saturating_sub(run_cycles);

                // when the wave timer expires, it will be restarted and the
                // position inside the wave duty proceeds
                if self.wave_timer == 0 {
                    self.wave_timer = self.wave_length;
                    self.wave_duty_step = self.wave_duty_step.wrapping_add(1);
                }

                remaining_cycles = remaining_cycles.saturating_sub(run_cycles);
            }
        }
    }


    fn is_dac_enabled(&self) -> bool {
        self.envelope.get_dac_enabled()
    }


    fn get_sample(&self, _registers: &ApuRegisters) -> u8 {
        let volume = self.envelope.get_current_volume();
        let wave = self.wave_duty.get_wave_at(self.wave_duty_step);
        wave * volume
    }
}


impl PulseSweepGenerator {
    pub fn new() -> Self {
        Self {
            pulse_generator: PulseGenerator::new(),
            freq_sweep: FrequencySweep::default(),
        }
    }
}


impl SoundGenerator for PulseSweepGenerator {
    fn on_trigger_event(&mut self, registers: &ApuChannelRegisters) {
        self.pulse_generator.on_trigger_event(registers);
        self.freq_sweep = FrequencySweep::from_registers(registers);
    }

    fn tick_sound_length(&mut self, registers: &ApuChannelRegisters) {
        self.pulse_generator.tick_sound_length(registers);
    }

    fn tick_freq_sweep(&mut self, registers: &ApuChannelRegisters) {
        self.pulse_generator.tick_freq_sweep(registers);

        match self.freq_sweep.tick(registers, self.pulse_generator.wave_length) {
            // the channel was disabled because of an overflow
            FrequencySweepResult::DisableChannel => {
                self.pulse_generator.enabled = false;
            }

            // apply the changed frequency
            FrequencySweepResult::FrequencyChanged(new_wave_length) => {
                self.pulse_generator.wave_length = new_wave_length;
            }

            _ => { }
        }
    }

    fn tick_envelope_sweep(&mut self, registers: &ApuChannelRegisters) {
        self.pulse_generator.tick_envelope_sweep(registers);
    }

    fn update(&mut self, registers: &ApuChannelRegisters, cycles: Clock) {
        self.pulse_generator.update(registers, cycles);
    }

    fn is_dac_enabled(&self) -> bool {
        self.pulse_generator.is_dac_enabled()
    }

    fn get_sample(&self, registers: &ApuRegisters) -> u8 {
        self.pulse_generator.get_sample(registers)
    }
}