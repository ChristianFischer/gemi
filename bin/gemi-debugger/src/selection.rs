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

use std::ops::Range;

use crate::event::UiEvent;

/// A struct describing an item currently being selected or highlighted.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(PartialEq, Clone)]
pub enum Selected {
    /// The selection is on a sprite, which is defined by the index of a 
    /// VRAM bank and the sprites index.
    Sprite(u8, usize),

    /// The selection is on an entry within the OAM table, defined by its index.
    OamEntry(usize),
    
    /// The selection is on a tile.
    /// The parameters contain the index of the tile and the
    /// [TileMap] it belongs to, referred by its value of the selection bit
    /// used in the LCDC register.
    Tile(bool, usize),

    /// The selection is on a disassembled instruction,
    /// referred by its address range.
    Instruction(Range<u16>),
}


/// Describes a kind of selection.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(PartialEq, Clone)]
pub enum Kind {
    /// An item which was actively selected and brought to the user's focus.
    /// While a 'selection' is unique to its [View], the 'focus' is the 
    /// application wide most recent selection.
    Focus,

    /// An item currently hovered with the mouse cursor.
    Hover,
}


/// A struct storing information of a current selection.
/// This can be used to change or clear the selection using the functions
/// provided by this struct. Changing a selection will cause events
/// to be fired to each view of this application.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Selection {
    /// The kind of selection managed by this object.
    kind: Kind,

    /// The currently active selection, which can be either a [Selected]
    /// or [None].
    selection: Option<Selected>,

    /// Flag to store when the selection was changed.
    changed: bool,
}


impl Selection {
    /// Creates a new [Selection] object.
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            selection: None,
            changed: false,
        }
    }


    /// Get the kind of selection handled by this object.
    pub fn get_kind(&self) -> &Kind {
        &self.kind
    }


    /// Get the current selection.
    pub fn get(&self) -> Option<&Selected> {
        self.selection.as_ref()
    }


    /// Checks whether a specific item is selected or not.
    pub fn is_selected(&self, selection: &Selected) -> bool {
        self.selection.as_ref() == Some(selection)
    }


    /// Either sets or clears the given selection, depending on the flag.
    pub fn set(&mut self, selection: Selected, is_selected: bool) {
        if is_selected {
            self.select(selection);
        }
        else {
            self.clear(selection);
        }
    }


    /// Change the active selection by selecting a new item.
    /// This will cause a [UiEvent::SelectionChanged] event in the next frame.
    pub fn select(&mut self, selection: Selected) {
        if self.selection.as_ref() != Some(&selection) {
            self.selection = Some(selection);
            self.changed   = true;
        }
    }


    /// Toggles a specific selection.
    /// Either the given [Selection] becomes the active one, or, if already
    /// selected, it will be deselected and the current selection being cleared.
    pub fn toggle(&mut self, selection: Selected) {
        if self.selection.as_ref() != Some(&selection) {
            self.selection = Some(selection);
        }
        else {
            self.selection = None;
        }

        self.changed = true;
    }


    /// Clear the current selection.
    /// This will cause a [UiEvent::SelectionChanged] event in the next frame.
    pub fn clear(&mut self, selection: Selected) {
        if self.selection == Some(selection) {
            self.selection = None;
            self.changed   = true;
        }
    }


    /// When changed since being called last time, this will return an
    /// [UiEvent] covering the selection change event.
    /// The `changed` flag will be reset after that.
    pub fn take_ui_event(&mut self) -> Option<UiEvent> {
        if self.changed {
            self.changed = false;

            Some(UiEvent::SelectionChanged(
                    self.get_kind().clone(),
                    self.selection.clone()
            ))
        }
        else {
            None
        }
    }
}
