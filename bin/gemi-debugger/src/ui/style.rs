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

use egui::{Color32, RichText, TextStyle};


/// A collection of style values defining the application theme.
pub struct GemiStyle;


/// A struct containing several parameters for text formatting.
pub struct TextFormatting {
    /// The text style to be used.
    pub style: TextStyle,

    /// The color of the text.
    pub color: Color32,

    /// Whether the text should be bold.
    pub bold: bool,

    /// Whether the text should be italic.
    pub italic: bool,
}


impl TextFormatting {
    /// Creates a rich text element based on the text formatting
    /// settings of this configuration.
    pub fn rich_text(&self, s: impl Into<String>) -> RichText {
        let mut rt = RichText::new(s)
            .text_style(self.style.clone())
            .color(self.color.clone())
        ;

        if self.bold {
            rt = rt.strong();
        }

        if self.italic {
            rt = rt.italics();
        }

        rt
    }
}


impl Default for TextFormatting {
    fn default() -> Self {
        Self {
            style: TextStyle::Body,
            color: Color32::WHITE,
            bold: false,
            italic: false,
        }
    }
}


impl GemiStyle {
    /// Text formatting to display addresses.
    pub const ADDRESS: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Color32::from_rgb(0x98, 0x76, 0xaa),
        bold: false,
        italic: false,
    };

    /// Text formatting to display a selected and highlighted address.
    pub const ADDRESS_SELECTED: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Color32::from_rgb(0xc1, 0x96, 0xd8),
        bold: true,
        italic: false,
    };

    /// Text formatting to display a read-only numeric value.
    pub const VALUE_READ_ONLY: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Color32::from_rgb(0x99, 0xac, 0x7a),
        bold: false,
        italic: false,
    };

    /// Text formatting to display a writable numeric value.
    pub const VALUE_WRITABLE: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Color32::from_rgb(0x68, 0x97, 0xbb),
        bold: false,
        italic: false,
    };

    /// Text formatting to display a highlighted, being edited numeric value.
    pub const VALUE_HIGHLIGHTED: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Color32::from_rgb(0x89, 0xc6, 0xf6),
        bold: false,
        italic: false,
    };
}
