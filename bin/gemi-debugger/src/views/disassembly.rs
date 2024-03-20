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

use egui::{Grid, Ui};

use gemi_core::cpu::opcode::{Instruction, Token};
use gemi_core::gameboy::GameBoy;

use crate::state::EmulatorState;
use crate::ui::style::GemiStyle;
use crate::views::View;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DisassemblyView {

}


impl View for DisassemblyView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Disassembly"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        if let Some(emu) = state.emu.get_emulator() {
            let mut remaining_height = ui.available_height();
            let mut pc               = emu.cpu.get_instruction_pointer();

            // accessor to read from emulator memory
            let read_emu = |address| emu.get_mmu().read_u8(address);

            Grid::new("grid")
                    .num_columns(3)
                    .show(ui, |ui| {
                        loop {
                            let instruction = Instruction::read_instruction(pc, read_emu);
                            pc += instruction.get_instruction_length();

                            self.render_instruction_row(ui, instruction, emu, false);

                            remaining_height -= ui.available_height();
                            remaining_height -= ui.spacing().item_spacing.y;
                            if remaining_height < 0.0 {
                                break;
                            }
                        }
                    })
            ;
        }
    }
}


impl DisassemblyView {
    pub fn new() -> Self {
        Self {
        }
    }

    /// Renders a single instruction into a row
    fn render_instruction_row(&mut self, ui: &mut Ui, instruction: Instruction, emu: &GameBoy, selected: bool) {
        // address
        {
            let address_str = format!("{:04x}", instruction.opcode_address);

            if selected {
                let address_text = GemiStyle::ADDRESS_SELECTED.rich_text(address_str);
                ui.label(address_text);
            }
            else {
                let address_text = GemiStyle::ADDRESS.rich_text(address_str);
                ui.label(address_text);
            };
        }

        // instruction bytes
        {
            // number of bytes for this instruction (+1 for 0xcb opcodes)
            let num_instruction_bytes = instruction.get_instruction_length();

            let bytes_string =
                    (0..num_instruction_bytes)
                    .into_iter()
                    .map(|offset| instruction.opcode_address.wrapping_add(offset))
                    .map(|address| emu.get_mmu().read_u8(address))
                    .fold(
                        String::new(),
                        |str, byte| format!("{str}{byte:02x} ")
                    )
            ;

            let bytes_text = GemiStyle::VALUE_READ_ONLY.rich_text(bytes_string);
            ui.label(bytes_text);
        }

        // format the opcode label
        {
            ui.horizontal(|ui|{
                for token in instruction.opcode.tokenize() {
                    match token {
                        Token::Command(cmd) => {
                            ui.label(GemiStyle::KEYWORD.rich_text(cmd));
                        }

                        Token::Text(t) => {
                            ui.label(GemiStyle::KEYWORD_LOW.rich_text(t));
                        }

                        Token::Argument(arg) => {
                            let str = instruction.resolve_argument(&arg);
                            ui.label(GemiStyle::KEYWORD_LOW.rich_text(str));
                        }
                    }

                    // reduce item spacing for each following item
                    ui.style_mut().spacing.item_spacing.x = 0.0;
                }
            });
        }

        ui.end_row();
    }
}
