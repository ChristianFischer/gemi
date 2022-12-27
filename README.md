# gb-emu

Another GameBoy emulator written in Rust.

My primary goal of this project is to learn Rust and as well something about "what the hell is going on inside of this weird things called CPU".
The second goal is to write an GameBoy emulator suitable to play some old games of my childhood.  
Feel free to make suggestions about missing features or games which are not supported properly.

# Feature List

|                                          |     |
|:-----------------------------------------|:---:|
| Memory Bank Controller                   | 🟢  |
| *MBC 1*                                  | 🟢  |
| *MBC 2*                                  | 🟢  |
| *MBC 3*                                  | 🔴  |
| *MBC 5*                                  | 🟢  |
| Persistent Cartridge Memory              | 🟢  |
| Save/Load Snapshots of the entire system | 🔴  |
| Sound                                    | 🔴  |
| Serial Port / Multiplayer                |  ⚪  |
| GameBoy Color Support                    | 🔵  |


# Test Results

## Acid 2 Test

|               |     |
|---------------|:---:|
| DMG Acid 2    | 🟢  |
| CGB Acid 2    | 🟢  |
| CGB Acid Hell | 🔴  |

## Blargg Test ROMs

|                      |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  | 10  | 11  | 12  |
|----------------------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| cgb_sound            |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |
| cpu_instrs           | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  |     |
| dmg_sound            |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |
| instr_timing         | 🟢  |     |     |     |     |     |     |     |     |     |     |     |
| interrupt_time       | 🔴  |     |     |     |     |     |     |     |     |     |     |     |
| mem_timing           | 🟢  | 🟢  | 🟢  |     |     |     |     |     |     |     |     |     |
| mem_timing-2         | 🟢  | 🟢  | 🟢  |     |     |     |     |     |     |     |     |     |
| oam_bug              | 🔴  | 🔴  | 🟢  | 🔴  | 🔴  | 🟢  | 🔴  | 🔴  |     |     |     |     |
| halt_bug (dmg / cgb) | 🔴  |  ❓  |     |     |     |     |     |     |     |     |     |     |

## Gambatte Test ROMs

|                      |     |
|----------------------|:---:|
| bgen                 | 🔴  |
| bgtiledata           | 🔴  |
| bgtilemap            | 🔴  |
| cgbpal_m3            | 🟡  |
| display_startstate   | 🔴  |
| div                  | 🟡  |
| dma                  | 🟡  |
| dmgpalette_during_m3 | 🔴  |
| enable_display       | 🟡  |
| halt                 | 🟡  |
| irq_precedence       | 🔴  |
| lcd_offset           | 🔴  |
| lcdirq_precedence    | 🟡  |
| ly0                  | 🟡  |
| lyc0int_m0irq        | 🟡  |
| lyc153int_m2irq      | 🟡  |
| lycenable            | 🟡  |
| lycint_ly            | 🔴  |
| lycint_lycflag       | 🔴  |
| lycint_lycirq        | 🔴  |
| lycint_m0stat        | 🟡  |
| lycm2int             | 🟡  |
| lywrite              | 🔴  |
| m0enable             | 🟡  |
| m0int_m0irq          | 🟡  |
| m0int_m0stat         | 🔴  |
| m0int_m3stat         | 🟡  |
| m1                   | 🟡  |
| m2enable             | 🟡  |
| m2int_m0irq          | 🟡  |
| m2int_m0stat         | 🟡  |
| m2int_m2irq          | 🟡  |
| m2int_m2stat         | 🟡  |
| m2int_m3stat         | 🟡  |
| miscmstatirq         | 🟡  |
| oam_access           | 🟡  |
| oamdma               | 🟡  |
| scx_during_m3        | 🔴  |
| scy                  | 🔴  |
| serial               | 🔴  |
| sound                | 🔴  |
| speedchange          | 🔴  |
| sprites              | 🟡  |
| tima                 | 🟡  |
| undef_ops            | 🔴  |
| vram_m3              | 🟡  |
| vramw_m3end          | 🟡  |
| window               | 🟡  |

## Mooneye Test ROMs

|               |                |     |
|---------------|----------------|:---:|
| acceptance    | boot           | 🟡  |
|               | bits           | 🟡  |
|               | instr          | 🟢  |
|               | interrupts     | 🔴  |
|               | oam dma        | 🟡  |
|               | ppu            | 🔴  |
|               | serial         | 🔴  |
|               | timer          | 🟢  |
|               | opcode timings | 🔴  |
| emulator only | MBC 1          | 🟢  |
|               | MBC 2          | 🟢  |
|               | MBC 5          | 🟢  |
| misc          | bits           | 🔴  |
|               | boot           | 🔴  |
|               | ppu            | 🔴  |


## Symbols

|     |                                       |
|:---:|:--------------------------------------|
| 🟢  | completely implemented                |
| 🟡  | partially implemented / needs testing |
| 🔵  | currently work in progress            |
| 🔴  | not implemented                       |
|  ⚪  | currently not planned                 |
|  ❓  | Unknown / not tested                  |
