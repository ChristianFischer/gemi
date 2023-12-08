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

use crate::event::UiEvent;


/// A response object to be returned by views to carry their results.
pub struct ViewResponse {
    /// An optional event that may have been triggered by the view.
    pub event: Option<UiEvent>,
}


impl ViewResponse {
    /// Creates a new, empty response object.
    pub fn none() -> Self {
        Self {
            event: None,
        }
    }


    /// Creates a new response object with an event.
    pub fn event(event: UiEvent) -> Self {
        Self {
            event: Some(event),
        }
    }


    pub fn add(&mut self, other: Self) {
        // take the event of the other response if not already set
        if self.event.is_none() {
            self.event = other.event;
        }
    }
}
