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

use std::path::Path;
use gemi_core::cartridge::Cartridge;
use gemi_core::gameboy::GameBoy;
use gemi_core::input::InputButton;
use gemi_utils::keybindings::KeyBindings;


/// An enum to store the different update modes of the emulator
/// inside of this debugger application.
pub enum UpdateMode {
    /// The execution of the emulator is currently paused.
    Paused,

    /// The emulator is currently running in continuous mode.
    /// This will run the emulator in real time and continuously spawn new frames.
    Continuous,

    /// Run the emulator until the next frame is finished and then switch into pause mode.
    StepFrame,

    /// Executes the next instruction and then switches into pause mode.
    StepInstruction,
}



/// An object handling the current state of the emulator.
/// This provides functionality to load ROMs and serialize the emulator state.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmulatorState {
    /// The actual instance of the emulator.
    /// Will be [None] if no ROM is loaded.
    #[serde(skip)]
    emu: Option<GameBoy>,

    /// The key bindings used to control the emulator.
    #[serde(skip)]
    key_bindings: KeyBindings<egui::Key>,

    /// The current update mode of the emulator.
    #[serde(skip)]
    update_mode: UpdateMode,
}


impl Default for UpdateMode {
    fn default() -> Self {
        Self::Paused
    }
}


impl EmulatorState {
    /// Open a ROM file and load it into the emulator.
    /// If there's already a running instance of the emulator, this will be closed.
    pub fn open_rom(&mut self, path: &Path) -> Result<(), String> {
        // load the cartridge from the given path
        let cartridge = Cartridge::load_files_with_default_ram(path)
                .map_err(|e| format!("Failed to load ROM: {}", e))
                ?
        ;

        // on success build the new emulator instance
        let mut builder = GameBoy::build();
        builder.set_cartridge(cartridge);

        // finish & initialize
        let mut emu = builder.finish()?;
        emu.initialize();

        // reset key states after emulator loading
        self.key_bindings.reset_key_states(&mut emu);

        // store the new emulator instance
        self.emu = Some(emu);

        // success!
        Ok(())
    }


    /// Update the emulator, if any.
    pub fn update(&mut self) {
        self.update_mode = match self.update_mode {
            // stay in pause state
            UpdateMode::Paused => {
                UpdateMode::Paused
            }

            // process the next frame and stay in continuous mode
            UpdateMode::Continuous => {
                self.process_frame();
                UpdateMode::Continuous
            }

            // process the next frame and switch into pause mode
            UpdateMode::StepFrame => {
                self.process_frame();
                UpdateMode::Paused
            }

            // process the next instruction and switch into pause mode
            UpdateMode::StepInstruction => {
                // not implemented yet
                UpdateMode::Paused
            }
        }
    }


    /// Forward key events into the emulator.
    pub fn set_key_pressed(&mut self, key: egui::Key, pressed: bool) {
        if let Some(emu) = &mut self.emu {
            self.key_bindings.set_key_pressed_and_fwd(key, pressed, emu);
        }
    }


    /// Get the currently running emulator instance.
    pub fn get_emulator(&self) -> Option<&GameBoy> {
        self.emu.as_ref()
    }


    /// Get the currently running emulator instance.
    pub fn get_emulator_mut(&mut self) -> Option<&mut GameBoy> {
        self.emu.as_mut()
    }


    /// Check if an emulator instance is currently loaded.
    pub fn is_emulator_loaded(&self) -> bool {
        self.emu.is_some()
    }


    /// Get the cartridge of the currently running emulator instance, if any.
    pub fn get_cartridge(&self) -> Option<&Cartridge> {
        self.get_emulator()
            .and_then(|emu| emu.get_peripherals().mem.get_cartridge())
    }


    /// Get the current update mode of the emulator.
    pub fn get_update_mode(&self) -> &UpdateMode {
        &self.update_mode
    }


    /// Checks whether the emulator is currently paused or not.
    pub fn is_paused(&self) -> bool {
        match self.update_mode {
            UpdateMode::Paused => true,
            _ => false,
        }
    }


    /// Checks whether the emulator is currently running or not.
    /// This will be the case if the emulator is loaded and not paused.
    pub fn is_running(&self) -> bool {
        self.is_emulator_loaded() && !self.is_paused()
    }


    /// Change the emulator's update mode.
    pub fn set_update_mode(&mut self, mode: UpdateMode) {
        self.update_mode = mode;
    }


    /// Process a single frame of the emulator, if any.
    pub fn process_frame(&mut self) {
        if let Some(emu) = self.get_emulator_mut() {
            emu.process_frame();
        }
    }
}



fn make_default_key_bindings() -> KeyBindings<egui::Key> {
    KeyBindings::with_mapping(
        vec![
            (InputButton::DPadRight,    vec![egui::Key::D,      egui::Key::ArrowRight   ]),
            (InputButton::DPadLeft,     vec![egui::Key::A,      egui::Key::ArrowLeft    ]),
            (InputButton::DPadUp,       vec![egui::Key::W,      egui::Key::ArrowUp      ]),
            (InputButton::DPadDown,     vec![egui::Key::S,      egui::Key::ArrowDown    ]),
            (InputButton::A,            vec![egui::Key::E,      egui::Key::X            ]),
            (InputButton::B,            vec![egui::Key::Q,      egui::Key::Y            ]),
            (InputButton::Select,       vec![egui::Key::Num1,   egui::Key::Backspace    ]),
            (InputButton::Start,        vec![egui::Key::Num2,   egui::Key::Enter        ]),
        ]
    )
}


impl Default for EmulatorState {
    fn default() -> Self {
        Self {
            emu: None,
            key_bindings: make_default_key_bindings(),
            update_mode: UpdateMode::Paused,
        }
    }
}
