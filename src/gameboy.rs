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

use std::time;
use std::time::{Duration, SystemTime};
use crate::boot_rom::BootRom;
use crate::cartridge::{Cartridge, LicenseeCode};
use crate::cpu::{Cpu, CPU_CLOCK_SPEED, CpuFlag, RegisterR8};
use crate::GameBoyColorSupport;
use crate::input::Input;
use crate::memory::{Memory, MemoryRead};
use crate::opcode::OpCodeContext;
use crate::ppu::{FrameState, Ppu, SCREEN_H, SCREEN_W};
use crate::timer::Timer;
use crate::utils::carrying_add_u8;
use crate::window::Window;


/// Type to measure clock ticks of the device.
/// Alias for unsigned 64bit integer.
pub type clock_t = u64;


/// The type of GameBoy device to be emulated.
#[derive(Copy, Clone)]
pub enum DeviceType {
    /// The original GameBoy with monochrome 4 color display.
    /// DMG = Dot Matrix Game
    GameBoyDmg,

    /// GameBoy Color with slightly more RAM and color support.
    GameBoyColor,

    /// GameBoy Advance
    GameBoyAdvance,

    /// Super GameBoy
    SuperGameBoy,

    /// Super GameBoy 2
    SuperGameBoy2,
}


/// Depending on the device and ROM being emulated, the type of
/// emulation running. For example, the GameBoy Color hardware may
/// run in DMG compatibility mode when a ROM without GBC support
/// is played.
#[derive(Copy, Clone)]
pub enum EmulationType {
    /// Classic GameBoy or compatibility mode.
    DMG,

    /// GameBoy Color support enabled.
    GBC,
}


/// A struct containing the setup information of the running device.
#[derive(Copy, Clone)]
pub struct DeviceConfig {
    /// The current device type being running.
    pub device: DeviceType,

    /// The current emulation mode (DMG compatibility or Color support)
    pub emulation: EmulationType,

    /// Flag if opcodes should be printed
    pub print_opcodes: bool,
}


/// A factory class to construct a GameBoy device object.
/// Usually created via GameBoy::build()
pub struct Builder {
    boot_rom:      Option<BootRom>,
    cartridge:     Option<Cartridge>,
    device_type:   Option<DeviceType>,
    print_opcodes: bool,
}


/// The GameBoy object providing access to all it's emulated components.
pub struct GameBoy {
    device_config: DeviceConfig,

    pub cpu: Cpu,
    pub ppu: Ppu,
    pub mem: Memory,
    pub timer: Timer,
    pub input: Input,
    pub window: Window,
}


impl DeviceConfig {
    /// Checks whether the current device is running with GameBoyColor support enabled.
    /// The running device needs to be a GBC or GBA *and* running a cartridge
    /// with GameBoy Color support.
    pub fn is_gbc_enabled(&self) -> bool {
        match self.emulation {
            EmulationType::DMG => false,
            EmulationType::GBC => true,
        }
    }
}


impl Builder {
    /// Creates a new empty GameBoy builder
    pub fn new() -> Self {
        Self {
            boot_rom:      None,
            cartridge:     None,
            device_type:   None,
            print_opcodes: false,
        }
    }

    /// Set the boot ROM, which will be executed before the actual ROM.
    pub fn set_boot_rom(&mut self, boot_rom: BootRom) {
        self.boot_rom = Some(boot_rom);
    }

    /// Set the cartridge, which ROM will be executed.
    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
    }

    /// Override the preferred device type.
    /// If not specified, the device type will be determined by the cartridge type.
    pub fn set_device_type(&mut self, device_type: DeviceType) {
        self.device_type = Some(device_type);
    }

    /// Configures whether the emulator should print all opcodes being executed or not.
    pub fn set_print_opcodes(&mut self, print: bool) {
        self.print_opcodes = print;
    }

    /// Get the preferred device type, which is either specified explicitly
    /// or selected by the cartridge properties.
    pub fn select_preferred_device_type(&self) -> DeviceType {
        // explicit type will be preferred
        if let Some(device_type) = &self.device_type {
            return *device_type;
        }

        // determine the preferred device type by the cartridge properties
        if let Some(cartridge) = &self.cartridge {
            return match cartridge.get_cgb_support() {
                GameBoyColorSupport::None      => DeviceType::GameBoyDmg,
                GameBoyColorSupport::Supported => DeviceType::GameBoyColor,
                GameBoyColorSupport::Required  => DeviceType::GameBoyColor,
            };
        }

        // default to classic GameBoy
        DeviceType::GameBoyDmg
    }

    /// Build the GameBoy device emulator based on the properties specified with this builder.
    pub fn finish(mut self) -> Result<GameBoy, String> {
        // select the preferred device type based on the current config and cartridge
        let device_type = self.select_preferred_device_type();

        // setup device config based on the current configuration
        let device_config = DeviceConfig {
            device: device_type,

            emulation: match device_type {
                DeviceType::GameBoyDmg => EmulationType::DMG,
                _                      => EmulationType::GBC,
            },

            print_opcodes: self.print_opcodes
        };

        // construct the GameBoy object
        let mut gb = GameBoy::new(device_config)?;

        // set boot ROM, if any
        if let Some(boot_rom) = self.boot_rom.take() {
            gb.set_boot_rom(boot_rom);
        }

        // insert cartridge, if any
        if let Some(cartridge) = self.cartridge.take() {
            gb.insert_cart(cartridge);
        }

        Ok(gb)
    }
}


