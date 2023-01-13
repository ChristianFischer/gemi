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

use crate::apu::channels::channel::ChannelComponent;
use crate::apu::registers::ApuChannelRegisters;
use crate::utils::get_bit;


/// Whether to increment or decrement the volume
pub enum Direction {
    Decrement,
    Increment,
}


/// An envelope function to modify a channels volume over time.
/// When started, it takes a volume and period value from the NRx2 register of the channel.
/// Each time the envelope unit is called by the APUs frame sequencer, the period timer is
/// decreased until it reaches zero. Then the volume will be increased or decreased depending
/// on the setting in NRx2 and the timer restarted.
/// Additionally, setting the volume and the direction bit to 0 will disable the channel's DAC
/// as well, which is also covered by this object.
pub struct Envelope {
    /// Flag to store whether the period timer is enabled or not.
    /// This is used to stop the timer once the volume reached it's minimum or maximum value
    /// and further calls wont have any effect.
    enabled: bool,

    /// Stores whether the channel's DAC will be enabled or disabled.
    /// The DAC is disabled when both the volume and direction bits or NRx2 are all zero.
    dac_enabled: bool,

    /// The current volume used for sound generation.
    /// It will be initialized by the values of NRx2 and changed each time the period timer becomes zero.
    volume: u8,

    /// The length of each period in ticks by the frame sequencer.
    period_length: u8,

    /// The current value of the period timer, which is decreased on each tick.
    period_timer: u8,

    /// Whether to increment or decrement the volume.
    direction: Direction,
}


impl Direction {
    pub fn from_bit(bit: bool) -> Self {
        match bit {
            false => Direction::Decrement,
            true  => Direction::Increment,
        }
    }
}


impl Envelope {
    /// Checks whether the channels DAC should be enabled.
    pub fn get_dac_enabled(&self) -> bool {
        self.dac_enabled
    }


    /// Get the current volume.
    pub fn get_current_volume(&self) -> u8 {
        self.volume
    }


    /// Reloads the timer once it reached zero.
    fn reload_envelope_timer(&mut self) {
        self.period_timer = self.period_length;
    }


    /// Receives the periodic call from the frame sequencer.
    /// Decrease the timer and modify the volume once the timer becomes zero.
    pub fn tick(&mut self) {
        if self.enabled {
            self.period_timer = self.period_timer.saturating_sub(1);

            // when the timer elapses
            if self.period_timer == 0 {
                // reload the timer
                self.reload_envelope_timer();

                // advance the volume; disable the timer,
                // when the volume reaches it's max or min value
                match self.direction {
                    Direction::Decrement => {
                        self.volume = self.volume.saturating_sub(1);

                        if self.volume == 0 {
                            self.enabled = false;
                        }
                    }

                    Direction::Increment => {
                        self.volume = self.volume.saturating_add(1);

                        if self.volume >= 0x0f {
                            self.volume = 0x0f;
                            self.enabled = false;
                        }
                    }
                }
            }
        }
    }
}


impl ChannelComponent for Envelope {
    fn on_register_changed(&mut self, number: u16, registers: &ApuChannelRegisters) {
        match number {
            2 => {
                let volume        = (registers.nr2 >> 4) & 0x0f;
                let period        = (registers.nr2 >> 0) & 0x07;
                let direction_bit = get_bit(registers.nr2, 3);
                let dac_enabled   = (registers.nr2 & 0xf8) != 0;
                let enabled       = period != 0;

                self.enabled        = enabled;
                self.dac_enabled    = dac_enabled;
                self.volume         = volume;
                self.period_length  = period;
                self.direction      = Direction::from_bit(direction_bit);
            }

            _ => { }
        }
    }


    fn on_trigger_event(&mut self) {
        self.reload_envelope_timer();
    }
}


impl Default for Envelope {
    fn default() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            volume:         0,
            period_length:  0,
            period_timer:   0,
            direction: Direction::Decrement,
        }
    }
}

