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

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Add;
use crate::memory_data::{MemoryData, MemoryDataDynamic};
use crate::mbc::MemoryBankController;


pub const FILE_EXT_GB:  &str = ".gb";
pub const FILE_EXT_GBC: &str = ".gbc";
pub const FILE_EXT_RAM: &str = ".sav";



/// Type of game boy color support
pub enum GameBoyColorSupport {
    /// CGB is not supported
    None,

    /// the cartridge supports CGB but runs on classic GB as well
    Supported,

    /// CGB is required to run this cartridge
    Required
}


/// Hold the licensee code (either it's old or new version)
pub enum LicenseeCode {
    /// old licensee code
    Old(u8),

    /// new licensee code
    New(u8),
}


/// This object holds the plain data of a ROM.
pub struct RomData {
    data: Vec<u8>,
}


/// This object represents a cartridge of a single game.
pub struct Cartridge {
    file: String,

    title: String,
    rom: RomData,
    ram: MemoryDataDynamic,

    manufacturer_code: String,
    licensee_code: LicenseeCode,

    mbc: MemoryBankController,

    rom_size: usize,
    ram_size: usize,

    supports_cgb: GameBoyColorSupport,
    supports_sgb: bool,

    has_ram: bool,
    has_timer: bool,
    has_battery: bool,
    has_rumble: bool,
}


const ROM_OFFSET_ENTRY_POINT:           usize = 0x0100;
const ROM_OFFSET_LOGO_BITMAP:           usize = 0x0104;
const ROM_OFFSET_TITLE_STRING:          usize = 0x0134;
const ROM_OFFSET_MANUFACTURER_CODE:     usize = 0x013F;
const ROM_OFFSET_FLAG_CGB:              usize = 0x0143;
const ROM_OFFSET_NEW_LICENSEE_CODE:     usize = 0x0144;
const ROM_OFFSET_FLAG_SGB:              usize = 0x0146;
const ROM_OFFSET_ROM_TYPE:              usize = 0x0147;
const ROM_OFFSET_ROM_SIZE:              usize = 0x0148;
const ROM_OFFSET_RAM_SIZE:              usize = 0x0149;
const ROM_OFFSET_DESTINATION_CODE:      usize = 0x014A;
const ROM_OFFSET_OLD_LICENSEE_CODE:     usize = 0x014B;



impl RomData {
    /// Get the ROM data on a particular address.
    pub fn get_at(&self, address: usize) -> u8 {
        self.data[address]
    }

    /// Read the game title from the ROM data.
    pub fn read_title(self: &RomData) -> String {
        let mut title_length: usize = 0;

        while title_length < 15 && self.data[ROM_OFFSET_TITLE_STRING + title_length] != 0 {
            title_length += 1;
        }

        let title_start = ROM_OFFSET_TITLE_STRING;
        let title_end   = ROM_OFFSET_TITLE_STRING + title_length;
        let title_chars  = &self.data[title_start..title_end];

        match std::str::from_utf8(&title_chars) {
            Ok(v) => v.to_string(),
            Err(_) => String::new(),
        }
    }


    /// Read the manufacturer code from the ROM data.
    pub fn read_manufacturer_code(self: &RomData) -> String {
        if self.data[ROM_OFFSET_MANUFACTURER_CODE - 1] == 0
            && self.data[ROM_OFFSET_MANUFACTURER_CODE] != 0
        {
            let mfc = &self.data[ROM_OFFSET_MANUFACTURER_CODE..ROM_OFFSET_MANUFACTURER_CODE + 4];

            return match std::str::from_utf8(mfc) {
                Ok(v) => v.to_string(),
                Err(_) => String::new(),
            };
        }

        return String::new();
    }
}


/// Change the file extension of a filename.
pub fn change_filename_ext(filepath: &str, ext: &str) -> String {
    {
        if filepath.ends_with(FILE_EXT_GB) {
            &filepath[0 .. filepath.len() - 3]
        }
        else if filepath.ends_with(FILE_EXT_GBC) {
            &filepath[0 .. filepath.len() - 4]
        }
        else {
            filepath
        }
    }.to_string().add(ext)
}


