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
use testrunner::runner::run_with_config;
use tests_shared::test_config::*;


const MOONEYE_RESULT_SEQ_PASS : &[u8] = &[3, 5, 8, 13, 21, 34];
const MOONEYE_RESULT_SEQ_FAIL : &[u8] = &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42];


mod acceptance {
    use super::*;


    #[test]
    #[ignore]
    fn test_add_sp_e_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/add_sp_e_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_add_sp_e_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/add_sp_e_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }


    mod bits {
        use super::*;


        #[test]
        #[ignore]
        fn test_mem_oam_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/mem_oam.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_mem_oam_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/mem_oam.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_reg_f_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/reg_f.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_reg_f_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/reg_f.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_unused_hwio_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/unused_hwio-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_unused_hwio_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/unused_hwio-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_unused_hwio_sgb2() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy2),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/unused_hwio-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }



    #[test]
    #[ignore]
    fn test_boot_div_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div-dmgABCmgb.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_div_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div-S.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_div_sgb2() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy2),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div-S.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_div2_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div2-S.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_div2_sgb2() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy2),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div2-S.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_hwio_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_hwio-dmgABCmgb.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_hwio_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_hwio-S.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_hwio_sgb2() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy2),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_hwio-S.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_regs_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_regs-dmgABC.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_regs_mgb_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_regs-mgb.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_regs_mgb_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_regs-mgb.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_regs_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_regs-sgb.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_boot_regs_sgb2() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy2),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_regs-sgb2.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_call_cc_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_call_cc_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_call_cc_timing2_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing2.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_call_cc_timing2_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing2.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_call_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_call_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_call_timing2_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing2.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_call_timing2_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing2.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_div_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/div_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_div_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/div_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_di_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/di_timing-GS.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_di_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/di_timing-GS.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_di_timing_sgb2() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy2),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/di_timing-GS.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ei_sequence_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_sequence.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ei_sequence_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_sequence.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ei_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ei_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_halt_ime0_ei_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_ei.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_halt_ime0_ei_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_ei.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_halt_ime0_nointr_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_nointr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_halt_ime0_nointr_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_nointr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_halt_ime1_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_halt_ime1_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_halt_ime1_timing2_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing2-GS.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_halt_ime1_timing2_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing2-GS.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_halt_ime1_timing2_sgb2() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy2),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing2-GS.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_if_ie_registers_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/if_ie_registers.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_if_ie_registers_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/if_ie_registers.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }


    mod instr {
        use super::*;


        #[test]
        #[ignore]
        fn test_daa_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/instr/daa.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_daa_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/instr/daa.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }


    mod interrupts {
        use super::*;


        #[test]
        #[ignore]
        fn test_ie_push_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/interrupts/ie_push.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_ie_push_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/interrupts/ie_push.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }



    #[test]
    #[ignore]
    fn test_intr_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/intr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_intr_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/intr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_jp_cc_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_cc_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_jp_cc_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_cc_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_jp_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_jp_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ld_hl_sp_e_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ld_hl_sp_e_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ld_hl_sp_e_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ld_hl_sp_e_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }


    mod oam_dma {
        use super::*;


        #[test]
        #[ignore]
        fn test_basic_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/basic.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_basic_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/basic.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_reg_read_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/reg_read.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_reg_read_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/reg_read.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_sources_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/sources-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_sources_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/sources-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_sources_sgb2() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy2),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/sources-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }



    #[test]
    #[ignore]
    fn test_oam_dma_restart_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_restart.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_oam_dma_restart_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_restart.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_oam_dma_start_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_start.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_oam_dma_start_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_start.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_oam_dma_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_oam_dma_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_pop_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/pop_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_pop_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/pop_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }


    mod ppu {
        use super::*;


        #[test]
        #[ignore]
        fn test_hblank_ly_scx_timing_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/hblank_ly_scx_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_hblank_ly_scx_timing_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/hblank_ly_scx_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_hblank_ly_scx_timing_sgb2() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy2),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/hblank_ly_scx_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_1_2_timing_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_1_2_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_1_2_timing_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_1_2_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_1_2_timing_sgb2() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy2),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_1_2_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_0_timing_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_0_timing_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_mode0_timing_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_mode0_timing_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_mode0_timing_sprites_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing_sprites.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_mode0_timing_sprites_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing_sprites.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_mode3_timing_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode3_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_mode3_timing_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode3_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_oam_ok_timing_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_oam_ok_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_intr_2_oam_ok_timing_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_oam_ok_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_lcdon_timing_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_lcdon_timing_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_lcdon_timing_sgb2() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy2),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_lcdon_write_timing_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_write_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_lcdon_write_timing_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_write_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_lcdon_write_timing_sgb2() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy2),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_write_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_stat_irq_blocking_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_irq_blocking.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_stat_irq_blocking_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_irq_blocking.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_stat_lyc_onoff_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_lyc_onoff.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_stat_lyc_onoff_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_lyc_onoff.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_vblank_stat_intr_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/vblank_stat_intr-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_vblank_stat_intr_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/vblank_stat_intr-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_vblank_stat_intr_sgb2() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy2),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/vblank_stat_intr-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }



    #[test]
    #[ignore]
    fn test_push_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/push_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_push_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/push_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_rapid_di_ei_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rapid_di_ei.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_rapid_di_ei_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rapid_di_ei.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_reti_intr_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_intr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_reti_intr_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_intr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_reti_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_reti_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ret_cc_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_cc_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ret_cc_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_cc_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ret_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_ret_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_rst_timing_dmg() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rst_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }



    #[test]
    #[ignore]
    fn test_rst_timing_sgb() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::SuperGameBoy),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rst_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                .. CheckResultConfig::default()
            },
        };

        let mut gb = run_with_config(cfg);

        let test_result_message = gb.serial.take_output();
        assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
        assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
    }


    mod serial {
        use super::*;


        #[test]
        #[ignore]
        fn test_boot_sclk_align_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/serial/boot_sclk_align-dmgABCmgb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }


    mod timer {
        use super::*;


        #[test]
        #[ignore]
        fn test_div_write_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/div_write.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_div_write_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/div_write.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rapid_toggle_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/rapid_toggle.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rapid_toggle_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/rapid_toggle.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim00_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim00_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim00_div_trigger_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim00_div_trigger_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim01_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim01_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim01_div_trigger_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim01_div_trigger_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim10_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim10_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim10_div_trigger_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim10_div_trigger_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim11_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim11_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim11_div_trigger_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tim11_div_trigger_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tima_reload_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_reload.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tima_reload_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_reload.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tima_write_reloading_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tima_write_reloading_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tma_write_reloading_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tma_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_tma_write_reloading_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tma_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }

}


