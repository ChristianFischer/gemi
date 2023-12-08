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


use egui::{Rect, Ui, WidgetText};
use egui_tiles::{SimplificationOptions, TileId, UiResponse};
use crate::state::EmulatorState;
use crate::view_response::ViewResponse;
use crate::views::{View, ViewClass};


/// The implementation of the behaviour trait for the UI tile tree.
pub struct TreeBehaviour {
    /// The current state of the emulator.
    /// Note: This is a bit misplaced here since it logically
    /// belongs to the application object, but required by the
    /// view objects.
    /// If there's a way to pass the state reference into the behaviour
    /// this should be moved into the application object.
    state: EmulatorState,
    
    /// Some options to control the behaviour of the tiled UI.
    simplification_options: SimplificationOptions,

    /// A response object collecting the results of the current frame.
    this_frame_response: ViewResponse,
}


impl TreeBehaviour {
    /// Reset the per-frame data of the behaviour.
    pub fn reset_frame_data(&mut self) {
        self.this_frame_response = ViewResponse::none();
    }


    /// Get the response object of the current frame.
    pub fn get_frame_response(&self) -> &ViewResponse {
        &self.this_frame_response
    }


    /// Get a reference to the emulator state.
    pub fn get_state(&self) -> &EmulatorState {
        &self.state
    }


    /// Get a mutable reference to the emulator state.
    pub fn get_state_mut(&mut self) -> &mut EmulatorState {
        &mut self.state
    }
}


impl Default for TreeBehaviour {
    fn default() -> Self {
        Self {
            state: EmulatorState::default(),

            simplification_options: SimplificationOptions {
                all_panes_must_have_tabs: true,
                .. Default::default()
            },

            this_frame_response: ViewResponse::none(),
        }
    }
}


impl egui_tiles::Behavior<ViewClass> for TreeBehaviour {
    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut ViewClass) -> UiResponse {
        let available_rect = ui.available_rect_before_wrap();

        // create a rectangle aligned to pixels for each child
        let child_rect = Rect::from_two_pos(
                available_rect.min.ceil(),
                available_rect.max.floor(),
        );

        // new ui object using the child rectangle
        let mut child_ui = Ui::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.id().with(_tile_id),
                child_rect,
                child_rect
        );

        let response = pane.ui(self.get_state_mut(), &mut child_ui);
        self.this_frame_response.add(response);

        // Currently no drag option here
        UiResponse::None
    }


    fn tab_title_for_pane(&mut self, pane: &ViewClass) -> WidgetText {
        pane.title(self.get_state_mut()).into()
    }


    fn simplification_options(&self) -> SimplificationOptions {
        self.simplification_options
    }
}
