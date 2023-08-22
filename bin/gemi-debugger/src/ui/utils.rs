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

use crate::views::ViewClass;


/// Visit all tiles of the given tree.
pub fn visit_tiles(
    tree: &mut egui_tiles::Tree<ViewClass>,
    mut visitor: impl FnMut(&mut ViewClass),
) {
    tree.tiles.tiles.iter_mut().for_each(|(_, tile)| {
        match tile {
            egui_tiles::Tile::Pane(pane) => {
                visitor(pane);
            }

            _ => { }
        }
    });
}
