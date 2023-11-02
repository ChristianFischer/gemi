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

use std::cmp::max;
use std::marker::PhantomData;
use std::ops::RangeInclusive;
use std::string::ToString;
use egui::{Grid, Label, PointerButton, ScrollArea, Sense, TextEdit, TextStyle, Ui, Vec2, vec2, Widget, WidgetText};
use crate::ui::style::GemiStyle;


/// A placeholder value in case when a memory address was not readable.
const PLACEHOLDER_NO_VALUE: &str = "--";


/// A generic memory editor, which can display and edit memory values
/// from any given source.
/// The type of the source is configured via the generic type parameter.
/// This object is serializable and stores it's current state.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MemoryEditor<Source> {
    /// A set of memory areas to be displayed.
    /// The memory editor needs at least one memory area
    /// in order to display any data.
    #[serde(skip)]
    memory_areas: Vec<MemoryArea>,

    /// Runtime information of this editor instance.
    #[serde(skip)]
    rt: MemoryEditorRuntimeData,

    /// A phantom data that does not occupy memory,
    /// but allows us to use the generic type parameter.
    #[serde(skip)]
    _phantom_data: PhantomData<Source>,

    /// The serializable state of this memory editor.
    state: MemoryEditorState,
}


/// A specific memory area additional with it's parameters
/// to be displayed in the memory editor.
pub struct MemoryArea {
    /// The name to be displayed along the memory area.
    name: String,

    /// The range of memory addresses to be displayed.
    memory_range: RangeInclusive<usize>,

    /// Whether the memory area is writable.
    writable: bool,

    /// Whether the memory area is expanded or collapsed.
    expanded: bool,
}


/// The serializable state of the memory editor.
#[derive(serde::Serialize, serde::Deserialize)]
struct MemoryEditorState {
    /// The currently selected memory address.
    selected_address: usize,

    /// Whether editing the memory values is enabled or not.
    is_editable: bool,
}


/// The internal data of the memory editor which will not be serialized.
struct MemoryEditorRuntimeData {
    /// Flag to store whether we need to measure UI sizes
    /// next time before rendering.
    need_to_measure: bool,

    /// The last measured width of the frame.
    /// If this is different next time, this will trigger a re-measurement.
    last_frame_width: f32,

    /// Height of the content within a single line.
    line_content_height: f32,

    /// The distance between two lines on the Y axis.
    /// This is the sum of the content height and the spacing.
    line_distance_y: f32,

    /// The width of the 'category' column.
    column_width_category: f32,

    /// The width of the 'address' column.
    column_width_address: f32,

    /// The number of columns to be displayed per line.
    columns_per_line: usize,

    /// The upper bound of the address range (meaning the largest address to be displayed).
    address_upper_bound: usize,

    /// The number of characters to be displayed for an address label.
    number_of_address_characters: usize,

    /// Stores whether the memory editor is currently in edit mode or not.
    is_editing: bool,

    /// The current content of the text to be edited while in edit mode.
    edit_string: String,

    /// The width of the current editor text box, inherited from it's previous label.
    edit_label_width: f32,

    /// This flag intends to request focus for the edit box the first time
    /// after entering edit mode.
    edit_label_request_focus: bool,
}


impl<Source> MemoryEditor<Source> {
    /// Creates a new [MemoryEditor] object.
    pub fn new() -> Self {
        Self {
            memory_areas: Vec::new(),
            rt: MemoryEditorRuntimeData::default(),
            _phantom_data: PhantomData::default(),

            state: MemoryEditorState {
                selected_address: 0,
                is_editable: false,
            }
        }
    }


    /// Enables or disables the ability to edit memory values.
    pub fn set_editable(&mut self, editable: bool) {
        self.state.is_editable = editable;
    }


    /// Returns whether the ability to edit memory values is enabled or not.
    pub fn is_editable(&self) -> bool {
        self.state.is_editable
    }


