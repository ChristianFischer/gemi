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

use std::cell::{Ref, RefCell, RefMut};
use std::cmp::{max, min};
use std::io;
use std::rc::Rc;
use crate::boot_rom::BootRom;
use crate::cartridge::Cartridge;
use crate::cpu::Interrupt;
use crate::memory_data::{MemoryData, MemoryDataFixedSize};
use crate::gameboy::{Clock, DeviceConfig, EmulationType};
use crate::graphic_data::GbcPaletteData;
use crate::io_registers::IoRegister;
use crate::mbc::{create_mbc, Mbc};
use crate::mbc::mbc_none::MbcNone;
use crate::memory_data::mapped::MemoryDataMapped;
use crate::utils::{clear_bit, get_bit, set_bit, to_u16, to_u8};

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
pub const MEMORY_LOCATION_APU_NR21:                 u16 = 0xff16;
pub const MEMORY_LOCATION_APU_NR22:                 u16 = 0xff17;
pub const MEMORY_LOCATION_APU_NR23:                 u16 = 0xff18;
pub const MEMORY_LOCATION_APU_NR24:                 u16 = 0xff19;
pub const MEMORY_LOCATION_APU_NR30:                 u16 = 0xff1a;
pub const MEMORY_LOCATION_APU_NR31:                 u16 = 0xff1b;
pub const MEMORY_LOCATION_APU_NR32:                 u16 = 0xff1c;
pub const MEMORY_LOCATION_APU_NR33:                 u16 = 0xff1d;
pub const MEMORY_LOCATION_APU_NR34:                 u16 = 0xff1e;
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


/// Helper macro to map memory addresses into their distinct areas.
macro_rules! memory_map {
    ($addr:expr => { $($from:literal $(..= $to:literal)? => [$($param:ident)?] $code:expr),+ }) => {
        match $addr {
            $(
                $from $(..= $to)? => {
                    $(let $param: usize = ($addr as usize) - ($from as usize);)?
                    $code
                }
            )+
        }
    }
}



/// Stores the information of an active OAM DMA transfer
/// The DMA transfer copies data from the given address
/// into OAM memory.
/// In total, 160 bytes will be transferred, so it takes
/// 160 cycles to transfer for the transfer to be completed.
pub struct DmaTransferInfo {
    /// The address where to start copying the memory from.
    start_address: u16,

    /// The next byte to be copied.
    next_byte: u16,
}


/// State of the OAM DMA transfer, whether it be disabled or
/// in progress, including the time remaining.
pub enum DmaTransferState {
    /// No transfer is active.
    Disabled,

    /// A transfer is currently in progress.
    /// The attached struct stores information where
    /// the data should be taken from and how much data
    /// was already transferred.
    Transferring(DmaTransferInfo),
}


/// The memory object as the main owner of the emulator's memory.
/// This object can be used to create additional handles which
/// allow read and write access to the device memory.
pub struct Memory {
    internal: MemoryInternalRef,
}

/// A memory handle to provide readonly access to the device memory.
/// This object can be created from an existing Memory object.
#[derive(Clone)]
pub struct MemoryReadOnlyHandle {
    internal: MemoryInternalRef,
}

/// A memory handle to provide read/write access to the device memory.
/// This object can be created from an existing Memory object.
#[derive(Clone)]
pub struct MemoryReadWriteHandle {
    internal: MemoryInternalRef,
}


pub type VRamBank       = MemoryDataFixedSize<8192>;
pub type WRamBank       = MemoryDataFixedSize<4096>;
pub type OamRamBank     = MemoryDataFixedSize<160>;
pub type HRamBank       = MemoryDataFixedSize<127>;
pub type GbcPaletteBank = MemoryDataMapped<[GbcPaletteData; 8]>;
pub type IoRegisterBank = MemoryDataMapped<IoRegister>;


/// Shared internal object for multiple Memory and MemoryReadWrite instances.
struct MemoryInternal {
    /// The configuration of the running device
    device_config: DeviceConfig,

    /// Video RAM (DMG = 1 * 8kiB, GBC = 2 * 8kiB)
    vram_banks: Vec<VRamBank>,

    /// Active Video RAM Bank (0-1, CGB only)
    vram_active_bank: u8,

    /// Work RAM banks (DMG = 2 * 4kiB, GBC = 8 * 4kiB)
    wram_banks: Vec<WRamBank>,

