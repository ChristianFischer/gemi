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

use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use gbemu_core::apu::audio_output::{AudioOutputSpec, SamplesReceiver};
use gbemu_core::gameboy::{DeviceType, EmulationType, GameBoy};
use gbemu_core::input::InputButton;
use gbemu_core::mmu::memory_data::MemoryData;
use crate::cartridge::Cartridge;


/// Web Assembly frontend for the emulator.
/// This will be instantiated from JS provides an interface to the emulator backend.
#[wasm_bindgen]
pub struct WasmPlayer {
    /// The emulator instance.
    gb: GameBoy,

    /// The rendering context of the canvas element assigned to receive the frames rendered.
    rc: CanvasRenderingContext2d,

    /// The channel receiver to receive audio samples from the emulator's APU.
    samples_receiver: Option<SamplesReceiver>,

    /// The key map to use for mapping JS key events to emulator input.
    key_map: KeyMap,

    /// The keys currently pressed.
    keys_pressed: HashMap<String, bool>,
}


type KeyMap = Vec<(InputButton, Vec<String>)>;

/// The default key map to use.
fn default_keymap() -> KeyMap {
    let entry = |button: InputButton, keys: Vec<&str>| {
        let keys = keys.iter().map(|s| s.to_string()).collect();
        (button, keys)
    };

    vec![
        entry(InputButton::DPadRight, vec!["d", "ArrowRight"]),
        entry(InputButton::DPadLeft,  vec!["a", "ArrowLeft"]),
        entry(InputButton::DPadUp,    vec!["w", "ArrowUp"]),
        entry(InputButton::DPadDown,  vec!["s", "ArrowDown"]),
        entry(InputButton::A,         vec!["e", "y"]),
        entry(InputButton::B,         vec!["q", "x"]),
        entry(InputButton::Select,    vec!["1", "Shift"]),
        entry(InputButton::Start,     vec!["2", "Enter"]),
    ]
}


#[wasm_bindgen]
impl WasmPlayer {
    /// Create a new emulator instance with an existing cartridge and a canvas element
    /// where to send the frame data to.
    #[wasm_bindgen]
    pub fn create_with_cartridge(
            cartridge: Cartridge,
            canvas: HtmlCanvasElement,
            desired_device: Option<String>,
    ) -> Result<WasmPlayer, JsValue> {
        // initialize GameBoy setup
        let mut builder = GameBoy::build();

        // select the device type to emulate
        let device = {
            match desired_device {
                // when a device type is given, parse it and throw an error if the type is invalid
                Some(abbr) => {
                    DeviceType::from_abbreviation(&abbr)
                        .ok_or_else(|| JsValue::from_str(&format!("Invalid device type '{abbr}'")))
                }?,

                // without any specific device type, select the device type
                // based on whether the cartridge supports GBC or not
                None => {
                    if cartridge.is_gbc() {
                        DeviceType::GameBoyColor
                    }
                    else {
                        DeviceType::GameBoyDmg
                    }
                }
            }
        };

        // apply the selected device type
        builder.set_device_type(device);

        // take the native cartridge out of it's wrapper
        builder.set_cartridge(cartridge.into());

        // get the rendering context
        let rc = canvas.get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Failed to get canvas context"))
            .map(|obj| obj.dyn_into::<CanvasRenderingContext2d>())??
        ;

        // finalize and initialize the emulator
        let mut gb = builder.finish()?;
        gb.initialize();

        Ok(
            WasmPlayer {
                gb,
                rc,

                samples_receiver: None,

                key_map: default_keymap(),
                keys_pressed: HashMap::new(),
            }
        )
    }


    /// Get the device type which is currently being emulated.
    #[wasm_bindgen]
    pub fn get_device_type(&self) -> String {
        self.gb.get_config().device.get_abbreviation().to_string()
    }


    /// Checks whether the emulator is currently running in GBC mode.
    #[wasm_bindgen]
    pub fn is_gbc_mode(&self) -> bool {
        match self.gb.get_config().emulation {
            EmulationType::DMG => false,
            EmulationType::GBC => true,
        }
    }


    /// Process the next frame and publish it to the canvas.
    #[wasm_bindgen]
    pub fn next_frame(&mut self) -> Result<(), JsValue> {
        self.process_frame();
        self.render_frame()?;

        Ok(())
    }


    /// Open the audio channel to the emulator.
    /// After doing so, audio samples may be received via [take_audio_samples].
    #[wasm_bindgen]
    pub fn open_audio(&mut self, sample_rate: u32) -> Result<(), JsValue> {
        self.samples_receiver = self.gb.get_peripherals_mut().apu.get_audio_output().open_channel(AudioOutputSpec {
            sample_rate
        });

        Ok(())
    }


    /// Takes all pending audio samples from the audio channel.
    /// This channel has to be opened via [open_audio] first.
    /// All pending samples will be put together into a continuous array with alternating between
    /// left and right channel samples.
    #[wasm_bindgen]
    pub fn take_audio_samples(&mut self) -> Result<Vec<f32>, JsValue> {
        match &self.samples_receiver {
            Some(receiver) => {
                Ok(
                    receiver
                        .try_iter()
                        .fuse()
                        .flat_map(|samples| samples.into_iter())
                        .flat_map(|sample| [sample.left.get_value(), sample.right.get_value()])
                        .collect::<Vec<_>>()
                )
            }

            None => {
                Err(JsValue::from_str("No audio channel available. Invoke open_audio first."))
            }
        }
    }


    /// If the current cartridge has a battery supported RAM,
    /// get the current RAM data as a byte array.
    #[wasm_bindgen]
    pub fn save_cartridge_ram(&self) -> Option<Vec<u8>> {
        self.gb
            .get_peripherals().mem
            .get_cartridge()
            .as_ref()
            .map(|cartridge| {
                if cartridge.has_ram() && cartridge.has_battery() {
                    Some(cartridge.get_ram().as_slice().to_vec())
                }
                else {
                    None
                }
            })
            .flatten()
   }


    /// Set the pressed state of a key.
    /// `key` is the key identifier as provided by the JS key event and will be mapped into
    /// the corresponding emulator [InputButton] value.
    #[wasm_bindgen]
    pub fn set_key_pressed(&mut self, key: String, pressed: bool) {
        self.keys_pressed.insert(key.clone(), pressed);

        for (button, key_list) in self.key_map.iter() {
            // find an entry in the keymap that contains the key
            if key_list.contains(&key) {
                // check if any of the keys in the list is pressed
                let pressed = key_list.iter()
                    .map(|key| self.keys_pressed.get(key).unwrap_or(&false))
                    .any(|pressed| *pressed)
                ;

                // set the button state
                self.gb.get_peripherals_mut().input.set_button_pressed(*button, pressed);
            }
        }
    }
}


impl WasmPlayer {
    /// Process a single frame until the next VBlank completion.
    pub fn process_frame(&mut self) {
        self.gb.process_frame();
    }


    /// Render the current frame to the canvas.
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        let frame = self.gb.get_peripherals().ppu.get_lcd();
        let image = ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(frame.get_pixels_as_slice()),
            frame.get_width(),
            frame.get_height()
        )?;

        self.rc.put_image_data(&image, 0.0, 0.0)
    }
}
