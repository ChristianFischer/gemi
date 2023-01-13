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

use crate::apu::channels::channel::features::{FEATURE_FREQUENCY_SWEEP_DISABLED, FEATURE_LENGTH_TIMER_DISABLED, FEATURE_VOLUME_ENVELOPE_DISABLED};
use crate::apu::channels::envelope::Envelope;
use crate::apu::channels::freq_sweep::{FrequencySweep, FrequencySweepResult};
use crate::apu::channels::generator::SoundGenerator;
use crate::apu::channels::length_timer::LengthTimer;
use crate::apu::dac::DigitalAudioConverter;
use crate::apu::registers::{ApuChannelRegisters, ApuRegisters};
use crate::gameboy::Clock;

pub mod features {
    pub const FEATURE_LENGTH_TIMER_DISABLED : u8        = 0;
    pub const FEATURE_LENGTH_TIMER_6_BIT : u8           = 6;
    pub const FEATURE_LENGTH_TIMER_8_BIT : u8           = 8;

    pub const FEATURE_FREQUENCY_SWEEP_DISABLED : u8     = 0;
    pub const FEATURE_FREQUENCY_SWEEP_ENABLED : u8      = 1;

    pub const FEATURE_VOLUME_ENVELOPE_DISABLED : u8     = 0;
    pub const FEATURE_VOLUME_ENVELOPE_ENABLED : u8      = 1;
}


/// The type of a channel.
pub enum ChannelType {
    Ch1Pulse1,
    Ch2Pulse2,
    Ch3Wave,
    Ch4Noise,
}


/// A trait for any component used inside an audio channel.
/// This trait allows components to receive changes on their registers and to get notified
/// when a channel was triggered by setting the trigger bit.
pub trait ChannelComponent {
    /// Called when the value of a register was changed by writing on it.
    fn on_register_changed(&mut self, number: u16, registers: &ApuChannelRegisters) {
        _ = (number, registers);
    }

    /// Called when the channel was triggered by setting bit 7 of it's NRx4 register.
    /// This should start the channel to generate sound.
    fn on_trigger_event(&mut self) {
    }
}


/// Represents a single channel inside the GameBoy APU.
/// Each channel contains a distinct sound generator which generates
/// an audio signal and a DAC to convert the digital value into an
/// analogue sound wave.
pub struct Channel<
    G : SoundGenerator,
    const FEATURE_LENGTH_TIMER : u8,
    const FEATURE_FREQUENCY_SWEEP : u8,
    const FEATURE_VOLUME_ENVELOPE : u8,
> {
    /// Stores whether the channel is enabled or not.
    channel_enabled: bool,

    /// The current channels type.
    channel_type: ChannelType,

    /// The sound generator associated with this channel.
    generator: G,

    /// A length timer controlling how long the sound generator will run.
    length_timer: LengthTimer<FEATURE_LENGTH_TIMER>,

    /// A frequency sweep function to modify the generators wave length.
    freq_sweep: FrequencySweep,

    /// An envelope function to provide the volume for the generated wave.
    vol_envelope: Envelope,

    /// A digital audio converter to convert the digital sound value
    /// into a sound wave.
    dac: DigitalAudioConverter,
}


impl<
    G : SoundGenerator,
    const FEATURE_LENGTH_TIMER: u8,
    const FEATURE_FREQUENCY_SWEEP : u8,
    const FEATURE_VOLUME_ENVELOPE : u8,
> Channel<
    G,
    FEATURE_LENGTH_TIMER,
    FEATURE_FREQUENCY_SWEEP,
    FEATURE_VOLUME_ENVELOPE,