    /// Active Work RAM banks.
    /// Bank 0 is fixed, Bank 1 can be switched between 1-7 on GBC.
    wram_active_bank_0: u8,
    wram_active_bank_1: u8,

    /// OAM memory: 40 sprites, 4 bytes each = 160B
    oam: OamRamBank,

    /// IO Registers
    io_registers: IoRegisterBank,

    /// Holds a bit for each IO register, which is set once
    /// the according register was written to.
    io_registers_written: [bool; 256],

    /// High RAM
    hram: HRamBank,

    /// GameBoy Color only: storage for background palettes
    gbc_background_palette: GbcPaletteBank,

    /// GameBoy Color only: storage for object palettes
    gbc_object_palette: GbcPaletteBank,

    mbc:        Box<dyn Mbc>,
    dma:        DmaTransferState,
    boot_rom:   Option<BootRom>,
    cartridge:  Option<Cartridge>,
}

/// Helper object to store a shared reference to the internal memory object.
/// The shared reference can be owned by multiple memory objects and therefore
/// granting read or write access to the device memory for multiple components.
#[derive(Clone)]
struct MemoryInternalRef {
    r: Rc<RefCell<MemoryInternal>>,
}

/// A trait for memory objects allowing read access to the memory data.
pub trait MemoryRead {
    /// Read a single byte from the device memory.
    fn read_byte(&self, address: u16) -> u8;

    /// Read an u8 value from the given address in the device memory.
    fn read_u8(&self, address: u16) -> u8 {
        self.read_byte(address)
    }

    /// Read an i8 value from the given address in the device memory.
    fn read_i8(&self, address: u16) -> i8 {
        self.read_byte(address) as i8
    }

    /// Read an u16 value from the given address in the device memory.
    fn read_u16(&self, address: u16) -> u16 {
        let l = self.read_byte(address);
        let h = self.read_byte(address + 1);
        to_u16(h, l)
    }

    /// Read an i16 value from the given address in the device memory.
    fn read_i16(&self, address: u16) -> i16 {
        self.read_u16(address) as i16
    }

    /// Get the n'th bit of a byte on the given address.
    fn get_bit(&self, address: u16, bit: u8) -> bool {
        let byte = self.read_u8(address);
        get_bit(byte, bit)
    }
}

/// A trait for memory objects allowing write access to the memory data.
pub trait MemoryWrite : MemoryRead {
    /// Write a single byte into the device memory.
    fn write_byte(&mut self, address: u16, value: u8);

    /// Writes an u8 value into the given address in the device memory.
    fn write_u8(&mut self, address: u16, value: u8) {
        self.write_byte(address, value);
    }

    /// Writes an i8 value into the given address in the device memory.
    fn write_i8(&mut self, address: u16, value: i8) {
        self.write_byte(address, value as u8);
    }

    /// Writes an u16 value into the given address in the device memory.
    fn write_u16(&mut self, address: u16, value: u16) {
        let (h, l) = to_u8(value);
        self.write_byte(address + 0, l);
        self.write_byte(address + 1, h);
    }

    /// Writes an i16 value into the given address in the device memory.
    fn write_i16(&mut self, address: u16, value: i16) {
        self.write_u16(address, value as u16);
    }

    /// Set the n'th bit of a byte on the given address.
    fn change_bit(&mut self, address: u16, bit: u8, value: bool) {
        if value {
            self.set_bit(address, bit);
        }
        else {
            self.clear_bit(address, bit);
        }
    }

    /// Set the n'th bit of a byte on the given address to 0.
    fn clear_bit(&mut self, address: u16, bit: u8) {
        let byte = self.read_u8(address);
        let result = clear_bit(byte, bit);
        self.write_byte(address, result);
    }

    /// Set the n'th bit of a byte on the given address to 1.
    fn set_bit(&mut self, address: u16, bit: u8) {
        let byte = self.read_u8(address);
        let result = set_bit(byte, bit);
        self.write_byte(address, result);
    }
}


