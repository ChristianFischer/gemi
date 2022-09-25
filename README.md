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
| *MBC 2*                                  | 🔴  |
| *MBC 3*                                  | 🔴  |
| *MBC 5*                                  | 🟢  |
| Persistent Cartridge Memory              | 🟢  |
| Save/Load Snapshots of the entire system | 🔴  |
| Sound                                    | 🔴  |
| Serial Port / Multiplayer                |  ⚪  |
| GameBoy Color Support                    | 🔴  |


# Test Results

## Acid 2 Test

|                     |     |
|---------------------|:---:|
| DMG Acid 2          | 🟢  |
| DMG Acid 2 (on cgb) |  ❓  |
| CGB Acid 2          |  ❓  |

## Blargg Test ROMs

|                      |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  | 10  | 11  | 12  |
|----------------------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| cgb_sound            |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |
| cpu_instrs           | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  | 🟢  |     |
| dmg_sound            |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |  ❓  |
| instr_timing         | 🔴  |     |     |     |     |     |     |     |     |     |     |     |
| interrupt_time       | 🔴  |     |     |     |     |     |     |     |     |     |     |     |
| mem_timing           | 🔴  | 🔴  | 🔴  |     |     |     |     |     |     |     |     |     |
| mem_timing-2         | 🔴  | 🔴  | 🔴  |     |     |     |     |     |     |     |     |     |
| oam_bug              | 🔴  | 🔴  | 🟢  | 🔴  | 🔴  | 🟢  | 🔴  | 🔴  |     |     |     |     |
| halt_bug (dmg / cgb) | 🔴  |  ❓  |     |     |     |     |     |     |     |     |     |     |

## Symbols

|     |                                       |
|:---:|:--------------------------------------|
| 🟢  | completely implemented                |
| 🟡  | partially implemented / needs testing |
| 🔵  | currently work in progress            |
| 🔴  | not implemented                       |
|  ⚪  | currently not planned                 |
|  ❓  | Unknown / not tested                  |
