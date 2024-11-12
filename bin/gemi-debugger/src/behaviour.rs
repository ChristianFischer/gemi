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


use egui::{Rect, Ui, UiBuilder, WidgetText};
use egui_tiles::{SimplificationOptions, Tabs, TileId, Tiles, UiResponse};

use crate::state::EmulatorState;
use crate::strings;
use crate::views::{View, ViewClass};


/// Container for a [ViewClass] to be inserted into the UI.
pub struct ViewToInsert {
    /// The actual [ViewClass] to be added.
    pub view: ViewClass,
    
    /// The ID of the tile, where it should be added,
    /// usually a Tab container.
    pub insert_at: TileId,
}


/// The implementation of the behaviour trait for the UI tile tree.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TreeBehaviour {
    /// The current state of the emulator.
    /// Note: This is a bit misplaced here since it logically
    /// belongs to the application object, but required by the
    /// view objects.
    /// If there's a way to pass the state reference into the behaviour
    /// this should be moved into the application object.
    state: EmulatorState,

    /// Stores a [ViewClass] which was created and shall be added into the UI.
    #[serde(skip)]
    view_to_insert: Option<ViewToInsert>,
    
    /// Some options to control the behaviour of the tiled UI.
    #[serde(skip)]
    simplification_options: SimplificationOptions,
}


impl TreeBehaviour {
    /// Get a reference to the emulator state.
    pub fn get_state(&self) -> &EmulatorState {
        &self.state
    }


    /// Get a mutable reference to the emulator state.
    pub fn get_state_mut(&mut self) -> &mut EmulatorState {
        &mut self.state
    }
    
    
    /// If a [ViewClass] was prepared to be added, take the 
    /// view from the behaviour object.
    /// The view will be cleared from the behaviour after this.
    pub fn take_view_insert(&mut self) -> Option<ViewToInsert> {
        self.view_to_insert.take()
    }
}


impl Default for TreeBehaviour {
    fn default() -> Self {
        Self {
            state: EmulatorState::default(),
            
            view_to_insert: None,

            simplification_options: SimplificationOptions {
                all_panes_must_have_tabs: true,
                .. Default::default()
            },
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
                UiBuilder::new()
                    .max_rect(child_rect)
        );

        pane.ui(self.get_state_mut(), &mut child_ui);

        // Currently no drag option here
        UiResponse::None
    }


    fn tab_title_for_pane(&mut self, pane: &ViewClass) -> WidgetText {
        pane.title(self.get_state_mut()).into()
    }


    fn is_tab_closable(&self, _tiles: &Tiles<ViewClass>, _tile_id: TileId) -> bool {
        true
    }


    fn top_bar_right_ui(&mut self, _tiles: &Tiles<ViewClass>, ui: &mut Ui, tile_id: TileId, _tabs: &Tabs, _scroll_offset: &mut f32) {
        // border to the right side to ensure the button and it's border are fully visible
        ui.add_space(2.0);

        ui.menu_button(strings::BUTTON_LABEL_ADD, |ui| {
            for (name, instantiate) in &ViewClass::ALL {
                if ui.button(*name).clicked() {
                    self.view_to_insert = Some(ViewToInsert { 
                        view: instantiate(),
                        insert_at: tile_id
                    });
                }
            }
        });
    }


    fn simplification_options(&self) -> SimplificationOptions {
        self.simplification_options
    }
}