impl Memory {
    /// Create a new Memory object.
    pub fn new(device_config: DeviceConfig) -> Self {
        let (num_vram_banks, num_wram_banks) = match device_config.emulation {
            EmulationType::DMG => (1, 2),
            EmulationType::GBC => (2, 8),
        };

        Self {
            internal: MemoryInternalRef::new(
                MemoryInternal {
                    device_config,

                    vram_banks: std::iter::repeat_with(|| VRamBank::new()).take(num_vram_banks).collect(),
                    vram_active_bank: 0,

                    wram_banks: std::iter::repeat_with(|| WRamBank::new()).take(num_wram_banks).collect(),
                    wram_active_bank_0: 0,
                    wram_active_bank_1: 1,

                    oam: OamRamBank::new(),
                    hram: HRamBank::new(),
                    io_registers: IoRegisterBank::new(IoRegister::default()),
                    io_registers_written: [false; 256],

                    gbc_background_palette: GbcPaletteBank::new([GbcPaletteData::new(); 8]),
                    gbc_object_palette: GbcPaletteBank::new([GbcPaletteData::new(); 8]),

                    mbc:        Box::new(MbcNone::new()),
                    dma:        DmaTransferState::Disabled,
                    boot_rom:   None,
                    cartridge:  None,
                }
            )
        }
    }


    /// Initializes the values of all IO registers.
    pub fn initialize_io_registers(&mut self, initial_values: [u8; 256]) {
        let mut mem = self.internal.get_mut();
        let io_regs = &mut mem.io_registers;

        for i in 0..256 {
            io_regs.set_at(i, initial_values[i]);
        }
    }


    /// Let the memory controller handle it's tasks.
    /// 'cycles' gives the number of ticks passed since
    /// the last call.
    pub fn update(&mut self, cycles: Clock) {
        self.internal.get_mut().handle_dma_transfer(cycles);
    }


    /// Creates a MemoryReadOnlyHandle from this Memory object.
    /// This will be used to provide read/write access to the device memory.
    pub fn create_readonly_handle(&self) -> MemoryReadOnlyHandle {
        MemoryReadOnlyHandle {
            internal: MemoryInternalRef::clone(&self.internal)
        }
    }

    /// Creates a MemoryReadWriteHandle from this Memory object.
    /// This will be used to provide read/write access to the device memory.
    pub fn create_read_write_handle(&self) -> MemoryReadWriteHandle {
        MemoryReadWriteHandle {
            internal: MemoryInternalRef::clone(&self.internal)
        }
    }

    /// Checks whether a boot rom is active or not.
    pub fn has_boot_rom(&self) -> bool {
        match self.internal.get().boot_rom {
            None    => false,
            Some(_) => true,
        }
    }

    /// Load a boot ROM into the memory.
    pub fn set_boot_rom(&mut self, boot_rom: BootRom) {
        let mut mem = self.internal.get_mut();
        mem.boot_rom = Some(boot_rom)
    }

    /// Load ROM data from a cartridge into the memory.
    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        let mut mem = self.internal.get_mut();
        mem.mbc       = create_mbc(cartridge.get_mbc());
        mem.cartridge = Some(cartridge);
    }

    /// Get a reference to the currently assigned cartridge, if any.
    pub fn get_cartridge(&self) -> Ref<Option<Cartridge>> {
        let mem = self.internal.get();
        Ref::map(mem, |mem| &mem.cartridge)
    }

    /// Save the cartridge RAM, if any.
    pub fn save_cartridge_ram_if_any(&self) -> io::Result<()> {
        let mem = self.internal.get();

        if let Some(cartridge) = &mem.cartridge {
            cartridge.save_ram_if_any()?;
        }

        Ok(())
    }
}

impl MemoryRead for Memory {
    fn read_byte(&self, address: u16) -> u8 {
        self.internal.get().read(address)
    }
}

impl MemoryWrite for Memory {
    fn write_byte(&mut self, address: u16, value: u8) {
        self.internal.get_mut().write(address, value);
    }
}

impl MemoryRead for MemoryReadOnlyHandle {
    fn read_byte(&self, address: u16) -> u8 {
        self.internal.get().read(address)
    }
}

impl MemoryRead for MemoryReadWriteHandle {
    fn read_byte(&self, address: u16) -> u8 {
        self.internal.get().read(address)
    }
}

impl MemoryWrite for MemoryReadWriteHandle {
    fn write_byte(&mut self, address: u16, value: u8) {
        self.internal.get_mut().write(address, value);
    }
}

impl MemoryReadWriteHandle {
    pub fn clone_readonly(&self) -> MemoryReadOnlyHandle {
        MemoryReadOnlyHandle {
            internal: self.internal.clone()
        }
    }