impl GameBoy {
    /// Creates a builder to build up the device.
    pub fn build() -> Builder {
        Builder::new()
    }

    /// Create a new GameBoy device.
    pub fn new(device_config: DeviceConfig) -> Result<GameBoy,String> {
        let mem = Memory::new(device_config);
        let window = Window::create("GameBoy")?;

        Ok(
            GameBoy {
                device_config,

                cpu: Cpu::new(mem.create_read_write_handle()),
                ppu: Ppu::new(device_config, mem.create_read_write_handle()),
                timer: Timer::new(mem.create_read_write_handle()),
                input: Input::new(mem.create_read_write_handle()),
                mem,
                window,
            }
        )
    }

    /// Assign a boot rom to be executed on startup.
    pub fn set_boot_rom(&mut self, boot_rom: BootRom) {
        self.mem.set_boot_rom(boot_rom);
    }

    /// Inserts a cartridge into the device and load ROM data into memory.
    pub fn insert_cart(&mut self, cartridge: Cartridge) {
        self.mem.set_cartridge(cartridge);
    }

    /// Get the configuration of the current GameBoy device.
    pub fn get_config(&self) -> &DeviceConfig {
        &self.device_config
    }

    /// Boot the device, initializing the Boot ROM program.
    pub fn initialize(&mut self) {
        if self.mem.has_boot_rom() {
            self.cpu.set_instruction_pointer(0x0000);
        }
        else {
            self.setup_initial_values();
        }
    }

    /// setup values like expected after the boot rom was executed on the original GameBoy.
    fn setup_initial_values(&mut self) {
        let pc = 0x0100;
        let sp = 0xfffe;

        // the title checksum is calculated on GBC and GBA in DMG compatibility mode
        // if licensee code is '1' in either old or new format
        let title_checksum = if let Some(cartridge) = self.mem.get_cartridge().as_ref() {
            match cartridge.get_licensee_code() {
                LicenseeCode::Old(1) | LicenseeCode::New(1) => {
                    cartridge.compute_title_checksum()
                }

                _ => 0x00
            }
        }
        else {
            // without cartridge, the checksum is 0xff
            0xff
        };

        // read cartridge header checksum
        let header_checksum = self.mem.read_u8(0x14d);

        // select initial values based on device type and emulation mode
        let (a, flag_z, flag_n, flag_h, flag_c, b, c, d, e, h, l) =
            match (self.device_config.device, self.device_config.emulation)
        {
            // classic GameBoy
            (DeviceType::GameBoyDmg, _) => {
                let ch = header_checksum != 0;
                (0x01, true, false, ch, ch, 0x00, 0x13, 0x00, 0xd8, 0x01, 0x4d)
            }

            // GameBoy Color with classic GameBoy cartridge
            (DeviceType::GameBoyColor, EmulationType::DMG) => {
                let b = title_checksum;

                let (h, l) = match b {
                    0x43 | 0x58 => (0x99, 0x1a), _ => (0x00, 0x7c)
                };

                (0x11, true, false, false, false, b, 0x00, 0x00, 0x08, h, l)
            }

            // GameBoy Color with a cartridge using the GBC extensions
            (DeviceType::GameBoyColor, EmulationType::GBC) => {
                (0x11, true, false, false, false, 0x00, 0x00, 0xff, 0x56, 0x00, 0x0d)
            }

            // GameBoy Advance with classic GameBoy cartridge
            (DeviceType::GameBoyAdvance, EmulationType::DMG) => {
                let (b, flag_h, _) = carrying_add_u8(title_checksum, 1, false);
                let flag_z = b == 0;

                let (h, l) = match b {
                    0x44 | 0x59 => (0x99, 0x1a), _ => (0x00, 0x7c)
                };

                (0x11, flag_z, false, flag_h, false, b, 0x00, 0x00, 0x08, h, l)
            }

            // GameBoy Advance with a cartridge using GBC extensions
            (DeviceType::GameBoyAdvance, EmulationType::GBC) => {
                (0x11, false, false, false, false, 0x01, 0x00, 0xff, 0x56, 0x00, 0x0d)
            }

            // SuperGameBoy with any cartridge?
            (DeviceType::SuperGameBoy, _) => {
                (0x01, false, false, false, false, 0x00, 0x14, 0x00, 0x00, 0xc0, 0x60)
            }

            // SuperGameBoy with any cartridge?
            (DeviceType::SuperGameBoy2, _) => {
                (0xff, false, false, false, false, 0x00, 0x14, 0x00, 0x00, 0xc0, 0x60)
            }
        };

        // build flags register
        let f = 0
            |   (if flag_z { 1 << CpuFlag::Zero.bit() }      else { 0 })
            |   (if flag_n { 1 << CpuFlag::Negative.bit() }  else { 0 })
            |   (if flag_h { 1 << CpuFlag::HalfCarry.bit() } else { 0 })
            |   (if flag_c { 1 << CpuFlag::Carry.bit() }     else { 0 })
        ;

        // update registers with the desired values
        self.cpu.set_r8(RegisterR8::A, a);
        self.cpu.set_r8(RegisterR8::F, f);
        self.cpu.set_r8(RegisterR8::B, b);
        self.cpu.set_r8(RegisterR8::C, c);
        self.cpu.set_r8(RegisterR8::D, d);
        self.cpu.set_r8(RegisterR8::E, e);
        self.cpu.set_r8(RegisterR8::H, h);
        self.cpu.set_r8(RegisterR8::L, l);
        self.cpu.set_instruction_pointer(pc);
        self.cpu.set_stack_pointer(sp);
    }

