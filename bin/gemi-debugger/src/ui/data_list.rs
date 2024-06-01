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

use egui::Grid;


/// An UI element to display a list of pre-allocated data items.
/// For efficiency reasons, the data items in this list will be added
/// only once to avoid expensive calls like formatting.
pub struct DataList {
    /// The ID of the data list.
    id: String,

    /// A list of data items to be rendered.
    values: Vec<DataItem>,
}


/// A single data item to be rendered in a [`DataList`].
struct DataItem {
    /// The name of the data item.
    name: String,

    /// The value of the data item.
    value: Value,
}


/// A container for a single data item value.
pub enum Value {
    /// A text value to be displayed as a label.
    Text(String),

    /// A boolean value to be displayed as a checkbox value.
    Bool(bool),
}


impl DataList {
    /// Creates a new [`DataList`] object with an specific ID.
    pub fn new(id: String) -> Self {
        Self {
            id,
            values: Vec::new(),
        }
    }


    /// Clears all data items from this list.
    pub fn clear(&mut self) {
        self.values.clear();
    }


    /// Adds a new data item to this list.
    pub fn add_value(&mut self, name: &str, value: Value) {
        self.values.push(DataItem {
            name: name.to_string(),
            value,
        });
    }


    /// Adds a new text data item to this list.
    pub fn add_text(&mut self, name: &str, value: impl Into<String>) {
        self.add_value(name, Value::Text(value.into()));
    }


    /// Adds a new boolean data item to this list.
    pub fn add_bool(&mut self, name: &str, value: bool) {
        self.add_value(name, Value::Bool(value));
    }


    /// Renders this data list UI.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        Grid::new(&self.id)
            .num_columns(2)
            .spacing([20.0, 2.0])
            .show(ui, |ui| {
                self.values.iter_mut().for_each(|item| {
                    ui.label(&item.name);

                    // render each value based on their type
                    match &item.value {
                        Value::Text(text) => {
                            ui.label(text);
                        }

                        Value::Bool(value) => {
                            // shadow the original value, which should not be modified
                            let mut value = *value;
                            ui.checkbox(&mut value, "");
                        }
                    }

                    ui.end_row();
                });
            });
    }
}