    /// Get the list of Work RAM banks on this device.
    /// DMG = 2 banks, GBC = 8 banks.
    pub fn get_wram_banks(&self) -> Ref<Vec<WRamBank>> {
        let mem = self.internal.get();
        Ref::map(mem, |mem| &mem.wram_banks)
    }

    /// Get the list of Video RAM banks on this device.
    /// DMG = 1 bank, GBC = 2 banks.
    pub fn get_vram_banks(&self) -> Ref<Vec<VRamBank>> {
        let mem = self.internal.get();
        Ref::map(mem, |mem| &mem.vram_banks)
    }

    /// Get the color palette used by background tiles on GameBoy Color.
    pub fn get_gbc_background_palettes(&self) -> Ref<[GbcPaletteData; 8]> {
        let mem = self.internal.get();
        Ref::map(mem, |mem| mem.gbc_background_palette.get())
    }

    /// Get the color palette used by background tiles on GameBoy Color.
    pub fn get_gbc_object_palettes(&self) -> Ref<[GbcPaletteData; 8]> {
        let mem = self.internal.get();
        Ref::map(mem, |mem| mem.gbc_object_palette.get())
    }

    /// Get the IO Registers struct.
    pub fn get_io_registers(&self) -> Ref<IoRegister> {
        let mem = self.internal.get();
        Ref::map(mem, |mem| mem.io_registers.get())
    }

    /// Get the IO Registers struct as a mutable reference.
    pub fn get_io_registers_mut(&mut self) -> RefMut<IoRegister> {
        let mem = self.internal.get_mut();
        RefMut::map(mem, |mem| mem.io_registers.get_mut())
    }

    /// Checks whether a specific IO register was written to. The flag will be kept until
    /// acknowledged by calling ```acknowledge_io_register_written```.
    pub fn was_io_register_written(&self, address: u16) -> bool {
        assert!(address >= 0xff00, "Address is not an IO register");
        let mem = self.internal.get();
        mem.io_registers_written[(address & 0xff) as usize]
    }

    /// Acknowledges the 'written to' flag for a specific IO register. This will set the
    /// flag to false until being written again.
    pub fn acknowledge_io_register_written(&mut self, address: u16) {
        assert!(address >= 0xff00, "Address is not an IO register");
        let mut mem = self.internal.get_mut();
        mem.io_registers_written[(address & 0xff) as usize] = false;
    }

    /// A convenience function which combines the functionality of ```was_io_register_written```,
    /// ```acknowledge_io_register_written``` and reading the value of the given register.
    /// If the requested IO register was changed, it acknowledges the writing operation
    /// and returns it's new value. Otherwise, it will return ```None```.
    pub fn take_changed_io_register(&mut self, address: u16) -> Option<u8> {
        assert!(address >= 0xff00, "Address is not an IO register");
        let mut mem = self.internal.get_mut();
        let index = (address & 0xff) as usize;

        if mem.io_registers_written[index] {
            mem.io_registers_written[index] = false;

            Some(mem.io_registers.get_at(index))
        }
        else {
            None
        }
    }

    /// requests an interrupt to be fired.
    /// This will set the according bit in the memory. If Interrupts
    /// are enabled for the CPU, the instruction pointer will jump
    /// to the according interrupt address, otherwise it will be ignored.
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.set_bit(MEMORY_LOCATION_INTERRUPTS_FLAGGED, interrupt.bit());
    }
}

impl MemoryInternalRef {
    /// Creates a new shared reference to a MemoryInternal object.
    /// The given MemoryInternal object will be moved inside the newly created shared reference.
    pub fn new(internal: MemoryInternal) -> MemoryInternalRef {
        MemoryInternalRef {
            r: Rc::new(RefCell::new(internal))
        }
    }

    /// Creates a clone of a reference to a MemoryInternal object.
    /// This just clones the reference, not the MemoryInternal object itself.
    pub fn clone(other: &MemoryInternalRef) -> MemoryInternalRef {
        MemoryInternalRef {
            r: Rc::clone(&other.r)
        }
    }

    /// Get a readonly reference to the MemoryInternal object.
    pub fn get(&self) -> Ref<MemoryInternal> {
        let ref_cell : &RefCell<MemoryInternal> = &(self.r);
        ref_cell.borrow()
    }

