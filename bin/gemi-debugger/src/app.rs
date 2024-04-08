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

use std::path::PathBuf;

use eframe::{CreationContext, Frame};
use egui::{ComboBox, Context};

use crate::behaviour::TreeBehaviour;
use crate::event::UiEvent;
use crate::state::{EmulatorState, UpdateMode, UpdateStepMode};
use crate::strings::*;
use crate::ui::sprite_cache;
use crate::ui::utils::visit_tiles;
use crate::views::{View, ViewClass};

/// The main application struct.
/// This contains the root elements of the UI
/// and provides access to the emulator state.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmulatorApplication {
    /// The tree object to handle the set of view objects
    /// and therefor acting as the root element of the tiled UI.
    tree: egui_tiles::Tree<ViewClass>,

    /// The implementation of the tiles behaviour trait
    /// which controls the behaviour of the tiled UI.
    #[serde(skip)]
    behaviour: TreeBehaviour,

    /// A user notification to be displayed in a message box.
    #[serde(skip)]
    display_message: Option<String>,

    /// When a notification is displayed to the user,
    /// setting this flag to true will close the message box.
    #[serde(skip)]
    close_message: bool,
}


impl EmulatorApplication {
    /// Try to load the serialized application state from the eframe storage.
    pub fn from_creation_context(cc: &CreationContext) -> Option<Self> {
        let mut app = eframe::get_value::<EmulatorApplication>(
            cc.storage?,
            eframe::APP_KEY
        )?;

        // when successfully restored and there's a ROM loaded, notify all views
        if app.get_state().emu.is_emulator_loaded() {
            visit_tiles(
                &mut app.tree,
                |tile| {
                    tile.on_emulator_loaded(app.behaviour.get_state_mut());
                }
            );
        }

        Some(app)
    }


    /// Get a reference to the emulator state.
    pub fn get_state(&self) -> &EmulatorState {
        self.behaviour.get_state()
    }


    /// Get a mutable reference to the emulator state.
    pub fn get_state_mut(&mut self) -> &mut EmulatorState {
        self.behaviour.get_state_mut()
    }
}


impl Default for EmulatorApplication {
    fn default() -> Self {
        let mut tiles = egui_tiles::Tiles::<ViewClass>::default();

        // utility function to add a set of views into the tiles UI
        let mut add_views = |views: Vec<ViewClass>| {
            let tile_ids = views
                .into_iter()
                .map(|view| tiles.insert_pane(view))
                .collect::<Vec<_>>()
            ;

            tiles.insert_tab_tile(tile_ids)
        };

        // views for the main area in the window's center
        let tiles_main = add_views(vec![
            ViewClass::new_display_view(),
            ViewClass::new_tile_map(),
        ]);

        // views for the sidebar on the right
        let tiles_sidebar = add_views(vec![
            ViewClass::new_cartridge_info(),
            ViewClass::new_cpu(),
            ViewClass::new_sprites(),
        ]);

        // views for the bottom area below the main area
        let tiles_bottom = add_views(vec![
            ViewClass::new_memory(),
            ViewClass::new_disassembly(),
            ViewClass::new_oam(),
        ]);

        // create a split between the main area and a bottom area below
        let v_split = {
            let linear = egui_tiles::Linear::new_binary(
                egui_tiles::LinearDir::Vertical,
                [ tiles_main, tiles_bottom ],
                0.75
            );

            let container = egui_tiles::Container::Linear(linear);

            tiles.insert_container(container)
        };

        // create another split for the sidebar on the right
        let h_split = {
            let linear = egui_tiles::Linear::new_binary(
                egui_tiles::LinearDir::Horizontal,
                [ v_split, tiles_sidebar ],
                0.8
            );

            let container = egui_tiles::Container::Linear(linear);

            tiles.insert_container(container)
        };

        // create the tree object
        let tree = egui_tiles::Tree::new("gemi", h_split, tiles);

        Self {
            tree,
            behaviour:          TreeBehaviour::default(),
            display_message:    None,
            close_message:      false,
        }
    }
}


impl eframe::App for EmulatorApplication {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        sprite_cache::on_frame();

        self.get_state_mut().update();

        self.update_menu_bar(ctx, frame);
        self.update_center_panel(ctx, frame);
        self.update_message_box(ctx, frame);
        self.update_input(ctx);
        self.handle_frame_response();

        // when the emulator is still running, request an immediate repaint
        // to update the display instead of waiting for the next event
        if self.get_state().is_running() {
            ctx.request_repaint();
        }
    }


    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self);
    }
}


impl EmulatorApplication {
    pub fn open_rom(&mut self, path: &PathBuf) -> Result<(), String> {
        let state = self.behaviour.get_state_mut();

        // open the ROM file
        state.open_rom(path)?;

        // on success, notify the views
        visit_tiles(
            &mut self.tree,
            |tile| {
                tile.on_emulator_loaded(state);
            }
        );

        Ok(())
    }