    /// Adds a new memory area to be displayed.
    pub fn add_memory_area(&mut self, name: impl Into<String>, memory_range: RangeInclusive<usize>, writable: bool) {
        self.memory_areas.push(MemoryArea {
            name: name.into(),
            memory_range,
            writable,
            expanded: true,
        });

        self.update_memory_areas();

        // need to update measurements
        self.rt.need_to_measure = true;
    }


    /// Removes all memory areas from this editor, leaving it empty.
    pub fn clear_memory_areas(&mut self) {
        self.memory_areas.clear();
    }


    /// To be called after changes of the memory areas to update
    /// internal data which depends on them.
    fn update_memory_areas(&mut self) {
        self.rt.address_upper_bound = self.memory_areas.iter()
            .map(|area| *area.memory_range.end())
            .max()
            .unwrap_or(0)
        ;

        // depending on the largest address, select the width of the address label
        self.rt.number_of_address_characters = match self.rt.address_upper_bound {
            0x0000_0000..=0x0000_00ff => 2,
            0x0000_0100..=0x0000_ffff => 4,
            0x0001_0000..=0xffff_ffff => 8,
            _                         => 16,
        };

    }


    /// Find the category for a given address.
    /// In case there are multiple categories for the same address,
    /// it will return the first one appearing in the list.
    /// If no category was found, it will return `None`.
    fn find_category_for_address(&mut self, address: usize) -> Option<&MemoryArea> {
        for memory_area in &self.memory_areas {
            if memory_area.memory_range.contains(&address) {
                return Some(memory_area);
            }
        }

        None
    }


    /// Show the memory editor.
    /// This will take into account the current state of the memory editor
    /// and takes the current values from the source delivered to this function.
    pub fn show(
            &mut self,
            ui: &mut Ui,
            source: &mut Source,
            on_read: impl Fn(&Source, usize) -> Option<u8>,
            on_write: impl FnMut(&mut Source, usize, u8)
    ) {
        self.show_internal(
            ui,
            source,
            &Box::new(on_read),
            &mut Box::new(on_write)
        );
    }


    /// Internal function of [MemoryEditor::show] having it's on_read and on_write
    /// functions boxed.
    fn show_internal(
            &mut self,
            ui: &mut Ui,
            source: &mut Source,
            on_read: &Box<impl Fn(&Source, usize) -> Option<u8>>,
            on_write: &mut Box<impl FnMut(&mut Source, usize, u8)>
    ) {
        self.measure_if_needed(ui);

        // create the widget for the scroll area
        let scroll_area = ScrollArea::vertical()
            .id_source("memory_view_scroll_area")
            .auto_shrink([false, false])
        ;

        // the maximum number of lines to be displayed
        let max_lines = (self.rt.address_upper_bound + 1) / self.rt.columns_per_line;

        // render the grid with all memory cells
        scroll_area.show_rows(
            ui,
            self.rt.line_content_height,
            max_lines,
            |ui, rows| {
                Grid::new("memory_view_grid")
                    .num_columns(3)
                    .min_col_width(10.0)
                    .show(ui, |ui| {
                        for row in rows {
                            let start_address = row * self.rt.columns_per_line;
                            self.display_memory_line(
                                    ui,
                                    source,
                                    start_address,
                                    on_read,
                                    on_write
                            );
                        }
                    })
                ;
            }
        );
    }