> {
    /// Creates a new sound channel with it's type and sound generator instance.
    pub fn new(channel_type: ChannelType) -> Self {
        Self {
            channel_enabled: true,
            channel_type,

            generator:      G::create(),

            length_timer:   LengthTimer::default(),
            freq_sweep:     FrequencySweep::default(),
            vol_envelope:   Envelope::default(),

            dac:            DigitalAudioConverter::new(),
        }
    }


    /// Checks whether this channel has a length timer.
    pub fn has_feature_length_timer() -> bool {
        FEATURE_LENGTH_TIMER != FEATURE_LENGTH_TIMER_DISABLED
    }


    /// Checks whether this channel has frequency sweep.
    pub fn has_feature_frequency_sweep() -> bool {
        FEATURE_FREQUENCY_SWEEP != FEATURE_FREQUENCY_SWEEP_DISABLED
    }


    /// Checks whether this channel has a volume envelope.
    pub fn has_feature_volume_envelope() -> bool {
        FEATURE_VOLUME_ENVELOPE != FEATURE_VOLUME_ENVELOPE_DISABLED
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


    /// Invokes a functor on each active component of this channel,
    /// including the generator component.
    fn for_each_component<F>(&mut self, mut func: F)
        where F : FnMut(&mut dyn ChannelComponent)
    {
        if Self::has_feature_length_timer() {
            func(&mut self.length_timer);
        }

        if Self::has_feature_frequency_sweep() {
            func(&mut self.freq_sweep);
        }

        if Self::has_feature_volume_envelope() {
            func(&mut self.vol_envelope);
        }

        func(&mut self.generator);
    }


    /// Enables or disables the DAC depending on the channels current settings.
    fn update_dac_enabled(&mut self) {
        // check whether the generator would disable the DAC
        let generator_dac_enabled = self.generator.is_dac_enabled();

        // on channels with an volume envelope, register NRx2 may also disable the DAC
        let envelope_dac_enabled = if Self::has_feature_volume_envelope() {
            self.vol_envelope.get_dac_enabled()
        }
        else {
            true
        };

        // enable or disable the DAC based on the current configuration
        self.dac.set_enabled(generator_dac_enabled && envelope_dac_enabled);
    }


    /// Fires the notification when a register of this channel was written to.
    pub fn fire_register_changed(&mut self, number: u16, registers: &ApuRegisters) {
        let channel_registers = &registers.channels[self.get_channel_ordinal() as usize];
        self.for_each_component(|c| c.on_register_changed(number, channel_registers));
    }


    /// Fires the trigger event when the channel was triggered by writing NRx4 bit 7.
    pub fn fire_trigger_event(&mut self) {
        self.channel_enabled = true;

        self.for_each_component(|c| c.on_trigger_event());

        self.update_dac_enabled();
    }


    /// Called by the frame sequencer to update the channels sound length timer.
    pub fn tick_length_timer(&mut self) {
        if Self::has_feature_length_timer() {
            let expired = self.length_timer.tick();

            // when the timer expires, the generator will be disabled
            if expired {
                self.channel_enabled = false;
            }
        }
    }


    /// Called by the frame sequencer to update the frequency sweep of channel 1.
    pub fn tick_freq_sweep(&mut self) {
        if Self::has_feature_frequency_sweep() {
            let frequency = self.generator.get_frequency();
            let result    = self.freq_sweep.tick(frequency);

            match result {
                // the channel was disabled because of an overflow
                FrequencySweepResult::DisableChannel => {
                    self.channel_enabled = false;
                }

                // apply the changed frequency
                FrequencySweepResult::FrequencyChanged(new_wave_length) => {
                    self.generator.set_frequency(new_wave_length);
                }

                _ => { }
            }
        }
    }


    /// Called by the frame sequencer to update the envelope function.
    pub fn tick_envelope_sweep(&mut self) {
        if Self::has_feature_volume_envelope() {
            self.vol_envelope.tick();
        }
    }


    /// Updates the current channel with the time passed since last call.
    pub fn update(&mut self, cycles: Clock) {
        self.generator.update(cycles);
    }


    /// Get the audio sample generated by the channels sound generator and
    /// converted by the channels DAC.
    pub fn get_sample(&self, registers: &ApuRegisters) -> i16 {
        let value = if self.channel_enabled {
            // take the current sample from the sound generator
            let generated_sample = self.generator.get_sample(registers);

            // get the volume level from the envelope function, if available
            let volume = if Self::has_feature_volume_envelope() {
                self.vol_envelope.get_current_volume()
            }
            else {
                1
            };

            // compute the samples amplitude, modified by volume
            generated_sample * volume
        }
        else {
            // a disabled channel just spawns zero
            0
        };

        // convert into 'analogue' signal via DAC
        let sample = self.dac.convert(value);

        sample
    }
}