    /// Get a mutable reference to the MemoryInternal object.
    pub fn get_mut(&mut self) -> RefMut<MemoryInternal> {
        let ref_cell : &RefCell<MemoryInternal> = &(self.r);
        ref_cell.borrow_mut()
    }
}

impl MemoryInternal {
    /// Reads data from any memory location.
    /// The request will be forwarded to the according device, depending
    /// on the physical location of the data (like cartridge, ppu, etc)
    pub fn read(&self, address: u16) -> u8 {
        memory_map!(
            address => {
                0x0000 ..= 0x00ff => [] self.read_boot_rom_or_cartridge(address),
                0x0100 ..= 0x7fff => [] self.read_from_cartridge(address),

                0x8000 ..= 0x9fff => [mapped_address] {
                    let bank = &self.vram_banks[self.vram_active_bank as usize];
                    bank.get_at(mapped_address)
                },

                0xa000 ..= 0xbfff => [] self.read_from_cartridge(address),

                0xc000 ..= 0xcfff => [mapped_address] {
                    let bank = &self.wram_banks[self.wram_active_bank_0 as usize];
                    bank.get_at(mapped_address)
                },

                0xd000 ..= 0xdfff => [mapped_address] {
                    let bank = &self.wram_banks[self.wram_active_bank_1 as usize];
                    bank.get_at(mapped_address)
                },

                0xe000 ..= 0xfdff => [mapped_address] {
                    // echo RAM; mapped into WRAM (0xc000 - 0xddff)
                    self.read((mapped_address + 0xc000) as u16)
                },

                0xfe00 ..= 0xfe9f => [mapped_address] self.oam.get_at(mapped_address),
                0xfea0 ..= 0xfeff => []               unreachable!(), // unusable ram area
                0xff00 ..= 0xff7f => [mapped_address] self.io_registers.get_at(mapped_address),
                0xff80 ..= 0xfffe => [mapped_address] self.hram.get_at(mapped_address),
                0xffff            => []               self.io_registers.get().interrupts_enabled
            }
        )
    }

    /// Reads data from the boot rom, if any, otherwise from the cartridge.
    fn read_boot_rom_or_cartridge(&self, address: u16) -> u8 {
        if let Some(boot_rom) = &self.boot_rom {
            return boot_rom.read(address);
        }

        self.read_from_cartridge(address)
    }

    /// Reads data from the cartridge.
    fn read_from_cartridge(&self, address: u16) -> u8 {
        if let Some(cartridge) = &self.cartridge {
            return self.mbc.read_byte(cartridge, address);
        }

        0xff
    }

    /// Writes data to any memory location.
    /// The request will be forwarded to the according device, depending
    /// on the physical location of the data (like cartridge, ppu, etc)
    pub fn write(&mut self, address: u16, value: u8) {
        memory_map!(
            address => {
                0x0000 ..= 0x7fff => [] self.write_to_cartridge(address, value),

                0x8000 ..= 0x9fff => [mapped_address] {
                    let bank = &mut self.vram_banks[self.vram_active_bank as usize];
                    bank.set_at(mapped_address, value)
                },

                0xa000 ..= 0xbfff => [] self.write_to_cartridge(address, value),

                0xc000 ..= 0xcfff => [mapped_address] {
                    let bank = &mut self.wram_banks[self.wram_active_bank_0 as usize];
                    bank.set_at(mapped_address, value)
                },

                0xd000 ..= 0xdfff => [mapped_address] {
                    let bank = &mut self.wram_banks[self.wram_active_bank_1 as usize];
                    bank.set_at(mapped_address, value)
                },

                0xe000 ..= 0xfdff => [mapped_address] {
                    // echo RAM; mapped into WRAM (0xc000 - 0xddff)
                    self.write((mapped_address + 0xc000) as u16, value)
                },

                0xfe00 ..= 0xfe9f => [mapped_address] self.oam.set_at(mapped_address, value),
                0xfea0 ..= 0xfeff => []               unreachable!(), // unusable ram area

                0xff00 ..= 0xff7f => [mapped_address] {
                    let old = self.io_registers.get_at(mapped_address);
                    self.io_registers.set_at(mapped_address, value);
                    self.io_registers_written[mapped_address] = true;
                    self.on_io_registers_changed(address, old, value);
                },

                0xff80 ..= 0xfffe => [mapped_address] self.hram.set_at(mapped_address, value),

                0xffff => [] {
                    let ioreg = self.io_registers.get_mut();
                    let old = ioreg.interrupts_enabled;
                    ioreg.interrupts_enabled = value;

                    self.io_registers_written[0xff] = true;

                    self.on_io_registers_changed(address, old, value);
                }
            }
        )
    }

