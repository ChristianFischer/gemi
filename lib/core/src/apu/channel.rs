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

use crate::apu::dac::DigitalAudioConverter;
use crate::apu::generators::SoundGenerator;
use crate::apu::registers::ApuRegisters;
use crate::gameboy::Clock;


/// The type of a channel.
pub enum ChannelType {
    Ch1Pulse1,
    Ch2Pulse2,
    Ch3Wave,
    Ch4Noise,
}


/// Represents a single channel inside the GameBoy APU.
/// Each channel contains a distinct sound generator which generates
/// an audio signal and a DAC to convert the digital value into an
/// analogue sound wave.
pub struct Channel<G : SoundGenerator> {
    /// The current channels type.
    channel_type: ChannelType,

    /// The sound generator associated with this channel.
    generator: G,

    /// A digital audio converter to convert the digital sound value
    /// into a sound wave.
    dac: DigitalAudioConverter,
}


impl<G : SoundGenerator> Channel<G> {
    /// Creates a new sound channel with it's type and sound generator instance.
    pub fn new(channel_type: ChannelType, generator: G) -> Self {
        Self {
            channel_type,
            generator,
            dac: DigitalAudioConverter::new()
        }
    }


    /// Get the type of this channel.
    pub fn get_channel_type(&self) -> &ChannelType {
        &self.channel_type
    }


    /// Get the ordinal number of this channel, starting with zero.
    /// So CH1 has the ordinal 0, CH2 ordinal 1 and so on.
    pub fn get_channel_ordinal(&self) -> u8 {
        match self.channel_type {
            ChannelType::Ch1Pulse1 => 0,
            ChannelType::Ch2Pulse2 => 1,
            ChannelType::Ch3Wave   => 2,
            ChannelType::Ch4Noise  => 3,
        }
    }


    /// Get the sound generator of this channel.
    pub fn get_generator_mut(&mut self) -> &mut G {
        &mut self.generator
    }


    /// Fires the trigger event when the channel was triggered by writing NRx4 bit 7.
    pub fn fire_trigger_event(&mut self, registers: &ApuRegisters) {
        let channel_registers = &registers.channels[self.get_channel_ordinal() as usize];
        self.generator.on_trigger_event(channel_registers);

        // enable or disable the DAC based on the current configuration
        let dac_enabled = self.generator.is_dac_enabled();
        self.dac.set_enabled(dac_enabled);
    }


    /// Called by the frame sequencer to update the channels sound length timer.
    pub fn tick_sound_length(&mut self, registers: &ApuRegisters) {
        let channel_registers = &registers.channels[self.get_channel_ordinal() as usize];
        self.generator.tick_sound_length(channel_registers);
    }


    /// Called by the frame sequencer to update the frequency sweep of channel 1.
    pub fn tick_freq_sweep(&mut self, registers: &ApuRegisters) {
        let channel_registers = &registers.channels[self.get_channel_ordinal() as usize];
        self.generator.tick_freq_sweep(channel_registers);
    }


    /// Called by the frame sequencer to update the envelope function.
    pub fn tick_envelope_sweep(&mut self, registers: &ApuRegisters) {
        let channel_registers = &registers.channels[self.get_channel_ordinal() as usize];
        self.generator.tick_envelope_sweep(channel_registers);
    }


    /// Updates the current channel with the time passed since last call.
    pub fn update(&mut self, registers: &ApuRegisters, cycles: Clock) {
        let channel_registers = &registers.channels[self.get_channel_ordinal() as usize];
        self.generator.update(channel_registers, cycles);
    }


    /// Get the audio sample generated by the channels sound generator and
    /// converted by the channels DAC.
    pub fn get_sample(&self) -> i16 {
        let value  = self.generator.get_sample();
        let sample = self.dac.convert(value);

        sample
    }
}