    /// Renders a single line within the memory editor.
    /// Each line begins with the address label, followed by the configured amount of memory cells.
    /// Optionally, each line may render an optional category header.
    fn display_memory_line(
            &mut self,
            ui: &mut Ui,
            source: &mut Source,
            start_address: usize,
            on_read: &Box<impl Fn(&Source, usize) -> Option<u8>>,
            on_write: &mut Box<impl FnMut(&mut Source, usize, u8)>
    ) {
        let line_address_range = start_address..(start_address + self.rt.columns_per_line);
        let is_line_selected   = line_address_range.contains(&self.state.selected_address);
        let item_spacing       = ui.spacing().item_spacing.x;

        // get the category properties
        let is_writable = match self.find_category_for_address(start_address) {
            Some(memory_area) => memory_area.writable,
            None => false,
        };

        // display the category header, if any
        self.display_category_header_at(
            ui,
            start_address,
            start_address + self.rt.columns_per_line - 1
        );

        // display the address label
        {
            let address_str = format!(
                "{:0width$x}",
                start_address,
                width = self.rt.number_of_address_characters
            );

            if is_line_selected {
                let address_text = GemiStyle::ADDRESS_SELECTED.rich_text(address_str);
                ui.label(address_text);
            }
            else {
                let address_text = GemiStyle::ADDRESS.rich_text(address_str);
                ui.label(address_text);
            };
        }

        // display the line of memory cells
        ui.horizontal(|ui| {
            //let space = self.rt.character_width.ceil();
            let space = item_spacing;

            for i in 0..self.rt.columns_per_line {
                let address = start_address + i;

                // for each four bytes we add a gap
                if address % 4 == 0 && i != 0 {
                    // with the gap being wider after 8 bytes and smaller after 4 bytes
                    ui.add_space(if address % 8 == 0 { space * 2.0 } else { space });
                }

                if self.rt.is_editing && self.state.selected_address == address {
                    self.display_value_editor(ui, source, address, on_write);
                }
                else {
                    self.display_value(ui, source, address, is_writable, on_read);
                }
            }
        });

        ui.end_row();
    }


    /// Checks if the address range of a memory line contains the start address of a category.
    /// If so, display the header of this category.
    fn display_category_header_at(&mut self, ui: &mut Ui, line_start_address: usize, line_end_address: usize) {
        for memory_area in &self.memory_areas {
            let memory_area_begin = *memory_area.memory_range.start();

            if memory_area_begin >= line_start_address && memory_area_begin <= line_end_address {
                ui.label(&memory_area.name);

                return;
            }
        }

        // if not on the first line of a category, just take the space to fill the cell
        ui.allocate_space(vec2(self.rt.column_width_category, 0.0));
    }


    /// Display the label with the current value of a memory cell.
    /// The label will be clickable if the cell is writable and then
    /// switch into editor mode when clicked.
    fn display_value(
            &mut self,
            ui: &mut Ui,
            source: &mut Source,
            address: usize,
            writable: bool,
            on_read: &Box<impl Fn(&Source, usize) -> Option<u8>>
    ) {
        let style = if writable {
            GemiStyle::VALUE_WRITABLE
        }
        else {
            GemiStyle::VALUE_READ_ONLY
        };

        // read the value from the memory source
        let value_str = match on_read(source, address) {
            Some(value) => format!("{:02x}", value),
            None => PLACEHOLDER_NO_VALUE.to_string()
        };

        let response = Label::new(style.rich_text(&value_str))
            .sense(Sense::click())
            .ui(ui)
        ;

        // when clicked, switch into edito mode
        if self.state.is_editable && response.clicked_by(PointerButton::Primary) {
            self.state.selected_address         = address;
            self.rt.is_editing                  = true;
            self.rt.edit_string                 = value_str;
            self.rt.edit_label_width            = response.rect.width();
            self.rt.edit_label_request_focus    = true;
        }
    }


    /// While in edit mode, this will handle the editor UI of the currently
    /// selected memory cell.
    fn display_value_editor(
            &mut self,
            ui: &mut Ui,
            source: &mut Source,
            address: usize,
            on_write: &mut Box<impl FnMut(&mut Source, usize, u8)>
    ) {
        let style = GemiStyle::VALUE_HIGHLIGHTED;

        // display the edit box
        let response = TextEdit::singleline(&mut self.rt.edit_string)
            .desired_width(self.rt.edit_label_width)
            .char_limit(2)
            .clip_text(false)
            .margin(Vec2::new(0.0, 0.0))
            .horizontal_align(egui::Align::Center)
            .frame(false)
            .font(style.style.clone())
            .text_color(style.color)
            .ui(ui)
        ;

        // request focus the first time after entering edit mode
        if self.rt.edit_label_request_focus {
            self.rt.edit_label_request_focus = false;
            response.request_focus();
        }

        // on change use the on_write callback to apply the value
        if response.changed() {
            if let Ok(value) = u8::from_str_radix(&self.rt.edit_string, 16) {
                on_write(source, address, value);
            }
        }

        // leave edit mode when out of focus
        if response.lost_focus() {
            self.rt.is_editing = false;
        }
    }