    /// Runs the program located on a cartridge, starting on the
    /// current location of the instruction pointer.
    pub fn run(&mut self) {
        let mut interval_begin  = time::Instant::now();
        let mut interval_cycles = 0;

        loop {
            let cycles = if self.cpu.is_running() {
                let instruction = self.cpu.fetch_next_instruction();
                let mut context = OpCodeContext::for_instruction(&instruction);

                (instruction.opcode.proc)(self, &mut context);

                if self.device_config.print_opcodes {
                    println!(
                        "/* {:04x} [{:02x}]{} */ {:<16}    ; {}",
                        instruction.opcode_address,
                        instruction.opcode_id,
                        if instruction.opcode_id <= 0xff { "  " } else { "" },
                        instruction.to_string(),
                        self.cpu
                    );
                }

                // take the number of cycles consumed by the last operation
                let cycles = context.get_cycles_consumed();

                cycles
            }
            else {
                // when in HALT state just pass 4 cycles
                // where the CPU idles
                4
            };

            // let other components handle their state
            self.mem.update(cycles);
            self.cpu.update(cycles);
            self.timer.update(cycles);
            self.input.update();

            // count the total cycles per interval
            interval_cycles += cycles;

            // let the PPU run for the same amount of cycles
            let ppu_state = self.ppu.update(cycles);

            // When a frame completed, it should be presented
            if let FrameState::FrameCompleted = ppu_state {
                self.window.poll_events();
                self.window.apply_key_states(&mut self.input);
                self.window.present(self.ppu.get_lcd(), &self.ppu);

                // handle frame times
                {
                    let frame_end_time = time::Instant::now();

                    let interval_duration_ns = (frame_end_time - interval_begin).as_nanos() as u64;
                    let expected_time_ns     = (1_000_000_000u64 * interval_cycles) / CPU_CLOCK_SPEED;

                    // when the interval time was shorter than expected,
                    // let the CPU sleep for the time difference
                    if interval_duration_ns < expected_time_ns {
                        let time_remaining = expected_time_ns - interval_duration_ns;
                        std::thread::sleep(Duration::from_nanos(time_remaining));
                    }

                    // reset the interval after counting more than the number of cycles per second,
                    // so we get a bit more precision than just counting per frame
                    if interval_cycles >= CPU_CLOCK_SPEED {
                        interval_cycles -= CPU_CLOCK_SPEED;
                        interval_begin = frame_end_time;
                    }
                }

                if !self.window.is_opened() {
                    // before exit, save the RAM image, if any
                    if let Err(e) = self.mem.save_cartridge_ram_if_any() {
                        println!("Failed to save cartridge RAM: {}", e);
                    }

                    return;
                }
            }
        }
    }
}
