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
use crate::apu::channels::channel::{ChannelComponent, default_on_read_register, default_on_write_register, TriggerAction};
use crate::apu::channels::frequency::Frequency;
use crate::apu::channels::generator::SoundGenerator;
use crate::gameboy::Clock;
use crate::utils::{as_bit_flag, get_bit};


const NR30_NON_READABLE_BITS : u8       = 0b_0111_1111;
const NR32_NON_READABLE_BITS : u8       = 0b_1001_1111;
const NR33_WRITE_ONLY_FREQUENCY : u8    = 0b_1111_1111;
const NR34_NON_READABLE_BITS : u8       = 0b_0011_1000;
const NR34_WRITE_ONLY_FREQUENCY : u8    = 0b_0000_0111;
const NR34_WRITE_ONLY_TRIGGER_BIT : u8  = 0b_1000_0000;


pub struct WaveGenerator {
    /// Stores whether the DAC of this channel is enabled or not.
    dac_enabled: bool,

    /// The output level value read from the NR32 register.
    output_level: u8,

    /// The number of bits to shift the value of the sound sample to lower it's volume.
    volume_shift: u8,

    /// The frequency is controlling how fast the wave will be played.
    /// The value will be read from NRx3 and NRx4 register of the channel.
    frequency: Frequency,

    /// Current value of the wave timer. The value is built based on the frequency
    /// configured by the application. It will be decreased with each CPU T-Cycle
    /// and restarts when reaching zero.
    wave_timer: Clock,

    /// The current position where to read the value from the wave duty.
    wave_step: u8,
}


impl WaveGenerator {
    pub fn new() -> Self {
        Self {
            dac_enabled:        false,
            output_level:       0,
            volume_shift:       0,
            frequency:          Frequency::default(),
            wave_timer:         0,
            wave_step:          0,
        }
    }


    /// After writing to either NRx3 or NRx4, update the channel's wave length
    fn refresh_wave_length(&mut self) {
        self.wave_timer = self.frequency.to_countdown();
    }
}


impl ChannelComponent for WaveGenerator {
    fn on_read_register(&self, number: u16) -> u8 {
        match number {
            0 => NR30_NON_READABLE_BITS | as_bit_flag(self.dac_enabled, 7),
            2 => NR32_NON_READABLE_BITS | (self.output_level << 5),
            3 => NR33_WRITE_ONLY_FREQUENCY,
            4 => NR34_WRITE_ONLY_FREQUENCY | NR34_NON_READABLE_BITS | NR34_WRITE_ONLY_TRIGGER_BIT,
            _ => default_on_read_register(number)
        }
    }


    fn on_write_register(&mut self, number: u16, value: u8, apu_state: &ApuState) -> TriggerAction {
        match number {
            0 => {
                self.dac_enabled = get_bit(value, 7);
                return if self.dac_enabled {
                    TriggerAction::EnableDac
                }
                else {
                    TriggerAction::DisableDac
                };
            }

            2 => {
                self.output_level = (value >> 5) & 0x03;
                self.volume_shift = match self.output_level {
                    0b00 => 4, // mute
                    0b01 => 0, // 100% (no change)
                    0b10 => 1, //  50%
                    0b11 => 2, //  25%
                    _ => unreachable!()
                };
            }

            3 | 4 => {
                self.frequency.set_by_register(number, value);
                self.refresh_wave_length();
            }

            _ => { }
        }

        default_on_write_register(number, value, apu_state)
    }


    fn on_reset(&mut self) {
        *self = Self::new();
    }
}


impl SoundGenerator for WaveGenerator {
    fn create() -> Self {
        Self::new()
    }


    fn get_frequency(&self) -> Frequency {
        self.frequency
    }


    fn set_frequency(&mut self, frequency: Frequency) {
        self.frequency = frequency;
    }


    fn update(&mut self, cycles: Clock) {
        let mut remaining_cycles = cycles;

        while remaining_cycles > 0 {
            let run_cycles = min(self.wave_timer, remaining_cycles);

            self.wave_timer  = self.wave_timer.saturating_sub(run_cycles);

            // when the wave timer expires, it will be restarted and the
            // position inside the wave ram proceeds
            if self.wave_timer == 0 {
                self.wave_timer = self.frequency.to_countdown();
                self.wave_step  = self.wave_step.wrapping_add(1);
            }

            remaining_cycles = remaining_cycles.saturating_sub(run_cycles);
        }
    }


    fn is_dac_enabled(&self) -> bool {
        self.dac_enabled
    }


    fn get_sample(&self, apu_state: &ApuState) -> u8 {
        // since wave ram contains two samples per byte,
        // the index within the wave ram is wave_step / 2.
        let index = (self.wave_step >> 1) & 0x0f;

        // read the byte from wave RAM
        let value = apu_state.wave_ram.0[index as usize];

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