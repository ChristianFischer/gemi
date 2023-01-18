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
use crate::apu::apu::ApuState;
use crate::apu::channels::channel::{ChannelComponent, TriggerAction, default_on_register_changed};
use crate::apu::channels::generator::SoundGenerator;
use crate::apu::channels::wave_duty::WaveDuty;
use crate::apu::registers::{ApuChannelRegisters, ApuRegisters};
use crate::gameboy::Clock;


/// A sound generator to generate a pulse wave. The wave is based is based on a wave duty value
/// and a volume computed by an envelope function.
pub struct PulseGenerator {
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


impl PulseGenerator {
    pub fn new() -> Self {
        Self {
            wave_duty:      WaveDuty::default(),
            wave_length:    0,
            wave_timer:     0,
            wave_duty_step: 0,
        }
    }
}


impl ChannelComponent for PulseGenerator {
    fn on_register_changed(&mut self, number: u16, registers: &ApuChannelRegisters, apu_state: &ApuState) -> TriggerAction {
        match number {
            1 => {
                let wave_duty_index = (registers.nr1 >> 6) & 0x03;
                self.wave_duty = WaveDuty::by_index(wave_duty_index);
            }

            3 | 4 => {
                let wave_length_register_value = (registers.nr3 as Clock) | (((registers.nr4 as Clock) & 0x07) << 8);
                let wave_length = (2048 - wave_length_register_value) * 4;

                self.wave_length = wave_length;
                self.wave_timer  = wave_length;
            }

            _ => { }
        }

        default_on_register_changed(number, registers, apu_state)
    }
}


impl SoundGenerator for PulseGenerator {
    fn create() -> Self {
        Self::new()
    }


    fn get_frequency(&self) -> Clock {
        self.wave_length
    }


    fn set_frequency(&mut self, frequency: Clock) {
        self.wave_length = frequency;
    }


    fn update(&mut self, cycles: Clock) {
        if self.wave_length != 0 {
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


    fn get_sample(&self, _registers: &ApuRegisters) -> u8 {
        let wave = self.wave_duty.get_wave_at(self.wave_duty_step);
        wave
    }
}
