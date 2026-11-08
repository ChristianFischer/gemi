/*
 * Copyright (C) 2026 by Christian Fischer
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

use crate::state::EmulatorState;
use crate::views::View;
use egui::Ui;
use egui_extras::Column;
use libgemi::core::cpu::interrupts::Interrupt;
use libgemi::debug::breakpoints::{AddressBreakpoint, Breakpoint, InterruptBreakpoint, PpuLineBreakpoint};
use render_breakpoint_table::{display_editable_value, find_next_free, RenderBreakpointTable, ValueFormat};


#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakpointsView {
}


impl BreakpointsView {
    pub fn new() -> Self {
        Self {
        }
    }
}


impl View for BreakpointsView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Breakpoints"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        ui.vertical(|ui| {
            let mut was_modified = false;

            was_modified |= self.render_instruction_breakpoints(ui, state);
            was_modified |= self.render_memory_breakpoints(ui, state);
            was_modified |= self.render_interrupt_breakpoints(ui, state);
            was_modified |= self.render_display_line_breakpoints(ui, state);
            // skip frame breakpoints here, as the debugger already has a frame step mode

            if was_modified {
                state.breakpoints.set_is_modified()
            }
        });
    }
}


impl BreakpointsView {
    fn render_instruction_breakpoints(&mut self, ui: &mut Ui, state: &mut EmulatorState) -> bool {
        RenderBreakpointTable::with_breakpoints(ui, &mut state.emu, &mut state.breakpoints.get_breakpoints_table_mut_no_modify().instruction_breakpoints)
            .show_header_with_add(
                "Instruction Breakpoints",
                |_, breakpoints| find_next_free(breakpoints, (0x0000..=0xffff).map(AddressBreakpoint))
            )
            .show_body(
                Breakpoint::InstructionHit,
                |table_builder| table_builder.column(Column::auto_with_initial_suggestion(80.0)),
                |breakpoints, breakpoint, row| {
                    let mut was_modified = false;

                    row.col(|ui| {
                        let mut address = breakpoint.0;
                        let changed = display_editable_value(ui, &mut address, ValueFormat::Hex);

                        if changed {
                            let new_breakpoint = AddressBreakpoint(address);
                            breakpoints.move_breakpoint(breakpoint, new_breakpoint);
                            was_modified = true;
                        }
                    });

                    was_modified
                }
            )
    }


    fn render_memory_breakpoints(&mut self, ui: &mut Ui, state: &mut EmulatorState) -> bool {
        RenderBreakpointTable::with_breakpoints(ui, &mut state.emu, &mut state.breakpoints.get_breakpoints_table_mut_no_modify().memory_access_breakpoints)
            .show_header_with_add(
                "Memory Access Breakpoints",
                    |_, breakpoints| find_next_free(breakpoints, (0x0000..=0xffff).map(AddressBreakpoint))
            )
            .show_body(
                Breakpoint::MemoryWritten,
                |table_builder| table_builder.column(Column::auto_with_initial_suggestion(80.0)),
                |breakpoints, breakpoint, row| {
                    let mut was_modified = false;

                    row.col(|ui| {
                        let mut address = breakpoint.0;
                        let changed = display_editable_value(ui, &mut address, ValueFormat::Hex);

                        if changed {
                            let new_breakpoint = AddressBreakpoint(address);
                            breakpoints.move_breakpoint(breakpoint, new_breakpoint);
                            was_modified = true;
                        }
                    });

                    was_modified
                }
            )
    }


    fn render_interrupt_breakpoints(&mut self, ui: &mut Ui, state: &mut EmulatorState) -> bool {
        RenderBreakpointTable::with_breakpoints(ui, &mut state.emu, &mut state.breakpoints.get_breakpoints_table_mut_no_modify().interrupt_breakpoints)
            .show_header_with_add(
                "Interrupt Breakpoints",
                |_, breakpoints| find_next_free(breakpoints, Interrupt::ALL_INTERRUPTS.map(InterruptBreakpoint))
            )
            .show_body(
                Breakpoint::InterruptTriggered,
                |table_builder| table_builder
                    .column(Column::initial(80.0).resizable(true))
                    .column(Column::auto()),
                |_, breakpoint, row| {
                    let interrupt = breakpoint.0;

                    row.col(|ui| { ui.label(interrupt.name().to_string()); });
                    row.col(|ui| { ui.monospace(format!("{:04x}", interrupt.address())); });

                    false
                }
            )
    }


    fn render_display_line_breakpoints(&mut self, ui: &mut Ui, state: &mut EmulatorState) -> bool {
        RenderBreakpointTable::with_breakpoints(ui, &mut state.emu, &mut state.breakpoints.get_breakpoints_table_mut_no_modify().ppu_line_breakpoints)
            .show_header_with_add(
                "Display Line Breakpoints",
                |_, breakpoints| find_next_free(breakpoints, (0..154).map(PpuLineBreakpoint))
            )
            .show_body(
                Breakpoint::PpuLineFinished,
                |table_builder| table_builder.column(Column::auto_with_initial_suggestion(80.0)),
                |breakpoints, breakpoint, row| {
                    let mut was_modified = false;

                    row.col(|ui| {
                        let mut line = breakpoint.0;
                        let changed = display_editable_value(ui, &mut line, ValueFormat::Dec);

                        if changed {
                            let new_breakpoint = PpuLineBreakpoint(line);
                            breakpoints.move_breakpoint(breakpoint, new_breakpoint);
                            was_modified = true;
                        }
                    });

                    was_modified
                }
            )
    }
}


mod render_breakpoint_table {
    use super::*;
    use crate::state::EmulatorInstance;
    use crate::strings::{BUTTON_LABEL_ADD, BUTTON_LABEL_DELETE, MARKER_BREAKPOINT_FIRED};
    use egui::collapsing_header::CollapsingState;
    use egui::{Align, Color32, Layout, TextStyle};
    use egui_extras::{TableBuilder, TableRow};
    use libgemi::core::debug::DebugListener;
    use libgemi::debug::breakpoint::{BreakpointCondition, BreakpointType};
    use libgemi::debug::breakpoint_set::BreakpointSet;
    use libgemi::debug::breakpoints::Breakpoint;
    use std::fmt::{LowerHex, UpperHex};


    /// Format for displaying and editing numeric values.
    #[derive(Copy, Clone)]
    pub(super) enum ValueFormat {
        /// Display and edit as hexadecimal with leading zeros.
        Hex,

        /// Display and edit as decimal.
        Dec,
    }


    pub(super) struct RenderBreakpointTable<'a, 'ui, B, C>
    where
        B: BreakpointType,
        C: BreakpointCondition,
    {
        ui: &'ui mut Ui,
        emulator_instance: &'a mut EmulatorInstance,
        breakpoints: &'a mut BreakpointSet<B, C>,
    }


    pub(super) struct RenderBreakpointTableHeaderState<'a, 'ui, B, C>
    where
        B: BreakpointType,
        C: BreakpointCondition,
    {
        breakpoints: &'a mut BreakpointSet<B, C>,
        emulator_instance: &'a mut EmulatorInstance,
        collapsing_header_state: egui::collapsing_header::HeaderResponse<'ui, bool>,
    }


    impl<'a, 'ui, B, C> RenderBreakpointTable<'a, 'ui, B, C>
    where
        B: BreakpointType,
        C: BreakpointCondition,
    {
        pub(super) fn with_breakpoints(ui: &'ui mut Ui, emulator_instance: &'a mut EmulatorInstance, breakpoints: &'a mut BreakpointSet<B, C>) -> Self {
            Self {
                ui,
                emulator_instance,
                breakpoints,
            }
        }


        #[allow(unused)] // currently not in use, but kept for reference
        pub(super) fn show_header(self, title: &'a str) -> RenderBreakpointTableHeaderState<'a, 'ui, B, C> {
            self.show_header_impl(title, None::<fn(&mut Ui, &mut BreakpointSet<B, C>) -> Option<B>>)
        }


        pub(super) fn show_header_with_add(self, title: &'a str, fn_add_new_bp: impl FnOnce(&mut Ui, &mut BreakpointSet<B, C>) -> Option<B>) -> RenderBreakpointTableHeaderState<'a, 'ui, B, C> {
            self.show_header_impl(title, Some(fn_add_new_bp))
        }


        fn show_header_impl(self, title: &'a str, fn_add_new_bp: Option<impl FnOnce(&mut Ui, &mut BreakpointSet<B, C>) -> Option<B>>) -> RenderBreakpointTableHeaderState<'a, 'ui, B, C> {
            let Self { ui, emulator_instance, breakpoints } = self;

            let ctx = ui.ctx();
            let id = ui.make_persistent_id(title);
            let state = ctx.data_mut(|d| d.get_persisted::<bool>(id)).unwrap_or(false);

            let collapsing_header_state = CollapsingState::load_with_default_open(ui.ctx(), id, state)
                .show_header(ui, |ui| {
                    let count = breakpoints.get_breakpoints().count();

                    let response = ui.horizontal(|ui| {
                        let mut was_modified = false;

                        // title bar with the number of entries
                        ui.label(title);
                        ui.monospace(format!("({})", count));

                        // "add" button for adding new breakpoints of this category
                        if let Some(fn_add_new_bp) = fn_add_new_bp {
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if ui.small_button(BUTTON_LABEL_ADD).clicked() {
                                    if let Some(new_breakpoint) = fn_add_new_bp(ui, breakpoints) {
                                        breakpoints.add_breakpoint(new_breakpoint);
                                        was_modified = true;
                                    }
                                }
                            });
                        }

                        was_modified
                    });

                    response.inner
                });

            RenderBreakpointTableHeaderState {
                breakpoints,
                emulator_instance,
                collapsing_header_state,
            }
        }
    }


    impl<'a, 'ui, B, C> RenderBreakpointTableHeaderState<'a, 'ui, B, C>
    where
        B: BreakpointType,
        C: BreakpointCondition,
    {
        pub(super) fn show_body(
            self,
            fn_into_enum: impl Fn(B) -> Breakpoint,
            fn_build_table: impl FnOnce(TableBuilder) -> TableBuilder,
            fn_show_row: impl Fn(&mut BreakpointSet<B, C>, &B, &mut TableRow) -> bool
        ) -> bool {
            let response = self.collapsing_header_state.body(|ui| {
                let line_height = ui.text_style_height(&TextStyle::Monospace);
                let mut was_modified = false;

                let table_builder = TableBuilder::new(ui)
                    .column(Column::initial(16.0).resizable(false))     // Event marker
                    .column(Column::auto())                             // Enabled Checkbox
                ;

                let table_builder = fn_build_table(table_builder)
                    .column(Column::remainder())
                    .column(Column::auto())                             // Delete Button
                    .vscroll(true)
                    .striped(true)
                ;

                table_builder.body(|mut body| {
                    let mut all_breakpoints = self.breakpoints
                        .get_breakpoints()
                        .cloned()
                        .collect::<Vec<_>>();

                    all_breakpoints.sort();

                    for breakpoint in all_breakpoints {
                        body.row(line_height, |mut row| {
                            // event marker column
                            row.col(|ui| {
                                if let Some(gb) = self.emulator_instance.get_gameboy() {
                                    if gb.get_debugger().is_halted() {
                                        let breakpoint = fn_into_enum(breakpoint.clone());
                                        if gb.get_debugger().is_halted_by(breakpoint) {
                                            ui.colored_label(Color32::YELLOW, MARKER_BREAKPOINT_FIRED);
                                        }
                                    }
                                }
                            });

                            // "is enabled" checkbox column
                            row.col(|ui| {
                                let is_enabled = self.breakpoints.get_breakpoint_attr(&breakpoint)
                                    .map(|attr| attr.enabled)
                                    .unwrap_or(false);

                                let mut is_enabled_checkbox_result = is_enabled;
                                if ui.checkbox(&mut is_enabled_checkbox_result, "").changed() {
                                    if let Some(attr) = self.breakpoints.get_breakpoint_attr_mut(&breakpoint) {
                                        attr.enabled = is_enabled_checkbox_result;
                                        was_modified = true;
                                    }
                                }
                            });

                            // present custom columns
                            was_modified |= fn_show_row(self.breakpoints, &breakpoint, &mut row);

                            // filler column
                            row.col(|_| ());

                            // delete button
                            row.col(|ui| {
                                if ui.small_button(BUTTON_LABEL_DELETE).clicked() {
                                    self.breakpoints.remove_breakpoint(&breakpoint);
                                    was_modified = true;
                                }
                            });
                        });
                    }
                });

                was_modified
            });

            let (_, header_response, body_response) = response;
            let was_modified_by_header = header_response.inner;
            let was_modified_by_body   = body_response.map(|response| response.inner).unwrap_or(false);

            was_modified_by_body || was_modified_by_header
        }
    }


    pub(super) fn find_next_free<B, C>(breakpoints: &BreakpointSet<B, C>, candidates: impl IntoIterator<Item=B>) -> Option<B>
    where B: BreakpointType,
          C: BreakpointCondition
    {
        candidates.into_iter().find(|candidate| !breakpoints.has_breakpoint(candidate))
    }


    /// Displays a value that can be edited by clicking.
    ///
    /// - Clicking the displayed value toggles between display and edit mode.
    /// - `format` determines whether the value is shown as hex or decimal.
    /// - Hex values are displayed with leading zeros.
    /// - Decimal values are displayed and entered as plain integers.
    pub(super) fn display_editable_value<V>(ui: &mut Ui, value: &mut V, format: ValueFormat) -> bool
    where V: Sized + Copy + PartialEq + LowerHex + UpperHex + TryFrom<u32> + std::fmt::Display
    {
        let edit_mode_id    = ui.scope_id().with("edit_mode");
        let edit_content_id = ui.scope_id().with("edit_content");

        // read the edit mode flag
        let is_editing = ui.ctx()
            .data_mut(|d| d.get_persisted::<bool>(edit_mode_id))
            .unwrap_or(false)
        ;

        let formatted_string = match format {
            ValueFormat::Hex => {
                let digits = size_of::<V>() * 2; // 2 digits per byte
                format!("{:0width$x}", value, width = digits)
            }

            ValueFormat::Dec => format!("{value}")
        };

        let radix = match format {
            ValueFormat::Hex => 16,
            ValueFormat::Dec => 10,
        };

        if is_editing {
            // Edit mode: show a text input field

            // get the current value being edited
            let mut edit_text = ui.ctx()
                .data_mut(|d| d.get_persisted::<String>(edit_content_id))
                .unwrap_or(formatted_string)
            ;

            let response = ui.add(egui::TextEdit::singleline(&mut edit_text)
                .code_editor()
                .desired_width(f32::INFINITY)
                .id(edit_mode_id.with("editor"))
            );

            // on change, update the stored field
            if response.changed() {
                ui.ctx().data_mut(|d| d.insert_persisted(edit_content_id, edit_text.clone()));
            }

            // Commit on Enter or Lost Focus, or Escape to cancel
            let lost_focus = response.lost_focus();
            let (enter_pressed, escape_pressed) = ui.input(|i| {
                (i.key_pressed(egui::Key::Enter), i.key_pressed(egui::Key::Escape))
            });

            let mut value_did_change = false;
            let is_still_editing = {
                if enter_pressed || lost_focus {
                    let parsed_result = u32::from_str_radix(&edit_text, radix).ok()
                        .and_then(|v| V::try_from(v).ok())
                    ;

                    if let Some(parsed) = parsed_result {
                        if parsed != *value {
                            *value = parsed;
                            value_did_change = true;
                        }
                    }

                    false
                }
                else if escape_pressed {
                    false
                }
                else {
                    is_editing
                }
            };

            // only have to update the flag once it get disabled
            if !is_still_editing {
                ui.ctx().data_mut(|d| {
                    d.insert_persisted(edit_mode_id, false);
                    d.remove::<String>(edit_content_id);
                });
            }

            // unless stopped editing, keep focus on the editor
            if is_still_editing {
                response.request_focus();
            }

            value_did_change
        }
        else {
            // Display mode: show the value as a clickable label
            let response = ui.monospace(&formatted_string);

            if response.clicked() {
                // Update persisted state and store the current string for editing
                ui.ctx().data_mut(|d| {
                    d.insert_persisted(edit_mode_id,    true);
                    d.insert_persisted(edit_content_id, formatted_string);
                });
            }

            false
        }
    }
}