    /// Checks whether the UI parameters have changed and
    /// triggers a re-measurement if needed.
    fn measure_if_needed(&mut self, ui: &mut Ui) {
        // check if the available width has changed
        let available_width = ui.available_width();
        if self.rt.last_frame_width != available_width {
            self.rt.last_frame_width = available_width;
            self.rt.need_to_measure = true;
        }

        if self.rt.need_to_measure {
            self.measure(ui);
        }
    }


    /// Measures the UI layout parameters.
    fn measure(&mut self, ui: &mut Ui) {
        self.rt.line_content_height = ui.text_style_height(&TextStyle::Monospace);
        self.rt.line_distance_y     = self.rt.line_content_height + ui.spacing().item_spacing.y;
        let item_spacing            = ui.spacing().item_spacing.x;

        // measure the size of the largest column label
        self.rt.column_width_category = self.memory_areas.iter()
            .map(|area| {
                self.measure_text_width(ui, &area.name, TextStyle::Body).ceil() as i32
            })
            .max()
            .unwrap_or(10) as f32
        ;

        // measure the size of the address label
        self.rt.column_width_address = self.measure_text_width(
            ui,
            &("0".repeat(self.rt.number_of_address_characters)),
            GemiStyle::ADDRESS.style
        );

        // measure the size of a single memory cell (a single byte value)
        let memory_cell_width = self.measure_text_width(ui, "00", TextStyle::Monospace);

        // compute the space left
        let remaining_width =
                self.rt.last_frame_width
            -   self.rt.column_width_category
            -   self.rt.column_width_address
            -   item_spacing // left border
            -   item_spacing // spacing between category and address
            -   item_spacing // spacing between address and memory cells
        ;

        // compute the number of columns that fit into the remaining space
        self.rt.columns_per_line = 32;
        while self.rt.columns_per_line > 4 {
            // one gap per 4-byte group and one additional gap per 8-byte group
            let gaps =
                    max(1, self.rt.columns_per_line / 4) - 1
                +   max(1, self.rt.columns_per_line / 8) - 1
            ;

            // compute the total line length with all memory cells and gaps
            let line_width =
                    ((memory_cell_width + item_spacing) * self.rt.columns_per_line as f32)
                +   ((item_spacing) * (gaps as f32))
            ;

            // if the width fits, we are got the best fit
            if line_width <= remaining_width {
                break;
            }

            // reduce the size for the next iteration
            self.rt.columns_per_line /= 2;
        }

        self.rt.need_to_measure = false;
    }


    /// Measures the width of a text using the given style.
    fn measure_text_width(&self, ui: &mut Ui, text: &str, style: TextStyle) -> f32 {
        WidgetText::from(text)
            .into_galley(ui, Some(false), 0.0, style)
            .galley
            .rect.width()
    }
}


impl Default for MemoryEditorRuntimeData {
    fn default() -> Self {
        Self {
            need_to_measure:                true,
            last_frame_width:               0.0,
            line_content_height:            0.0,
            line_distance_y:                0.0,
            column_width_category:          0.0,
            column_width_address:           0.0,
            columns_per_line:               32,
            address_upper_bound:            0x00,
            number_of_address_characters:   4,
            is_editing:                     false,
            edit_string:                    String::new(),
            edit_label_width:               0.0,
            edit_label_request_focus:       false,
        }
    }
}
