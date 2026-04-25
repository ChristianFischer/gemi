/*
 * Copyright (C) 2026 by Christian Fischer
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

use crate::core::apu::apu_client::{ApuClient, SampleBuffer, SAMPLE_BUFFER_SIZE};
use crate::core::apu::audio_queue::AudioQueue;
use crate::core::apu::sample::StereoSample;


/// The size of the audio queue.
const AUDIO_QUEUE_SIZE: usize = 16 * SAMPLE_BUFFER_SIZE;


/// Implementation of the GameBoy [ApuClient] interface.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameBoyAudio
{
    audio_queue: AudioQueue,
}


impl GameBoyAudio {
    pub fn new() -> Self {
        Self {
            audio_queue: AudioQueue::alloc(AUDIO_QUEUE_SIZE),
        }
    }


    /// Get the number of audio samples available.
    pub fn get_samples_available(&self) -> usize {
        self.audio_queue.get_samples_available()
    }


    /// Tries to take 'num_samples' audio samples from the queue.
    /// This returns either a vector with the requested number of samples
    /// or [None] if not enough samples are available.
    pub fn take(&mut self, num_samples: usize) -> Option<Vec<StereoSample>> {
        self.audio_queue.take(num_samples)
    }


    /// Tries to take all audio samples from the queue.
    /// This returns either a vector with all samples
    /// or [None] if there are currently no samples available.
    pub fn take_all(&mut self) -> Option<Vec<StereoSample>> {
        self.audio_queue.take_all()
    }
}


impl ApuClient for GameBoyAudio {
    fn push_samples(&mut self, samples: &SampleBuffer) {
        self.audio_queue.insert(samples);
    }
}
