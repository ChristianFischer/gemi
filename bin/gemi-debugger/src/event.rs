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

/// An event risen by any UI component which will be sent to other components.
#[derive(Clone)]
pub enum UiEvent {
    /// A sprite was selected.
    /// Contains the index of the selected sprite.
    SpriteSelected(usize),

    /// A sprite was deselected.
    /// Contains the index of the deselected sprite.
    SpriteDeselected(usize),
}
