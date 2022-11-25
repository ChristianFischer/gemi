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


mod cgb_sound {
    use super::*;


    #[test]
    #[ignore]
    fn test_cgb_sound() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("blargg/cgb_sound/cgb_sound.gb")
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

        let output = gb.serial.take_output_as_text();
        match output.trim().split('\n').into_iter().last() {
            Some("Passed all tests") => { }
            Some("Passed") => { }
            _ => { panic!("Unexpected output:\n{}", output); }
        }
    }


    mod rom_singles {
        use super::*;


        #[test]
        #[ignore]
        fn test_01_registers() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/01-registers.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_02_len_ctr() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/02-len ctr.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_03_trigger() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/03-trigger.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_04_sweep() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/04-sweep.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_05_sweep_details() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/05-sweep details.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_06_overflow_on_trigger() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/06-overflow on trigger.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_07_len_sweep_period_sync() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/07-len sweep period sync.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_08_len_ctr_during_power() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/08-len ctr during power.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_09_wave_read_while_on() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/09-wave read while on.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_10_wave_trigger_while_on() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/10-wave trigger while on.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_11_regs_after_power() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/11-regs after power.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_12_wave() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/12-wave.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }

    }

}


mod cpu_instrs {
    use super::*;


    #[test]
    fn test_cpu_instrs() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("blargg/cpu_instrs/cpu_instrs.gb")
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

        let output = gb.serial.take_output_as_text();
        match output.trim().split('\n').into_iter().last() {
            Some("Passed all tests") => { }
            Some("Passed") => { }
            _ => { panic!("Unexpected output:\n{}", output); }
        }
    }


    mod individual {
        use super::*;


        #[test]
        fn test_01_special() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/01-special.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_02_interrupts() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/02-interrupts.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_03_op_sp_hl() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/03-op sp,hl.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_04_op_r_imm() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/04-op r,imm.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_05_op_rp() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/05-op rp.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_06_ld_r_r() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/06-ld r,r.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_07_jr_jp_call_ret_rst() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_08_misc_instrs() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/08-misc instrs.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_09_op_r_r() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/09-op r,r.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_10_bit_ops() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/10-bit ops.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        fn test_11_op_a_hl_() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/11-op a,(hl).gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }

    }

}


mod dmg_sound {
    use super::*;


    #[test]
    #[ignore]
    fn test_dmg_sound() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("blargg/dmg_sound/dmg_sound.gb")
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

        let output = gb.serial.take_output_as_text();
        match output.trim().split('\n').into_iter().last() {
            Some("Passed all tests") => { }
            Some("Passed") => { }
            _ => { panic!("Unexpected output:\n{}", output); }
        }
    }


    mod rom_singles {
        use super::*;


        #[test]
        #[ignore]
        fn test_01_registers() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/01-registers.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_02_len_ctr() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/02-len ctr.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_03_trigger() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/03-trigger.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_04_sweep() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/04-sweep.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_05_sweep_details() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/05-sweep details.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_06_overflow_on_trigger() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/06-overflow on trigger.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_07_len_sweep_period_sync() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/07-len sweep period sync.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_08_len_ctr_during_power() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/08-len ctr during power.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_09_wave_read_while_on() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/09-wave read while on.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_10_wave_trigger_while_on() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/10-wave trigger while on.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_11_regs_after_power() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/11-regs after power.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_12_wave_write_while_on() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/12-wave write while on.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }

    }

}