mod emulator_only {
    use super::*;

    mod mbc1 {
        use super::*;


        #[test]
        fn test_bits_bank1_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank1.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_bits_bank1_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank1.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_bits_bank2_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_bits_bank2_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_bits_mode_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_mode.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_bits_mode_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_mode.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_bits_ramg_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_bits_ramg_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_multicart_rom_8mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/multicart_rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_multicart_rom_8mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/multicart_rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_ram_256kb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_256kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_ram_256kb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_256kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_ram_64kb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_64kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_ram_64kb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_64kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_16mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_16mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_1mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_1mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_2mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_2mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_4mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_4mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_512kb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_512kb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_8mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        fn test_rom_8mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }


    mod mbc2 {
        use super::*;


        #[test]
        #[ignore]
        fn test_bits_ramg_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_bits_ramg_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_bits_romb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_romb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_bits_romb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_romb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_bits_unused_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_unused.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_bits_unused_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_unused.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_ram_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/ram.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_ram_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/ram.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_1mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_1mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_2mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_2mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_512kb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_512kb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }


    mod mbc5 {
        use super::*;


        #[test]
        #[ignore]
        fn test_rom_16mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_16mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_1mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_1mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_2mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_2mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_32mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_32Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_32mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_32Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_4mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_4mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_512kb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_512kb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_64mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_64Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_64mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_64Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_8mb_dmg() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }



        #[test]
        #[ignore]
        fn test_rom_8mb_sgb() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::SuperGameBoy),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    .. CheckResultConfig::default()
                },
            };

            let mut gb = run_with_config(cfg);

            let test_result_message = gb.serial.take_output();
            assert_ne!(MOONEYE_RESULT_SEQ_FAIL, test_result_message, "ROM sent FAILED sequence.");
            assert_eq!(MOONEYE_RESULT_SEQ_PASS, test_result_message, "Missing 'Passed' Sequence from ROM, got {test_result_message:?} instead.");
        }

    }

}