impl Cartridge {
    /// Load a cartridge from a file.
    /// * `filepath` - relative path to the file to be loaded
    pub fn load_file(filepath: &str) -> Result<Cartridge, io::Error> {
        let mut file = File::open(filepath)?;
        let metadata = file.metadata()?;
        let mut buffer = vec![0; metadata.len() as usize];

        file.read(&mut buffer)?;

        let rom = RomData {
            data: buffer
        };

        let cgb_flag_value = rom.data[ROM_OFFSET_FLAG_CGB];
        let supports_cgb = match cgb_flag_value {
            0x80 => GameBoyColorSupport::Supported,
            0xC0 => GameBoyColorSupport::Required,
            _    => GameBoyColorSupport::None,
        };

        let sgb_flag_value = rom.data[ROM_OFFSET_FLAG_SGB];
        let supports_sgb = sgb_flag_value == 0x03;

        let rom_size_type = rom.data[ROM_OFFSET_ROM_SIZE];
        let rom_size = (32 * 1024) << rom_size_type;

        let ram_size_type = rom.data[ROM_OFFSET_RAM_SIZE];
        let ram_size = match ram_size_type {
            0x00 =>   0,
            0x01 =>   2 * 1024,
            0x02 =>   8 * 1024,
            0x03 =>  32 * 1024,
            0x04 => 128 * 1024,
            0x05 =>  64 * 1024,
            _    =>   0,
        };

        let rom_type = rom.data[ROM_OFFSET_ROM_TYPE];

        let mbc = match rom_type {
            0x01..=0x03 => MemoryBankController::MBC1,
            0x05..=0x06 => MemoryBankController::MBC2,
            0x0F..=0x13 => MemoryBankController::MBC3,
            0x19..=0x1E => MemoryBankController::MBC5,
            0x20        => MemoryBankController::MBC6,
            0x22        => MemoryBankController::MBC7,
            _           => MemoryBankController::None,
        };

        let has_ram = match rom_type {
            0x02 | 0x03 | 0x08 | 0x09 | 0x0C | 0x0D => true,
            0x10 | 0x12 | 0x13 | 0x1A | 0x1B | 0x1D | 0x1E => true,
            0x22 | 0xFF => true,
            _ => false,
        };

        let has_timer = match rom_type {
            0x0F | 0x10 => true,
            _ => false,
        };

        let has_battery = match rom_type {
            0x03 | 0x06 | 0x09 | 0x0D | 0x0F => true,
            0x10 | 0x13 | 0x1B | 0x1E | 0x22 | 0xFF => true,
            _ => false,
        };

        let has_rumble = match rom_type {
            0x1C | 0x1D | 0x1E | 0x22 => true,
            _ => false,
        };

        // allocate RAM banks for this cartridge
        let mut ram = MemoryDataDynamic::alloc(ram_size);

        // if RAM is available and powered by a battery, it's persistent
        // and we can try to load the RAM image from a file.
        if has_ram && has_battery {
            let ram_file = change_filename_ext(filepath, FILE_EXT_RAM);

            if let Err(e) = ram.read_from_file(&ram_file) {
                // don't fail when RAM could not be loaded, just print a message
                println!("Failed to load Cartridge RAM: {}", e);
            }
        }

        let licensee_code_old = rom.data[ROM_OFFSET_OLD_LICENSEE_CODE];

        // get the new licensee code, which is only valid if the old code is '0x33'
        let licensee_code = if licensee_code_old != 0x33 {
            LicenseeCode::Old(licensee_code_old)
        }
        else {
            LicenseeCode::New(
                    ((rom.data[ROM_OFFSET_NEW_LICENSEE_CODE + 0] - '0' as u8) * 10)
                 |  ((rom.data[ROM_OFFSET_NEW_LICENSEE_CODE + 1] - '0' as u8) *  1)
            )
        };

        let cartridge = Cartridge {
            file: filepath.to_string(),

            title: rom.read_title(),

            manufacturer_code: rom.read_manufacturer_code(),
            licensee_code,

            mbc,

            rom_size,
            ram_size,

            supports_cgb,
            supports_sgb,

            has_ram,
            has_timer,
            has_battery,
            has_rumble,

            rom,
            ram,
        };

        Ok(cartridge)
    }


    /// Get the cartridge filename with a different file extension.
    pub fn get_filename_with_ext(&self, ext: &str) -> String {
        change_filename_ext(&self.file, ext)
    }


    /// get the plain data of this cartridge
    pub fn get_rom(&self) -> &RomData {
        &self.rom
    }

    /// Get the RAM banks of this cartridge.
    pub fn get_ram(&self) -> &MemoryDataDynamic {
        &self.ram
    }

    /// Get the mutable RAM banks of this cartridge.
    pub fn get_mut_ram(&mut self) -> &mut MemoryDataDynamic {
        &mut self.ram
    }

    /// Saves the RAM to a file, if the cartridge has battery powered RAM.
    pub fn save_ram_if_any(&self) -> io::Result<()> {
        if self.has_ram && self.has_battery {
            let ram_file = self.get_filename_with_ext(FILE_EXT_RAM);
            self.get_ram().save_to_file(&ram_file)?;
        }

        Ok(())
    }


    /// get the game's title
    pub fn get_title(&self) -> &String {
        &self.title
    }

    /// Computes the checksum of all 16 title bytes
    pub fn compute_title_checksum(&self) -> u8 {
        let mut checksum = 0u8;

        for addr in 0x0134..=0x0143 {
            checksum = checksum.wrapping_add(self.rom.get_at(addr));
        }

        checksum
    }

    /// get the game's manufacturer code
    pub fn get_manufacturer_code(&self) -> &str {
        &self.manufacturer_code
    }

    /// Get the licensee code
    pub fn get_licensee_code(&self) -> &LicenseeCode {
        &self.licensee_code
    }

    /// get the kind of game boy color support
    pub fn get_cgb_support(&self) -> &GameBoyColorSupport {
        &self.supports_cgb
    }

    /// checks whether this cartridge supports game boy color features
    pub fn supports_cgb(&self) -> bool {
        match self.supports_cgb {
            GameBoyColorSupport::None => false,
            _ => true,
        }
    }

    /// checks whether this cartridge supports super game boy features
    pub fn supports_sgb(&self) -> bool {
        self.supports_sgb
    }

    /// get the memory bank controller used by this cartridge
    pub fn get_mbc(&self) -> &MemoryBankController {
        &self.mbc
    }

    /// get the ROM size of this cartridge
    pub fn get_rom_size(&self) -> usize {
        self.rom_size
    }

    /// get the RAM size of this cartridge
    pub fn get_ram_size(&self) -> usize {
        self.ram_size
    }

    /// checks whether this cartridge has RAM modules
    pub fn has_ram(&self) -> bool {
        self.has_ram
    }

    /// checks whether this cartridge has a timer
    pub fn has_timer(&self) -> bool {
        self.has_timer
    }

    /// checks whether this cartridge has a battery
    pub fn has_battery(&self) -> bool {
        self.has_battery
    }

    /// checks whether this cartridge has a rumble module
    pub fn has_rumble(&self) -> bool {
        self.has_rumble
    }
}


impl Display for LicenseeCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LicenseeCode::Old(code) => write!(f, "{:02x} (old)", code),
            LicenseeCode::New(code) => write!(f, "{:02x} (new)", code),
        }
    }
}