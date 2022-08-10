/*
 * Copyright (C) 2022 by Christian Fischer
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

use crate::memory::{MemoryRead, MemoryReadWriteHandle, MemoryWrite};
use crate::utils::change_bit;

pub const SCREEN_W: u32 = 160;
pub const SCREEN_H: u32 = 144;

pub const SCREEN_PIXELS: usize = (SCREEN_W * SCREEN_H) as usize;

pub const MEMORY_LOCATION_LCD_CONTROL:              u16 = 0xff40;
pub const MEMORY_LOCATION_LCD_STATUS:               u16 = 0xff41;
pub const MEMORY_LOCATION_SCY:                      u16 = 0xff42;
pub const MEMORY_LOCATION_SCX:                      u16 = 0xff43;
pub const MEMORY_LOCATION_LY:                       u16 = 0xff44;
pub const MEMORY_LOCATION_LYC:                      u16 = 0xff45;
pub const MEMORY_LOCATION_WY:                       u16 = 0xff4a;
pub const MEMORY_LOCATION_WX:                       u16 = 0xff4b;

pub const LCD_CONTROL_BIT_BG_WINDOW_ENABLED:        usize = 0;
pub const LCD_CONTROL_BIT_SPRITE_ENABLED:           usize = 1;
pub const LCD_CONTROL_BIT_SPRITE_SIZE:              usize = 2;
pub const LCD_CONTROL_BIT_BG_TILE_MAP_SELECT:       usize = 3;
pub const LCD_CONTROL_BIT_TILE_DATA_SELECT:         usize = 4;
pub const LCD_CONTROL_BIT_WINDOW_ENABLED:           usize = 5;
pub const LCD_CONTROL_BIT_WINDOW_TILE_MAP_SELECT:   usize = 6;
pub const LCD_CONTROL_BIT_LCD_ENABLED:              usize = 7;

pub const LCD_STATUS_BIT_PPU_MODE_0:                usize = 0;
pub const LCD_STATUS_BIT_PPU_MODE_1:                usize = 1;
pub const LCD_STATUS_BIT_FLAG_COINCIDENCE:          usize = 2;
pub const LCD_STATUS_BIT_ENABLE_IRQ_MODE_0:         usize = 3;
pub const LCD_STATUS_BIT_ENABLE_IRQ_MODE_1:         usize = 4;
pub const LCD_STATUS_BIT_ENABLE_IRQ_MODE_2:         usize = 5;
pub const LCD_STATUS_BIT_ENABLE_IRQ_LYC_EQ_LY:      usize = 6;
pub const LCD_STATUS_BIT_UNUSED:                    usize = 7;


type PixelBuffer160x144 = [u8; SCREEN_PIXELS];

pub struct LcdBuffer {
    pixels: PixelBuffer160x144,
}

#[derive(Copy, Clone)]
pub enum Mode {
    HBlank      = 0,
    VBlank      = 1,
    OamScan     = 2,
    DrawLine    = 3,
}

pub enum FrameState {
    Processing,
    FrameCompleted,
}


/// An object representing the gameboy's picture processing unit.
pub struct Ppu {
    clock: i32,
    mem: MemoryReadWriteHandle,

    /// The PPU's current mode.
    mode: Mode,

    /// The currently processed scanline.
    ly: u8,

    /// The data buffer to store the actual viewport content presented to the display.
    lcd_buffer: LcdBuffer,
}


impl LcdBuffer {
    pub fn alloc() -> LcdBuffer {
        LcdBuffer {
            pixels: [0x00; SCREEN_PIXELS]
        }
    }

    /// Get the width of the buffer image content.
    pub fn get_width(&self) -> u32 {
        SCREEN_W
    }

    /// Get the height of the buffer image content.
    pub fn get_height(&self) -> u32 {
        SCREEN_H
    }

    /// Get the value of a specific pixel.
    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        let index = x + (y * SCREEN_W);
        self.pixels[index as usize]
    }

    /// Set the value of a specific pixel.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let index = x + (y * SCREEN_W);
        self.pixels[index as usize] = value & 0x03;
    }

    /// Get the pixel data to be displayed.
    pub fn get_pixels(&self) -> &PixelBuffer160x144 {
        &self.pixels
    }
}


impl Ppu {
    /// Creates a new PPU object.
    pub fn new(mem: MemoryReadWriteHandle) -> Ppu {
        Ppu {
            clock: 0,
            mem,
            mode: Mode::OamScan,
            ly: 0,
            lcd_buffer: LcdBuffer::alloc(),
        }
    }


    /// Let the PPU process their data.
    /// This function takes the amount of ticks to be processed
    /// and the return value tells when VBlank finished and
    /// a whole new frame was generated.
    ///
    /// This is currently a mock implementation doing nothing
    /// than counting clock cycles and writing the current
    /// state into memory.
    pub fn update(&mut self, cycles: u32) -> FrameState {
        let mut result = FrameState::Processing;
        self.clock += cycles as i32;

        while self.clock > 0 {
            match self.mode {
                Mode::OamScan => {
                    self.clock -= 80;
                    self.enter_mode(Mode::DrawLine);
                }

                Mode::DrawLine => {
                    self.clock -= 172;
                    self.enter_mode(Mode::HBlank);
                }

                Mode::HBlank => {
                    self.clock -= 204;
                    self.ly    += 1;

                    self.mem.write_u8(MEMORY_LOCATION_LY, self.ly);

                    if self.ly >= 144 {
                        self.enter_mode(Mode::VBlank);
                    }
                    else {
                        self.enter_mode(Mode::OamScan);
                    }
                }

                Mode::VBlank => {
                    self.clock -= 4560;
                    self.ly = 0;
                    self.mem.write_u8(MEMORY_LOCATION_LY, self.ly);
                    self.enter_mode(Mode::OamScan);
                    result = FrameState::FrameCompleted;
                }
            };
        }

        result
    }


    fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode;

        let mut lcdc = self.mem.read_u8(MEMORY_LOCATION_LCD_STATUS);
        lcdc = lcdc & 0b_1111_1100;
        lcdc = lcdc | (self.mode as u8);

        self.mem.write_u8(MEMORY_LOCATION_LCD_STATUS, lcdc);
    }


    /// Get the LCD buffer which contains the actual data sent to the device's display.
    pub fn get_lcd(&self) -> &LcdBuffer {
        &self.lcd_buffer
    }

    /// Get the display viewport offset on X axis.
    pub fn get_scroll_x(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_SCX)
    }

    /// Get the display viewport offset on Y axis.
    pub fn get_scroll_y(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_SCY)
    }

    /// Get the window position on X axis.
    pub fn get_window_x(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_WX)
    }

    /// Get the window position on Y axis.
    pub fn get_window_y(&self) -> u8 {
        self.mem.read_u8(MEMORY_LOCATION_WY)
    }
}
