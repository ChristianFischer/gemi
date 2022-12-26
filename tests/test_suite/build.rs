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


use std::path::{Path, PathBuf};
use tests_generator::download::download_test_roms;
use tests_generator::generators::generate_all_tests;
use tests_generator::test_generator::TestGenerator;

const BASE_PATH_ROM_FILES:  &str = "res/test_roms/";
const BASE_PATH_TESTS:      &str = "tests/";
const SOURCE_URL:           &str = "https://github.com/c-sp/gameboy-test-roms/releases/download/v4.0/gameboy-test-roms-v4.0.zip";


fn main() {
    let path = Path::new(BASE_PATH_ROM_FILES);

    if !path.is_dir() {
        download_test_roms(path, &SOURCE_URL);
    }

    generate_all_tests(&TestGenerator {
        base_path_roms: PathBuf::from(BASE_PATH_ROM_FILES),
        base_path_tests: PathBuf::from(BASE_PATH_TESTS),

        // A list of tests currently known to fail
        // Those will be ignored until their functionality is fully supported.
        tests_known_to_fail: vec![
            "blargg/cgb_sound/",
            "blargg/dmg_sound/",
            "blargg/interrupt_time/interrupt_time",
            "blargg/mem_timing/",
            "blargg/mem_timing_2/",
            "blargg/oam_bug/",
            "blargg/blargg_other/halt_bug",

            "gambatte/bgen/",
            "gambatte/bgtiledata/",
            "gambatte/bgtilemap/",
            "gambatte/cgbpal_m3/",
            "gambatte/display_startstate/",
            "gambatte/div/",
            "gambatte/dma/",
            "gambatte/dmgpalette_during_m3/",
            "gambatte/enable_display/",
            "gambatte/halt/",
            "gambatte/irq_precedence/",
            "gambatte/lcd_offset/",
            "gambatte/lcdirq_precedence/",
            "gambatte/ly0/",
            "gambatte/lyc0int_m0irq/",
            "gambatte/lyc153int_m2irq/",
            "gambatte/lycenable/",
            "gambatte/lycint_ly/",
            "gambatte/lycint_lycflag/",
            "gambatte/lycint_lycirq/",
            "gambatte/lycint_m0stat/",
            "gambatte/lycm2int/",
            "gambatte/lywrite/",
            "gambatte/m0enable/",
            "gambatte/m0int_m0irq/",
            "gambatte/m0int_m0stat/",
            "gambatte/m0int_m3stat/",
            "gambatte/m1/",
            "gambatte/m2enable/",
            "gambatte/m2int_m0irq/",
            "gambatte/m2int_m0stat/",
            "gambatte/m2int_m2irq/",
            "gambatte/m2int_m2stat/",
            "gambatte/m2int_m3stat/",
            "gambatte/miscmstatirq/",
            "gambatte/oam_access/",
            "gambatte/oamdma/",
            "gambatte/scx_during_m3/",
            "gambatte/scy/",
            "gambatte/serial/",
            "gambatte/sound/",
            "gambatte/speedchange/",
            "gambatte/sprites/",
            "gambatte/tima/",
            "gambatte/undef_ops/",
            "gambatte/vram_m3/",
            "gambatte/vramw_m3end/",
            "gambatte/window/",

            "mooneye_test_suite/acceptance/bits",
            "mooneye_test_suite/acceptance/interrupts/",
            "mooneye_test_suite/acceptance/oam_dma/",
            "mooneye_test_suite/acceptance/ppu/",
            "mooneye_test_suite/acceptance/serial/",
            "mooneye_test_suite/acceptance/acceptance_other/",
            "mooneye_test_suite/misc/",
        ]
    });
}