    /// Writes data to the cartridge.
    fn write_to_cartridge(&mut self, address: u16, value: u8) {
        if let Some(cartridge) = &mut self.cartridge {
            self.mbc.write_byte(cartridge, address, value);
        }
    }

    /// Writes data into IO registers
    fn on_io_registers_changed(&mut self, address: u16, _old_value: u8, value: u8) {
        match address {
            MEMORY_LOCATION_DMA_ADDRESS => {
                let start_address = (value as u16) << 8;

                self.dma = DmaTransferState::Transferring(DmaTransferInfo {
                    start_address,
                    next_byte: 0,
                });
            },

            MEMORY_LOCATION_VBK => {
                // on GBC: switch VRAM bank
                if let EmulationType::GBC = self.device_config.emulation {
                    let bank = value & 0x01;
                    self.vram_active_bank = bank;

                    // register will contain the active bank in bit #0
                    // and all other bits set to 1
                    self.io_registers.get_mut().vbk = 0b_1111_1110 | bank;
                }
            },

            MEMORY_LOCATION_BOOT_ROM_DISABLE => {
                if (value & 0x01) != 0 {
                    self.boot_rom = None;
                }
            },

            MEMORY_LOCATION_BCPS => {
                // on writing background palette index load the according value into bcpd
                let palette_address = (value & 0x3f) as usize;
                self.io_registers.get_mut().bcpd = self.gbc_background_palette.get_at(palette_address);
            },

            MEMORY_LOCATION_BCPD => {
                let io_reg  = self.io_registers.get_mut();

                // get the address from BCPS
                let palette_address = (io_reg.bcps & 0x3f) as usize;

                // write palette data value
                self.gbc_background_palette.set_at(palette_address, value);

                // increment address, if auto increment is enabled
                if get_bit(io_reg.bcps, 7) {
                    io_reg.bcps = (((palette_address + 1) & 0x3f) | 0x80) as u8;
                }
            },

            MEMORY_LOCATION_OCPS => {
                // on writing object palette index load the according value into ocpd
                let palette_address = (value & 0x07) as usize;
                self.io_registers.get_mut().ocpd = self.gbc_object_palette.get_at(palette_address);
            },

            MEMORY_LOCATION_OCPD => {
                let io_reg  = self.io_registers.get_mut();

                // get the address from OCPS
                let palette_address = (io_reg.ocps & 0x3f) as usize;

                // write palette data value
                self.gbc_object_palette.set_at(palette_address, value);

                // increment address, if auto increment is enabled
                if get_bit(io_reg.ocps, 7) {
                    io_reg.ocps = (((palette_address + 1) & 0x3f) | 0x80) as u8;
                }
            },

            MEMORY_LOCATION_SVBK => {
                // on GBC: switch WRAM bank #1
                if let EmulationType::GBC = self.device_config.emulation {
                    let bank = value & 0x07;
                    self.wram_active_bank_1 = max(1, bank);
                }
            },

            _ => { }
        }
    }

    /// Handles an OAM DMA transfer, if any active.
    fn handle_dma_transfer(&mut self, cycles: Clock) {
        match self.dma {
            DmaTransferState::Disabled => {}

            DmaTransferState::Transferring(ref transfer) => {
                let oam_size       = MEMORY_LOCATION_OAM_END - MEMORY_LOCATION_OAM_BEGIN + 1;
                let transfer_begin = transfer.next_byte;
                let transfer_end   = min(transfer_begin.saturating_add(cycles as u16), oam_size);

                // Copy the amount of data the memory controller was able to handle
                for b in transfer_begin .. transfer_end {
                    let src = transfer.start_address.saturating_add(b);
                    let dst = b as usize;

                    self.oam.set_at(dst, self.read(src));
                }

                // store the current state or set the transfer state to 'Disabled'
                if transfer_end == oam_size {
                    self.dma = DmaTransferState::Disabled;
                }
                else {
                    self.dma = DmaTransferState::Transferring(DmaTransferInfo {
                        start_address: transfer.start_address,
                        next_byte:     transfer_end,
                    });
                }
            }
        }
    }
}
