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

use egui::Ui;
use crate::state::EmulatorState;
use crate::view_response::ViewResponse;
use crate::views::View;


/// A placeholder view to reserve space for views currently not implemented.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlaceholderView {
    /// The name of the view to be placed instead of the placeholder.
    name: String,
}


impl PlaceholderView {
    /// Creates a new [`PlaceholderView`] object with a dedicated name.
    pub fn new(name: &str) -> Self {
        PlaceholderView {
            name: name.to_string(),
        }
    }
}


impl View for PlaceholderView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        &self.name
    }


    fn ui(&mut self, _state: &mut EmulatorState, _ui: &mut Ui) -> ViewResponse {
        ViewResponse::none()
    }
}
