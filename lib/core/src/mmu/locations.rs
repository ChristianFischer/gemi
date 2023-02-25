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

pub const MEMORY_LOCATION_VRAM_BEGIN:               u16 = 0x8000;
pub const MEMORY_LOCATION_SPRITES_BEGIN:            u16 = 0x8000;
pub const MEMORY_LOCATION_BACKGROUND_MAP_BEGIN:     u16 = 0x9800;
pub const MEMORY_LOCATION_WRAM_BANK_0_BEGIN:        u16 = 0xc000;
pub const MEMORY_LOCATION_WRAM_BANK_1_BEGIN:        u16 = 0xc000;
pub const MEMORY_LOCATION_OAM_BEGIN:                u16 = 0xfe00;
pub const MEMORY_LOCATION_OAM_END:                  u16 = 0xfe9f;
pub const MEMORY_LOCATION_JOYP:                     u16 = 0xff00;
pub const MEMORY_LOCATION_SB:                       u16 = 0xff01;
pub const MEMORY_LOCATION_SC:                       u16 = 0xff02;
pub const MEMORY_LOCATION_REGISTER_DIV:             u16 = 0xff04;
pub const MEMORY_LOCATION_REGISTER_TIMA:            u16 = 0xff05;
pub const MEMORY_LOCATION_REGISTER_TMA:             u16 = 0xff06;
pub const MEMORY_LOCATION_REGISTER_TAC:             u16 = 0xff07;
pub const MEMORY_LOCATION_APU_NR10:                 u16 = 0xff10;
pub const MEMORY_LOCATION_APU_NR11:                 u16 = 0xff11;
pub const MEMORY_LOCATION_APU_NR12:                 u16 = 0xff12;
pub const MEMORY_LOCATION_APU_NR13:                 u16 = 0xff13;
pub const MEMORY_LOCATION_APU_NR14:                 u16 = 0xff14;
pub const MEMORY_LOCATION_APU_NR20:                 u16 = 0xff15;
pub const MEMORY_LOCATION_APU_NR21:                 u16 = 0xff16;
pub const MEMORY_LOCATION_APU_NR22:                 u16 = 0xff17;
pub const MEMORY_LOCATION_APU_NR23:                 u16 = 0xff18;
pub const MEMORY_LOCATION_APU_NR24:                 u16 = 0xff19;
pub const MEMORY_LOCATION_APU_NR30:                 u16 = 0xff1a;
pub const MEMORY_LOCATION_APU_NR31:                 u16 = 0xff1b;
pub const MEMORY_LOCATION_APU_NR32:                 u16 = 0xff1c;
pub const MEMORY_LOCATION_APU_NR33:                 u16 = 0xff1d;
pub const MEMORY_LOCATION_APU_NR34:                 u16 = 0xff1e;
pub const MEMORY_LOCATION_APU_NR40:                 u16 = 0xff1f;
pub const MEMORY_LOCATION_APU_NR41:                 u16 = 0xff20;
pub const MEMORY_LOCATION_APU_NR42:                 u16 = 0xff21;
pub const MEMORY_LOCATION_APU_NR43:                 u16 = 0xff22;
pub const MEMORY_LOCATION_APU_NR44:                 u16 = 0xff23;
pub const MEMORY_LOCATION_APU_NR50:                 u16 = 0xff24;
pub const MEMORY_LOCATION_APU_NR51:                 u16 = 0xff25;
pub const MEMORY_LOCATION_APU_NR52:                 u16 = 0xff26;
pub const MEMORY_LOCATION_LCD_CONTROL:              u16 = 0xff40;
pub const MEMORY_LOCATION_LCD_STATUS:               u16 = 0xff41;
pub const MEMORY_LOCATION_SCY:                      u16 = 0xff42;
pub const MEMORY_LOCATION_SCX:                      u16 = 0xff43;
pub const MEMORY_LOCATION_LY:                       u16 = 0xff44;
pub const MEMORY_LOCATION_LYC:                      u16 = 0xff45;
pub const MEMORY_LOCATION_DMA_ADDRESS:              u16 = 0xff46;
pub const MEMORY_LOCATION_PALETTE_BG:               u16 = 0xff47;
pub const MEMORY_LOCATION_PALETTE_OBP0:             u16 = 0xff48;
pub const MEMORY_LOCATION_PALETTE_OBP1:             u16 = 0xff49;
pub const MEMORY_LOCATION_WY:                       u16 = 0xff4a;
pub const MEMORY_LOCATION_WX:                       u16 = 0xff4b;
pub const MEMORY_LOCATION_VBK:                      u16 = 0xff4f;
pub const MEMORY_LOCATION_BOOT_ROM_DISABLE:         u16 = 0xff50;
pub const MEMORY_LOCATION_HDMA1:                    u16 = 0xff51;
pub const MEMORY_LOCATION_HDMA2:                    u16 = 0xff52;
pub const MEMORY_LOCATION_HDMA3:                    u16 = 0xff53;
pub const MEMORY_LOCATION_HDMA4:                    u16 = 0xff54;
pub const MEMORY_LOCATION_HDMA5:                    u16 = 0xff55;
pub const MEMORY_LOCATION_BCPS:                     u16 = 0xff68;
pub const MEMORY_LOCATION_BCPD:                     u16 = 0xff69;
pub const MEMORY_LOCATION_OCPS:                     u16 = 0xff6a;
pub const MEMORY_LOCATION_OCPD:                     u16 = 0xff6b;
pub const MEMORY_LOCATION_OPRI:                     u16 = 0xff6c;
pub const MEMORY_LOCATION_SVBK:                     u16 = 0xff70;
pub const MEMORY_LOCATION_INTERRUPTS_FLAGGED:       u16 = 0xff0f;
pub const MEMORY_LOCATION_INTERRUPTS_ENABLED:       u16 = 0xffff;
