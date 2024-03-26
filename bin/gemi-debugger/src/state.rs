/*
 * Copyright (C) 2022-2024 by Christian Fischer
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
use std::path::Path;

use gemi_core::cartridge::Cartridge;
use gemi_core::gameboy::GameBoy;
use gemi_core::input::InputButton;
use gemi_utils::keybindings::KeyBindings;

use crate::selection::{Kind, Selection};

/// An enum to store the different update modes of the emulator
/// inside of this debugger application.
#[derive(serde::Serialize, serde::Deserialize)]
pub enum UpdateMode {
    /// The execution of the emulator is currently paused.
    Paused,

    /// The emulator is currently running in continuous mode.
    /// This will run the emulator in real time and continuously spawn new frames.
    Continuous,

    /// Run the emulator for a single step and the switch into pause mode.
    /// The type of step is determined by the [SingleStepMode] parameter.
    Step,
}


#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone)]
/// an enum to define which step to be done when running the
/// emulator in single step mode.
/// See [UpdateMode::Step].
pub enum UpdateStepMode {
    /// Runs the emulator until the next frame was completed.
    Frame,
    
    /// Runs the emulator for a single instruction only.
    Instruction,
}



/// An object handling the current state of the emulator.
/// This provides functionality to load ROMs and serialize the emulator state.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmulatorState {
    /// The actual state of the emulator, which also
    /// contains the emulator instance itself.
    pub emu: EmulatorInstance,

    /// Various states of the UI, which contains the state of buttons
    /// or any active selection.
    pub ui: UiStates,
}


/// An object to store the instance of a running emulator
/// and to provide functionality to run the emulation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmulatorInstance {
    /// The actual instance of the emulator.
    /// Will be [None] if no ROM is loaded.
    #[serde(skip)]
    gb: Option<GameBoy>,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct UiStates {
    /// The key bindings used to control the emulator.
    #[serde(skip)]
    key_bindings: KeyBindings<egui::Key>,

    /// The current update mode of the emulator.
    update_mode: UpdateMode,

    /// When [update_mode] is [UpdateMode::Step], this determines
    /// which kind of step to perform.
    update_step_mode: UpdateStepMode,

    /// Describes the currently selected focus item within the UI.
    pub focus: Selection,
    
    /// While moving the mouse cursor over the UI, this will contain the
    /// currently hovered item.
    pub hover: Selection,
}


impl Default for UpdateMode {
    fn default() -> Self {
        Self::Paused
    }
}


impl Default for UpdateStepMode {
    fn default() -> Self {
        Self::Frame
    }
}


impl Display for UpdateStepMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateStepMode::Frame       => write!(f, "Frame"),
            UpdateStepMode::Instruction => write!(f, "Instruction"),
        }
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
        let mut gb = builder.finish()?;
        gb.initialize();

        // reset key states after emulator loading
        self.ui.key_bindings.reset_key_states(&mut gb);

        // store the new emulator instance
        self.emu.gb = Some(gb);

        // success!
        Ok(())
    }


    /// Checks whether the emulator is currently running or not.
    /// This will be the case if the emulator is loaded and not paused.
    pub fn is_running(&self) -> bool {
        self.emu.is_emulator_loaded() && !self.ui.is_paused()
    }


    /// Update the emulator, if any.
    pub fn update(&mut self) {
        self.ui.update_mode = match self.ui.update_mode {
            // stay in pause state
            UpdateMode::Paused => {
                UpdateMode::Paused
            }

            // process the next frame and stay in continuous mode
            UpdateMode::Continuous => {
                self.emu.run_frame();
                UpdateMode::Continuous
            }

            // process the next step and switch into pause mode
            UpdateMode::Step => {
                match self.ui.update_step_mode {
                    UpdateStepMode::Frame       => self.emu.run_frame(),
                    UpdateStepMode::Instruction => self.emu.run_single_step(),
                }

                UpdateMode::Paused
            }
        }
    }


    /// Forward key events into the emulator.
    pub fn set_key_pressed(&mut self, key: egui::Key, pressed: bool) {
        if let Some(gb) = &mut self.emu.gb {
            self.ui.key_bindings.set_key_pressed_and_fwd(key, pressed, gb);
        }
    }
}


impl EmulatorInstance {
    /// Get the currently running emulator instance.
    pub fn get_emulator(&self) -> Option<&GameBoy> {
        self.gb.as_ref()
    }


    /// Get the currently running emulator instance.
    pub fn get_emulator_mut(&mut self) -> Option<&mut GameBoy> {
        self.gb.as_mut()
    }


    /// Check if an emulator instance is currently loaded.
    pub fn is_emulator_loaded(&self) -> bool {
        self.gb.is_some()
    }


    /// Get the cartridge of the currently running emulator instance, if any.
    pub fn get_cartridge(&self) -> Option<&Cartridge> {
        self.get_emulator()
            .and_then(|emu| emu.get_peripherals().mem.get_cartridge())
    }


    /// Process a single frame of the emulator, if any.
    pub fn run_frame(&mut self) {
        if let Some(emu) = self.get_emulator_mut() {
            emu.run_frame();
        }
    }


    pub fn run_single_step(&mut self) {
        if let Some(emu) = self.get_emulator_mut() {
            emu.run_single_step();
        }
    }
}


impl UiStates {
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


    /// Change the emulator's update mode.
    pub fn set_update_mode(&mut self, mode: UpdateMode) {
        self.update_mode = mode;
    }


    /// Get what will be performed a single step.
    pub fn get_update_step_mode(&self) -> &UpdateStepMode {
        &self.update_step_mode
    }


    /// Set what will be performed a single step.
    pub fn set_update_step_mode(&mut self, mode: UpdateStepMode) {
        self.update_step_mode = mode;
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
            emu: EmulatorInstance {
                gb: None,
            },

            ui: UiStates {
                key_bindings:       make_default_key_bindings(),
                update_mode:        UpdateMode::Paused,
                update_step_mode:   UpdateStepMode::Frame,
                focus:              Selection::new(Kind::Focus),
                hover:              Selection::new(Kind::Hover),
            },
        }
    }
}
