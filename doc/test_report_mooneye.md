## Mooneye Test ROM Results

|                                                                          | dmg  | mgb  | gbc  | gba  | sgb  | sgb2 |
|--------------------------------------------------------------------------|:----:|:----:|:----:|:----:|:----:|:----:|
| **mooneye_test_suite/acceptance/bits**                                   |      |      |      |      |      |      |
| mem_oam.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| reg_f.gb                                                                 |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| unused_hwio-GS.gb                                                        |  ✔️   |  ✔️   |      |      |  ✔️   |  ✔️   |
| **mooneye_test_suite/acceptance/instr**                                  |      |      |      |      |      |      |
| daa.gb                                                                   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| **mooneye_test_suite/acceptance/interrupts**                             |      |      |      |      |      |      |
| ie_push.gb                                                               |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| **mooneye_test_suite/acceptance/oam_dma**                                |      |      |      |      |      |      |
| basic.gb                                                                 |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| reg_read.gb                                                              |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| sources-GS.gb                                                            |  ❌   |  ❌   |      |      |  ❌   |  ❌   |
| **mooneye_test_suite/acceptance/ppu**                                    |      |      |      |      |      |      |
| hblank_ly_scx_timing-GS.gb                                               |  ❌   |  ❌   |      |      |  ❌   |  ❌   |
| intr_1_2_timing-GS.gb                                                    |  ❌   |  ❌   |      |      |  ❌   |  ❌   |
| intr_2_0_timing.gb                                                       |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| intr_2_mode0_timing.gb                                                   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| intr_2_mode0_timing_sprites.gb                                           |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| intr_2_mode3_timing.gb                                                   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| intr_2_oam_ok_timing.gb                                                  |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| lcdon_timing-GS.gb                                                       |  ❌   |  ❌   |      |      |  ❌   |  ❌   |
| lcdon_write_timing-GS.gb                                                 |  ❌   |  ❌   |      |      |  ❌   |  ❌   |
| stat_irq_blocking.gb                                                     |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| stat_lyc_onoff.gb                                                        |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| vblank_stat_intr-GS.gb                                                   |  ✔️   |  ✔️   |      |      |  ✔️   |  ✔️   |
| **mooneye_test_suite/acceptance/serial**                                 |      |      |      |      |      |      |
| boot_sclk_align-dmgABCmgb.gb                                             |  ❌   |  ❌   |      |      |      |      |
| **mooneye_test_suite/acceptance/timer**                                  |      |      |      |      |      |      |
| div_write.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rapid_toggle.gb                                                          |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tim00.gb                                                                 |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tim00_div_trigger.gb                                                     |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tim01.gb                                                                 |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tim01_div_trigger.gb                                                     |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tim10.gb                                                                 |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tim10_div_trigger.gb                                                     |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tim11.gb                                                                 |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tim11_div_trigger.gb                                                     |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tima_reload.gb                                                           |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tima_write_reloading.gb                                                  |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| tma_write_reloading.gb                                                   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| **mooneye_test_suite/acceptance/acceptance_other**                       |      |      |      |      |      |      |
| add_sp_e_timing.gb                                                       |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| boot_div-S.gb                                                            |      |      |      |      |  ❌   |  ❌   |
| boot_div-dmg0.gb                                                         |      |      |      |      |      |      |
| boot_div-dmgABCmgb.gb                                                    |  ❌   |  ❌   |      |      |      |      |
| boot_div2-S.gb                                                           |      |      |      |      |  ❌   |  ❌   |
| boot_hwio-S.gb                                                           |      |      |      |      |  ❌   |  ❌   |
| boot_hwio-dmg0.gb                                                        |      |      |      |      |      |      |
| boot_hwio-dmgABCmgb.gb                                                   |  ❌   |  ❌   |      |      |      |      |
| boot_regs-dmg0.gb                                                        |      |      |      |      |      |      |
| boot_regs-dmgABC.gb                                                      |  ✔️   |      |      |      |      |      |
| boot_regs-mgb.gb                                                         |      |  ❌   |      |      |      |      |
| boot_regs-sgb.gb                                                         |      |      |      |      |  ✔️   |      |
| boot_regs-sgb2.gb                                                        |      |      |      |      |      |  ✔️   |
| call_cc_timing.gb                                                        |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| call_cc_timing2.gb                                                       |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| call_timing.gb                                                           |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| call_timing2.gb                                                          |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| di_timing-GS.gb                                                          |  ❌   |  ❌   |      |      |  ❌   |  ❌   |
| div_timing.gb                                                            |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| ei_sequence.gb                                                           |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| ei_timing.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| halt_ime0_ei.gb                                                          |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| halt_ime0_nointr_timing.gb                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| halt_ime1_timing.gb                                                      |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| halt_ime1_timing2-GS.gb                                                  |  ❌   |  ❌   |      |      |  ❌   |  ❌   |
| if_ie_registers.gb                                                       |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| intr_timing.gb                                                           |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| jp_cc_timing.gb                                                          |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| jp_timing.gb                                                             |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| ld_hl_sp_e_timing.gb                                                     |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| oam_dma_restart.gb                                                       |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| oam_dma_start.gb                                                         |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| oam_dma_timing.gb                                                        |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| pop_timing.gb                                                            |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| push_timing.gb                                                           |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| rapid_di_ei.gb                                                           |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| ret_cc_timing.gb                                                         |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| ret_timing.gb                                                            |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| reti_intr_timing.gb                                                      |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| reti_timing.gb                                                           |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| rst_timing.gb                                                            |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |  ❌   |
| **mooneye_test_suite/emulator_only/mbc1**                                |      |      |      |      |      |      |
| bits_bank1.gb                                                            |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| bits_bank2.gb                                                            |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| bits_mode.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| bits_ramg.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| multicart_rom_8Mb.gb                                                     |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| ram_256kb.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| ram_64kb.gb                                                              |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_16Mb.gb                                                              |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_1Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_2Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_4Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_512kb.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_8Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| **mooneye_test_suite/emulator_only/mbc2**                                |      |      |      |      |      |      |
| bits_ramg.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| bits_romb.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| bits_unused.gb                                                           |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| ram.gb                                                                   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_1Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_2Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_512kb.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| **mooneye_test_suite/emulator_only/mbc5**                                |      |      |      |      |      |      |
| rom_16Mb.gb                                                              |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_1Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_2Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_32Mb.gb                                                              |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_4Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_512kb.gb                                                             |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_64Mb.gb                                                              |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| rom_8Mb.gb                                                               |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |  ✔️   |
| **mooneye_test_suite/misc/bits**                                         |      |      |      |      |      |      |
| unused_hwio-C.gb                                                         |      |      |  ❌   |  ❌   |      |      |
| **mooneye_test_suite/misc/ppu**                                          |      |      |      |      |      |      |
| vblank_stat_intr-C.gb                                                    |      |      |  ❌   |  ❌   |      |      |
| **mooneye_test_suite/misc/misc_other**                                   |      |      |      |      |      |      |
| boot_div-A.gb                                                            |      |      |      |  ❌   |      |      |
| boot_div-cgb0.gb                                                         |      |      |      |      |      |      |
| boot_div-cgbABCDE.gb                                                     |      |      |      |      |      |      |
| boot_hwio-C.gb                                                           |      |      |  ❌   |  ❌   |      |      |
| boot_regs-A.gb                                                           |      |      |      |  ✔️   |      |      |
| boot_regs-cgb.gb                                                         |      |      |      |      |      |      |