    /// Handle the menu bar at the top of the window.
    fn update_menu_bar(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let enabled = !self.is_message_box_open();
                ui.set_enabled(enabled);

                ui.menu_button("File", |ui| self.update_submenu_file(ui, frame));
                ui.separator();

                self.update_player_toolbar(ui);
            });
        });
    }


    /// Handle the "File" submenu of the menu bar.
    fn update_submenu_file(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        // "Open" button to open a ROM file
        if ui.button("Open").clicked() {
            ui.close_menu();

            // open a file dialog to select a ROM file
            match nfd2::open_file_dialog(Some("gb,gbc;*;txt"), None) {
                Ok(nfd2::Response::Okay(path)) => {
                    // try to load the file
                    if let Err(e) = self.open_rom(&path) {
                        // display an error message on failure
                        self.display_message_box(&format!("Error: {}", e));
                    }
                }

                Ok(nfd2::Response::OkayMultiple(_)) => {
                    // cannot handle multiple files
                    self.display_message_box("Cannot handle multiple files");
                }

                Err(e) => {
                    // display an error message on failure
                    self.display_message_box(&format!("Error: {}", e));
                }

                // ignore any other result (e.g. user cancelled)
                _ => { }
            }
        }

        // "Quit" button to close the application
        if ui.button("Quit").clicked() {
            ui.close_menu();
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }


    fn update_player_toolbar(&mut self, ui: &mut egui::Ui) {
        let state = self.get_state_mut();
        let mut is_running = state.is_running();
        let mut is_paused  = state.ui.is_paused();

        // "Play" button
        if ui.toggle_value(&mut is_running, BUTTON_LABEL_PLAY).clicked() {
            if is_running {
                state.ui.set_update_mode(UpdateMode::Continuous);
            }
        }

        // "Pause" button
        if ui.toggle_value(&mut is_paused,  BUTTON_LABEL_PAUSE).clicked() {
            if is_paused {
                state.ui.set_update_mode(UpdateMode::Paused);
            }
        }

        // "Step" button
        if ui.button(BUTTON_LABEL_STEP).clicked() {
            state.ui.set_update_mode(UpdateMode::Step);
        }

        // Step type
        {
            let mut selected_index = *state.ui.get_update_step_mode() as usize;

            let all_modes = [
                UpdateStepMode::Frame,
                UpdateStepMode::Line,
                UpdateStepMode::Instruction
            ];

            let response = ComboBox::from_id_source("update_step")
                    .show_index(
                        ui,
                        &mut selected_index,
                        all_modes.len(),
                        |i| all_modes[i].to_string()
                    )
            ;
            
            if response.changed() {
                state.ui.set_update_step_mode(all_modes[selected_index]);
            }
        }
    }


    /// Handle the content area of the window.
    fn update_center_panel(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(
            ctx,
            |ui| {
                let enabled = !self.is_message_box_open();
                ui.set_enabled(enabled);

                self.tree.ui(&mut self.behaviour, ui);
            }
        );
    }


    /// Handle the message box, if any.
    fn update_message_box(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Some(msg) = &self.display_message {
            // render message box window
            egui::Window::new("Message")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(msg);
                    if ui.button("Ok").clicked() {
                        // set the flag to close the message box window
                        self.close_message = true;
                    }
                })
            ;
        }

        // close the message box window if the user clicked the "Ok" button
        if self.close_message {
            self.display_message = None;
            self.close_message = false;
        }
    }


    /// Handles input events
    fn update_input(&mut self, ctx: &Context) {
        ctx.input(|input| {
            input.events.iter().for_each(|event| {
                match event {
                    egui::Event::Key { key, pressed, repeat, .. } => {
                        self.on_key_event(*key, *pressed, *repeat);
                    }

                    _ => { }
                }
            });
        });
    }


    /// Handle key pressed or key released events and
    /// forwards them into the emulator
    fn on_key_event(&mut self, key: egui::Key, pressed: bool, repeat: bool) {
        // ignore repeated events, since we only need to track the pressed state once
        if !repeat {
            self.get_state_mut().set_key_pressed(key, pressed);
        }
    }


    /// Display a message box with the given message.
    /// The message box will be displayed until the user clicks the "Ok" button.
    fn display_message_box(&mut self, msg: &str) {
        self.display_message = Some(msg.to_string());
        self.close_message = false;
    }


    /// Check if a message box is currently open.
    fn is_message_box_open(&self) -> bool {
        self.display_message.is_some()
    }


    /// Handle the response of the views during this frame.
    fn handle_frame_response(&mut self) {
        let events = [
            self.behaviour.get_state_mut().ui.focus.take_ui_event(),
            self.behaviour.get_state_mut().ui.hover.take_ui_event(),
        ].into_iter().filter_map(|event| event);

        for event in events {
            self.send_event(event);
        }
    }
    
    
    /// Send a single event to all views. 
    fn send_event(&mut self, event: UiEvent) {
        visit_tiles(
            &mut self.tree,
            |tile| {
                tile.handle_ui_event(&event);
            }
        );
    }
}
