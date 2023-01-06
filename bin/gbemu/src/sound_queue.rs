/*
 * Copyright (C) 2023 by Christian Fischer
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

use sdl2::audio::*;
use sdl2::Sdl;


const SAMPLE_FREQ   : i32 = 48_000;
const CHANNEL_COUNT : u8 = 2;


/// SoundQueue to feed sound data into the audio device.
pub struct SoundQueue {
    /// The device for audio output
    audio_device:  AudioDevice<SoundQueueCallback>,
}


/// SDL callback object to fetch audio samples.
struct SoundQueueCallback {
    /// List of audio samples received from the emulator.
    samples_queue: Vec<Vec<i16>>,

    /// The current list of samples to be sent to the audio device.
    samples: Vec<i16>,

    /// The index within the current samples list.
    samples_index: usize,
}


impl SoundQueue {
    /// Creates a new SoundQueue
    pub fn create(sdl: &Sdl) -> Result<Self, String> {
        let sdl_audio = sdl.audio()?;

        let audio_spec = AudioSpecDesired {
            freq:     Some(SAMPLE_FREQ),
            channels: Some(CHANNEL_COUNT),
            samples:  None,
        };

        let audio_device = sdl_audio.open_playback(
            None,
            &audio_spec,
            |_spec| SoundQueueCallback {
                samples_queue: vec![],
                samples: vec![],
                samples_index: 0,
            }
        )?;

        audio_device.resume();

        Ok (Self {
            audio_device,
        })
    }


    /// Push new samples into the queue.
    pub fn push_audio_samples(&mut self, samples: Vec<i16>) {
        // lock the audio device to get access to the queue callback
        self.audio_device.lock().samples_queue.push(samples);
    }
}


impl Drop for SoundQueue {
    fn drop(&mut self) {
        self.audio_device.pause();
    }
}


impl AudioCallback for SoundQueueCallback {
    type Channel = i16;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        let mut insert_index = 0;
        let mut sample = 0;

        'insert_loop:
        while insert_index < out.len() {
            // check if out of data of the current samples
            while self.samples_index >= self.samples.len() {
                // try to take the first list from the queue
                if let Some(samples) = self.samples_queue.first() {
                    self.samples = samples.clone();
                    self.samples_index = 0;
                    self.samples_queue.remove(0);
                }
                else {
                    // no more samples available, exit the loop
                    break 'insert_loop;
                }
            }

            // get the next sample
            sample = self.samples[self.samples_index];
            self.samples_index += 1;

            out[insert_index] = sample;
            insert_index += 1;
        }

        // when the queue could not be filled completely,
        // repeat the last known sample
        while insert_index < out.len() {
            out[insert_index] = sample;
            insert_index += 1;
        }
    }
}
