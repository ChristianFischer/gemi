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

use crate::apu::apu_client::SAMPLE_BUFFER_SIZE;
use crate::apu::sample::StereoSample;
use crate::utils::SerializableBuffer;

use alloc::{vec, vec::Vec};


/// A queue of audio samples.
/// This object can be used by clients to implement an audio output device.
///
/// The audio queue is implemented as a ring buffer, so repeatedly inserting new samples
/// will overwrite old ones. Thus, the buffer will never grow, and if a client can't catch up
/// reading samples, the audio will never become outdated.
///
/// To read and insert samples, a read and insert cursor will store the according position.
/// Reading will only be possible until the current position of the insert cursor. After reading
/// up to the insert cursor, the AudioQueue will be considered "empty".
/// Insert operations may invalidate the reading cursor when samples get inserted faster than
/// being read, which could lead to audio glitches.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AudioQueue
{
    /// The storage of the audio samples.
    audio_queue: SerializableBuffer<StereoSample>,

    /// The size of the audio queue above.
    audio_queue_length: usize,

    /// The cursor, in which slot the buffer is inserted.
    insert_pos: usize,

    /// The cursor, from which array position the next sample will be read.
    read_pos: usize,
}


impl AudioQueue {
    /// Creates a new audio queue with an allocated sample buffer.
    pub fn alloc(size: usize) -> Self {
        // ensure a valid minimum size for the audio queue
        let size = usize::max(size, 16 * SAMPLE_BUFFER_SIZE);

        Self {
            audio_queue: vec![StereoSample::default(); size].into(),
            audio_queue_length: size,
            insert_pos: 0,
            read_pos: 0,
        }
    }


    /// Get the number of audio samples available.
    pub fn get_samples_available(&self) -> usize {
        (self.audio_queue_length + self.insert_pos - self.read_pos) % self.audio_queue_length
    }


    /// Insert samples into the audio queue.
    ///
    /// If the list of samples to be added is larger than the buffer size, the samples will be truncated.
    pub fn insert(&mut self, samples: &[StereoSample]) {
        let mut need_to_reset_read_pos = false;
        let mut samples               = samples;

        while !samples.is_empty() {
            let space_until_buffer_end = self.audio_queue_length - self.insert_pos;
            let samples_to_insert      = usize::min(samples.len(), space_until_buffer_end);
            let insert_range           = self.insert_pos..self.insert_pos + samples_to_insert;

            // Check if the current read position would be overwritten by the samples to be inserted.
            // This would mean the client could not keep up consuming the samples being produced,
            // and on the next read, the client could read a mix of new and old samples.
            if self.read_pos > insert_range.start && self.read_pos <= insert_range.end {
                need_to_reset_read_pos = true;
            }

            self.audio_queue[insert_range].copy_from_slice(&samples[..samples_to_insert]);
            self.insert_pos = (self.insert_pos + samples_to_insert) % self.audio_queue_length;
            samples = &samples[samples_to_insert..];
        }

        // If required, we 'reset' the reading cursor to a position distant to the insert position.
        // This may cause a single audio pop, but should avoid noise when the
        // emulator would constantly modify the audio queue on the read position.
        if need_to_reset_read_pos {
            self.read_pos = (self.insert_pos + self.audio_queue_length / 2) % self.audio_queue_length;
        }
    }


    /// Tries to take 'num_samples' audio samples from the queue.
    /// This returns either a vector with the requested number of samples
    /// or [None] if not enough samples are available.
    pub fn take(&mut self, num_samples: usize) -> Option<Vec<StereoSample>> {
        if self.get_samples_available() < num_samples {
            return None;
        }

        let mut target_buffer  = vec![StereoSample::default(); num_samples];
        let mut samples_copied = 0;

        // read 'num_samples' samples from the audio queue
        while samples_copied < num_samples {
            let samples_to_copy = usize::min(self.audio_queue_length - self.read_pos, num_samples - samples_copied);
            let src_range       = self.read_pos..self.read_pos + samples_to_copy;
            let dst_range       = samples_copied..samples_copied + samples_to_copy;

            target_buffer[dst_range].copy_from_slice(&self.audio_queue[src_range]);
            samples_copied += samples_to_copy;

            self.read_pos = (self.read_pos + samples_to_copy) % self.audio_queue_length;
        }

        Some(target_buffer)
    }


    /// Tries to take all audio samples from the queue.
    /// This returns either a vector with all samples
    /// or [None] if there are currently no samples available.
    pub fn take_all(&mut self) -> Option<Vec<StereoSample>> {
        let num_samples = self.get_samples_available();
        if num_samples > 0 {
            self.take(num_samples)
        }
        else {
            None
        }
    }
}
