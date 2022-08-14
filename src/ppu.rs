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

pub const MEMORY_LOCATION_SPRITES_BEGIN:            u16 = 0x8000;
pub const MEMORY_LOCATION_BACKGROUND_MAP_BEGIN:     u16 = 0x9800;
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

    /// The currently processed pixel in the current scanline.
    current_line_pixel: u8,

    /// The number of cycles being consumed for the current scanline.
    current_line_cycles: i32,

    /// The data buffer to store the actual viewport content presented to the display.
    lcd_buffer: LcdBuffer,

    /// Flag to be set after a frame was completed.
    frame_completed: bool,
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
            current_line_pixel: 0,
            current_line_cycles: 0,
            lcd_buffer: LcdBuffer::alloc(),
            frame_completed: false,
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
        self.clock += cycles as i32;

        while self.clock > 0 {
            match self.mode {
                Mode::OamScan  => self.process_oam_scan(),
                Mode::DrawLine => self.process_draw_line(),
                Mode::HBlank   => self.process_hblank(),
                Mode::VBlank   => self.process_vblank(),
            };

            // return if 'frame completed'
            if self.frame_completed {
                self.frame_completed = false;
                return FrameState::FrameCompleted;
            }
        }

        FrameState::Processing
    }


    /// Scans the object attribute memory for the current scanline
    /// to collect the objects to be drawn in this line.
    /// Enters Mode::DrawLine after the OAM scan was completed.
    fn process_oam_scan(&mut self) {
        self.clock -= 80;

        self.current_line_pixel  = 0;
        self.current_line_cycles = 0;

        self.enter_mode(Mode::DrawLine);
    }


    /// Draws pixels of the current scanline into the LCD buffer.
    /// Enters Mode::HBlank after the drawing was completed.
    fn process_draw_line(&mut self) {
        // update clock
        self.current_line_cycles += 1;
        self.clock -= 1;

        // get the current background pixel
        let pixel = self.read_background_pixel(
            self.current_line_pixel,
            self.ly
        );

        // write pixel into LCD buffer
        self.lcd_buffer.set_pixel(
            self.current_line_pixel as u32,
            self.ly as u32,
            pixel
        );

        // set next pixel to compute
        self.current_line_pixel += 1;

        // when reached the end of the current scanline, enter HBlank mode
        if self.current_line_pixel as u32 >= SCREEN_W {
            self.enter_mode(Mode::HBlank);
        }
    }


    /// Process the HBlank mode after each drawn scanline.
    /// Enters Mode::OamScan for the next line or
    /// Mode::VBlank if the current line was the last one.
    fn process_hblank(&mut self) {
        let max_cycles       = 456_i32;
        let remaining_cycles = max_cycles - self.current_line_cycles;

        self.clock -= remaining_cycles;

        self.next_ly();
    }


    /// Process the VBlank mode after all scanlines were drawn.
    /// Enters Mode::OamScan for the first scanline of the next frame,
    /// afters the VBlank was completed.
    fn process_vblank(&mut self) {
        let cycles_per_line = 456;

        self.clock -= cycles_per_line;
        self.next_ly();

        // set the flag for 'frame completed' after switching back to scanline 0
        if self.ly == 0 {
            self.frame_completed = true;
        }
    }


    /// Switches into a given PPU mode.
    /// Updates the LCD status byte with the current mode.
    fn enter_mode(&mut self, mode: Mode) {
        self.mode = mode;

        let mut lcdc = self.mem.read_u8(MEMORY_LOCATION_LCD_STATUS);
        lcdc = lcdc & 0b_1111_1100;
        lcdc = lcdc | (self.mode as u8);

        self.mem.write_u8(MEMORY_LOCATION_LCD_STATUS, lcdc);
    }


    /// Switches to the next scanline.
    /// Checks for coincidence with the LYC byte and updates the
    /// LCD status byte as well as the current LY byte in memory.
    /// Enters either Mode::OamScan or Mode::VBlank depending on
    /// the next scanline.
    fn next_ly(&mut self) {
        if self.ly == 153 {
            self.ly = 0;
        }
        else {
            self.ly = self.ly + 1;
        }

        // update ly value in memory
        self.mem.write_u8(MEMORY_LOCATION_LY, self.ly);

        // check for ly == lyc coincidence
        {
            let lyc = self.mem.read_u8(MEMORY_LOCATION_LYC);
            let coincidence = self.ly == lyc;
            let mut lcdc = self.mem.read_u8(MEMORY_LOCATION_LCD_STATUS);
            lcdc = change_bit(lcdc, LCD_STATUS_BIT_FLAG_COINCIDENCE, coincidence);
            self.mem.write_u8(MEMORY_LOCATION_LCD_STATUS, lcdc);
        }

        // enter vblank when beyond the last scanline
        // enter OAM scan for next scanline otherwise
        if self.ly >= 144 {
            self.enter_mode(Mode::VBlank);
        }
        else {
            self.enter_mode(Mode::OamScan);
        }
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

    /// Read the pixel value of the background on a given screen position
    /// honoring the X and Y scroll values
    pub fn read_background_pixel(&self, screen_x: u8, screen_y: u8) -> u8 {
        let background_x = (screen_x as u32 + self.get_scroll_x() as u32) % 256;
        let background_y = (screen_y as u32 + self.get_scroll_y() as u32) % 256;
        let tile_x       = (background_x / 8) as u16;
        let tile_y       = (background_y / 8) as u16;
        let tile_pixel_x = (background_x % 8) as u8;
        let tile_pixel_y = (background_y % 8) as u8;
        let tile_index   = tile_y * 32 + tile_x;
        let tile_address = MEMORY_LOCATION_BACKGROUND_MAP_BEGIN + tile_index;
        let sprite       = self.mem.read_u8(tile_address as u16);

        self.read_sprite_pixel(
            sprite,
            tile_pixel_x,
            tile_pixel_y
        )
    }

    /// Read the pixel value of a sprite.
    pub fn read_sprite_pixel(&self, sprite: u8, x: u8, y: u8) -> u8 {
        let sprite_address      = MEMORY_LOCATION_SPRITES_BEGIN + (sprite as u16 * 16);
        let sprite_line_address = sprite_address + y as u16 * 2;
        let pixel_mask            = 1u8 << (7 - x);
        let byte0                 = self.mem.read_u8(sprite_line_address + 0);
        let byte1                 = self.mem.read_u8(sprite_line_address + 1);

        let pixel =
                (if (byte0 & pixel_mask) != 0 { 0x01 } else { 0x00 })
            |   (if (byte1 & pixel_mask) != 0 { 0x02 } else { 0x00 })
        ;

        pixel
    }
}
