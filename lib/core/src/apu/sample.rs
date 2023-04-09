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

use std::ops;

/// The data type to be used to store audio sample data.
pub type SampleType = f32;


/// A sample of a single channel as to be transmitted to the emulator frontend.
#[derive(Default, Copy, Clone)]
pub struct Sample {
    value: SampleType
}


/// A sample of left and right channel as to be transmitted to the emulator frontend.
#[derive(Default, Copy, Clone)]
pub struct StereoSample {
    pub left: Sample,
    pub right: Sample,
}


impl Sample {
    pub fn new(value: SampleType) -> Self {
        Self {
            value
        }
    }


    /// Get this sample's value.
    pub fn get_value(&self) -> SampleType {
        self.value
    }
}


impl ops::Add<Sample> for Sample {
    type Output = Sample;

    fn add(self, rhs: Sample) -> Self::Output {
        Sample {
            value: self.value + rhs.value,
        }
    }
}


impl ops::AddAssign<Sample> for Sample {
    fn add_assign(&mut self, rhs: Sample) {
        self.value += rhs.value;
    }
}


impl ops::Mul<SampleType> for Sample {
    type Output = Sample;

    fn mul(self, rhs: SampleType) -> Self::Output {
        Sample {
            value: self.value * rhs,
        }
    }
}


impl ops::Div<SampleType> for Sample {
    type Output = Sample;

    fn div(self, rhs: SampleType) -> Self::Output {
        Sample {
            value: self.value / rhs,
        }
    }
}



impl ops::Add<StereoSample> for StereoSample {
    type Output = StereoSample;

    fn add(self, rhs: StereoSample) -> Self::Output {
        StereoSample {
            left:  self.left  + rhs.left,
            right: self.right + rhs.right,
        }
    }
}


impl ops::AddAssign<StereoSample> for StereoSample {
    fn add_assign(&mut self, rhs: StereoSample) {
        self.left  += rhs.left;
        self.right += rhs.right;
    }
}


impl ops::Mul<SampleType> for StereoSample {
    type Output = StereoSample;

    fn mul(self, rhs: SampleType) -> Self::Output {
        StereoSample {
            left:  self.left  * rhs,
            right: self.right * rhs,
        }
    }
}