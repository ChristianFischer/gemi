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

use crate::apu::sample::StereoSample;


/// Size of the sample buffer to be submitted to the client.
pub const SAMPLE_BUFFER_SIZE: usize = 1024;


/// Type alias for the sample buffer, holding [SAMPLE_BUFFER_SIZE] stereo samples.
pub type SampleBuffer = [StereoSample; SAMPLE_BUFFER_SIZE];


/// A trait for clients that can receive audio samples from the APU.
///
/// Samples will be received as a batch of [SAMPLE_BUFFER_SIZE] stereo samples.
pub trait ApuClient {
    /// Receives a batch of [SAMPLE_BUFFER_SIZE] stereo samples.
    fn push_samples(&mut self, samples: &SampleBuffer);
}


/// A dummy implementation of the ApuClient trait without functionality.
#[derive(Default)]
pub struct NullApuClient;

impl ApuClient for NullApuClient {
    fn push_samples(&mut self, _samples: &SampleBuffer) { }
}
