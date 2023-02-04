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

use flagset::{flags, FlagSet};


flags! {

    /// A list of flags which are part of the LCD control register
    /// to control the behaviour of the PPU.
    pub enum LcdControlFlag : u8 {
        /// DMG: Checks if background and window should be drawn.
        /// GBC: Master priority flag.
        BackgroundAndWindowEnabled,

        /// If enabled, sprites from the OAM table will be drawn on the screen.
        SpritesEnabled,

        /// Controls the size of sprites:
        /// 0 - 8x8 pixel sprites
        /// 1 - 8x16 pixel sprites
        SpritesSize,

        /// Selects the tile map to be displayed as a background image:
        /// 0 - 0x9800,
        /// 1 - 0x9C00,
        BackgroundTileMapSelect,

        /// Selects the address mode which defines where to load tile images
        /// from the memory:
        /// 0 - 0x8800 - 0x97ff,
        /// 1 - 0x8000 - 0x8fff,
        TileDataSelect,

        /// Enables or disables the window.
        WindowEnabled,

        /// Selects the tile map to be displayed within the window,
        /// similar to the background tile map selection:
        /// 0 - 0x9800,
        /// 1 - 0x9C00,
        WindowTileMapSelect,

        /// Enables or disables the whole LCD and PPU.
        LcdEnabled,
    }


    /// A set of flags which are part of the LCD Status register
    /// to control which PPU mode will trigger the LCD stat interrupt.
    pub enum LcdInterruptFlag : u8 {
        InterruptByHBlank       = 0b_0000_1000,
        InterruptByVBlank       = 0b_0001_0000,
        InterruptByOam          = 0b_0010_0000,
        InterruptByCoincidence  = 0b_0100_0000,
    }
}

pub type LcdControl         = FlagSet<LcdControlFlag>;
pub type LcdInterruptFlags  = FlagSet<LcdInterruptFlag>;