#[test]
#[ignore]
fn test_halt_bug() {
    let cfg = EmulatorTestConfig {
        setup: SetUpConfig {
            device: Some(DeviceType::GameBoyDmg),
            enable_serial_output: true,
            .. SetUpConfig::with_rom_file("blargg/halt_bug.gb")
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

    let output = gb.serial.take_output_as_text();
    match output.trim().split('\n').into_iter().last() {
        Some("Passed all tests") => { }
        Some("Passed") => { }
        _ => { panic!("Unexpected output:\n{}", output); }
    }
}


mod instr_timing {
    use super::*;


    #[test]
    #[ignore]
    fn test_instr_timing() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("blargg/instr_timing/instr_timing.gb")
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

        let output = gb.serial.take_output_as_text();
        match output.trim().split('\n').into_iter().last() {
            Some("Passed all tests") => { }
            Some("Passed") => { }
            _ => { panic!("Unexpected output:\n{}", output); }
        }
    }

}


mod interrupt_time {
    use super::*;


    #[test]
    #[ignore]
    fn test_interrupt_time() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("blargg/interrupt_time/interrupt_time.gb")
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

        let output = gb.serial.take_output_as_text();
        match output.trim().split('\n').into_iter().last() {
            Some("Passed all tests") => { }
            Some("Passed") => { }
            _ => { panic!("Unexpected output:\n{}", output); }
        }
    }

}


mod mem_timing {
    use super::*;

    mod individual {
        use super::*;


        #[test]
        #[ignore]
        fn test_01_read_timing() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/01-read_timing.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_02_write_timing() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/02-write_timing.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_03_modify_timing() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/03-modify_timing.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }

    }



    #[test]
    #[ignore]
    fn test_mem_timing() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("blargg/mem_timing/mem_timing.gb")
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

        let output = gb.serial.take_output_as_text();
        match output.trim().split('\n').into_iter().last() {
            Some("Passed all tests") => { }
            Some("Passed") => { }
            _ => { panic!("Unexpected output:\n{}", output); }
        }
    }

}


mod mem_timing_2 {
    use super::*;


    #[test]
    #[ignore]
    fn test_mem_timing() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("blargg/mem_timing-2/mem_timing.gb")
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

        let output = gb.serial.take_output_as_text();
        match output.trim().split('\n').into_iter().last() {
            Some("Passed all tests") => { }
            Some("Passed") => { }
            _ => { panic!("Unexpected output:\n{}", output); }
        }
    }


    mod rom_singles {
        use super::*;


        #[test]
        #[ignore]
        fn test_01_read_timing() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/01-read_timing.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_02_write_timing() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/02-write_timing.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_03_modify_timing() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/03-modify_timing.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }

    }

}


mod oam_bug {
    use super::*;


    #[test]
    #[ignore]
    fn test_oam_bug() {
        let cfg = EmulatorTestConfig {
            setup: SetUpConfig {
                device: Some(DeviceType::GameBoyDmg),
                enable_serial_output: true,
                .. SetUpConfig::with_rom_file("blargg/oam_bug/oam_bug.gb")
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

        let output = gb.serial.take_output_as_text();
        match output.trim().split('\n').into_iter().last() {
            Some("Passed all tests") => { }
            Some("Passed") => { }
            _ => { panic!("Unexpected output:\n{}", output); }
        }
    }


    mod rom_singles {
        use super::*;


        #[test]
        #[ignore]
        fn test_1_lcd_sync() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/1-lcd_sync.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_2_causes() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/2-causes.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_3_non_causes() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/3-non_causes.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_4_scanline_timing() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/4-scanline_timing.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_5_timing_bug() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/5-timing_bug.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_6_timing_no_bug() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/6-timing_no_bug.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_7_timing_effect() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/7-timing_effect.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }



        #[test]
        #[ignore]
        fn test_8_instr_effect() {
            let cfg = EmulatorTestConfig {
                setup: SetUpConfig {
                    device: Some(DeviceType::GameBoyDmg),
                    enable_serial_output: true,
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/8-instr_effect.gb")
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

            let output = gb.serial.take_output_as_text();
            match output.trim().split('\n').into_iter().last() {
                Some("Passed all tests") => { }
                Some("Passed") => { }
                _ => { panic!("Unexpected output:\n{}", output); }
            }
        }

    }

}

