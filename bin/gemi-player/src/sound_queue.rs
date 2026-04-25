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

use crate::BoxError;
use libgemi::core::apu::audio_output::AudioOutputSpec;
use libgemi::core::apu::audio_queue::AudioQueue;
use libgemi::core::apu::sample::{SampleType, StereoSample};
use libgemi::core::apu::Apu;
use sdl3::audio::*;
use sdl3::Sdl;


const SAMPLE_FREQ    : u32   = 48_000;
const CHANNEL_COUNT  : u8    = 2;
const DEFAULT_VOLUME : f32   = 0.10;

// provide space to store 1/4 second of audio
const AUDIO_QUEUE_SIZE: usize = (SAMPLE_FREQ as usize) / 4;


/// SoundQueue to feed sound data into the audio device.
pub struct SoundQueue {
    /// The device for audio output
    audio_device: AudioDevice,

    /// The audio stream to feed audio data into the audio device.
    audio_stream:  AudioStreamWithCallback<SoundQueueCallback>,

    /// Mirror value of the configured volume in the queue callback.
    /// Used to avoid unnecessary locking of the callback object.
    volume: f32,
}


/// SDL callback object to fetch audio samples.
struct SoundQueueCallback {
    /// Buffer containing audio samples received from the backend.
    buffer: AudioQueue,

    /// The current volume.
    volume: f32,
}


impl SoundQueue {
    /// Creates a new SoundQueue
    pub fn create(sdl: &Sdl, apu: &mut Apu) -> Result<Self, BoxError> {
        let sdl_audio = sdl.audio()?;

        let audio_spec = AudioSpec {
            freq:     Some(SAMPLE_FREQ as i32),
            channels: Some(CHANNEL_COUNT as i32),
            format:   Some(SampleType::audio_format()),
        };

        // open a channel to the APU backend to receive audio data
        apu.get_audio_output().set_audio_spec(
            AudioOutputSpec {
                sample_rate: SAMPLE_FREQ,
            }
        )?;

        let audio_device = sdl_audio.open_playback_device(&audio_spec)?;

        let audio_stream = sdl_audio.open_playback_stream_with_callback(
            &audio_device,
            &audio_spec,
            SoundQueueCallback {
                buffer: AudioQueue::alloc(AUDIO_QUEUE_SIZE),
                volume: DEFAULT_VOLUME,
            }
        )?;

        audio_stream.resume()?;

        Ok (Self {
            audio_device,
            audio_stream,
            volume: DEFAULT_VOLUME
        })
    }


    /// Set the playback volume.
    pub fn set_volume(&mut self, volume: f32) {
        let volume_clamped = volume.clamp(0.0, 1.0);

        if self.volume != volume_clamped {
            self.volume = volume_clamped;

            if let Some(mut context) = self.audio_stream.lock() {
                context.volume = volume_clamped;
            }
        }
    }


    /// Get the current playback volume.
    pub fn get_volume(&self) -> f32 {
        self.volume
    }


    /// Push audio samples into the audio queue.
    pub fn push_samples(&mut self, samples: &[StereoSample]) {
        if let Some(mut audio) = self.audio_stream.lock() {
            audio.buffer.insert(samples);
        }
    }
}


impl Drop for SoundQueue {
    fn drop(&mut self) {
        self.audio_device.pause();
    }
}


impl AudioCallback<SampleType> for SoundQueueCallback {
    fn callback(&mut self, out: &mut AudioStream, requested: i32) {
        if requested < 0 {
            return;
        }

        // get the number of frames requested
        let requested_frames = requested as usize / CHANNEL_COUNT as usize;

        // try to fill the buffer with samples from the backend
        let samples = match self.buffer.take(requested_frames) {
            // convert received stereo samples into a flat array of f32 samples and apply the volume
            Some(samples) => samples
                    .into_iter()
                    .flat_map(|stereo| [stereo.left, stereo.right])
                    .map(|sample| sample.get_value())
                    .map(|sample| sample * self.volume)
                    .collect::<Vec<_>>(),

            // if no samples are available, fill the buffer with silence
            None => vec![0.0; requested_frames * CHANNEL_COUNT as usize],
        };

        // insert into the audio stream
        let result = out.put_data_f32(samples.as_slice());

        if let Err(err) = result {
            eprintln!("Error writing audio data: {err}");
        }
    }
}
