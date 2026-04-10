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

use crate::core::ppu::lcd_buffer::LcdBuffer;
use crate::core::ppu::ppu_client::PpuClient;
use gemi_core::device_type::{DeviceConfig, EmulationType};
use gemi_core::ppu::graphic_data::{Color, DmgDisplayPalette, DmgLcdPixel};


/// Implementation of a GameBoy display.
///
/// The display receives pixel data from the emulator core and writes them
/// into an [LcdBuffer] object.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameBoyDisplay {
    device_config: DeviceConfig,

    /// If in DMG mode, a set of RGB colors to translate the LCD intensity values
    /// into RGB colors to be displayed on color screens.
    dmg_display_palette: DmgDisplayPalette,

    /// The data buffer to store the actual viewport content presented to the display.
    lcd_buffer: Box<LcdBuffer>,
}


impl GameBoyDisplay {
    /// Creates a new display instance for a given device configuration.
    pub fn for_device(device_config: DeviceConfig) -> Self {
        // initialize the DMG palette with a default palette for the device
        let dmg_display_palette = DmgDisplayPalette::default_for_device(device_config);

        // select the blank color for the device
        let blank_color = Self::make_blank_color(device_config, dmg_display_palette);

        Self {
            device_config,
            dmg_display_palette,
            lcd_buffer: Box::new(LcdBuffer::alloc_with_color(blank_color))
        }
    }


    /// Replaces the DMG display palette of this display instance.
    /// Resets the LCD buffer to the blank color, if in DMG mode.
    pub fn with_dmg_palette(mut self, dmg_display_palette: DmgDisplayPalette) -> Self {
        self.set_dmg_display_palette(dmg_display_palette);
        self
    }


    /// Get the LCD data buffer containing the display's pixel data.
    pub  fn get_lcd(&self) -> &LcdBuffer {
        &self.lcd_buffer
    }


    /// Set the palette to be used to translate DMG LCD color values into RGBA colors.
    /// Resets the LCD buffer to the blank color, if in DMG mode.
    pub fn set_dmg_display_palette(&mut self, palette: DmgDisplayPalette) {
        self.dmg_display_palette = palette;

        // reset the LCD buffer to the blank color, if in DMG mode
        if self.device_config.emulation == EmulationType::DMG {
            let blank_color = Self::make_blank_color(self.device_config, self.dmg_display_palette);
            self.lcd_buffer.fill(blank_color);
        }
    }


    /// Get the current palette to be used to translate DMG LCD color intensities into RGBA colors
    pub fn get_dmg_display_palette(&self) -> &DmgDisplayPalette {
        &self.dmg_display_palette
    }


    /// Get the RGBA color for any color color index.
    pub fn translate_dmg_color_index(&self, pixel: &DmgLcdPixel) -> &Color {
        self.get_dmg_display_palette().get_color(pixel)
    }


    fn get_blank_color(&self) -> Color {
        Self::make_blank_color(self.device_config, self.dmg_display_palette)
    }


    /// Get the blank color for a disabled screen.
    fn make_blank_color(device_config: DeviceConfig, dmg_display_palette: DmgDisplayPalette) -> Color {
        match device_config.emulation {
            EmulationType::DMG => dmg_display_palette.get_colors()[0],
            EmulationType::GBC => Color::white(),
        }
    }
}


impl PpuClient for GameBoyDisplay {
    fn clear_screen(&mut self) {
        let blank_color = self.get_blank_color();
        self.lcd_buffer.fill(blank_color);
    }


    fn put_dmg_pixel(&mut self, x: u32, y: u32, pixel: DmgLcdPixel) {
        let color = *self.translate_dmg_color_index(&pixel);
        self.lcd_buffer.set_pixel(x, y, color);
    }


    fn put_color_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.lcd_buffer.set_pixel(x, y, color);
    }


    fn on_frame_finished(&mut self) {
    }
}