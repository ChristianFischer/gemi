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

use gbemu_core::gameboy::DeviceType;
use gbemu_core::graphic_data::{Color, DmgDisplayPalette};
use testrunner::runner::{CheckResultConfig, EmulatorTestConfig, run_with_config, RunConfig, SetUpConfig};


#[test]
pub fn acid2_dmg() {
    let cfg = EmulatorTestConfig {
        setup: SetUpConfig {
            device: Some(DeviceType::GameBoyDmg),
            dmg_display_palette: Some(DmgDisplayPalette::new([
                Color::from_rgba32(0xffffffff),
                Color::from_rgba32(0xaaaaaaff),
                Color::from_rgba32(0x555555ff),
                Color::from_rgba32(0x000000ff),
            ])),
            .. SetUpConfig::with_rom_file("dmg-acid2/dmg-acid2.gb")
        },

        run_config: RunConfig {
            stop_on_halt: true,
            .. Default::default()
        },

        result: CheckResultConfig {
            compare_lcd_with_image: Some("dmg-acid2/dmg-acid2-dmg.png".to_string()),
            .. Default::default()
        },
    };

    run_with_config(cfg);
}


#[test]
pub fn acid2_gbc() {
    let cfg = EmulatorTestConfig {
        setup: SetUpConfig {
            device: Some(DeviceType::GameBoyColor),
            .. SetUpConfig::with_rom_file("cgb-acid2/cgb-acid2.gbc")
        },

        run_config: RunConfig {
            stop_on_halt: true,
            .. Default::default()
        },

        result: CheckResultConfig {
            compare_lcd_with_image: Some("cgb-acid2/cgb-acid2.png".to_string()),
            .. Default::default()
        },
    };

    run_with_config(cfg);
}


#[test]
#[ignore]
pub fn acid2_gbc_hell() {
    let cfg = EmulatorTestConfig {
        setup: SetUpConfig {
            device: Some(DeviceType::GameBoyColor),
            .. SetUpConfig::with_rom_file("cgb-acid-hell/cgb-acid-hell.gbc")
        },

        run_config: RunConfig {
            stop_on_halt: true,
            .. Default::default()
        },

        result: CheckResultConfig {
            compare_lcd_with_image: Some("cgb-acid-hell/cgb-acid-hell.png".to_string()),
            .. Default::default()
        },
    };

    run_with_config(cfg);
}