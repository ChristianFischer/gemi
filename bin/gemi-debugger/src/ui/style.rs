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

use egui::{Color32, RichText, TextBuffer, TextEdit, TextStyle};

/// A collection of style values defining the application theme.
pub struct GemiStyle;


/// A struct containing several parameters for text formatting.
pub struct TextFormatting {
    /// The text style to be used.
    pub style: TextStyle,

    /// The color of the text.
    pub color: Option<Color32>,

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
        ;

        if let Some(color) = &self.color {
            rt = rt.color(*color);
        }

        if self.bold {
            rt = rt.strong();
        }

        if self.italic {
            rt = rt.italics();
        }

        rt
    }


    /// Creates a single line edit box using the current style.
    pub fn text_edit_singleline<'t>(&self, text: &'t mut dyn TextBuffer) -> TextEdit<'t> {
        let mut t = TextEdit::singleline(text)
                .font(self.style.clone())
        ;

        if let Some(color) = &self.color {
            t = t.text_color(*color);
        }

        t
    }
}


impl Default for TextFormatting {
    fn default() -> Self {
        Self {
            style: TextStyle::Body,
            color: None,
            bold: false,
            italic: false,
        }
    }
}


impl GemiStyle {
    /// The background color of highlighted areas.
    pub const BACKGROUND_HIGHLIGHT_SELECTION: Color32 = Color32::from_rgb(0x00, 0x7e, 0xe1);
    pub const BACKGROUND_HIGHLIGHT_HOVER:     Color32 = Color32::from_rgb(0x7e, 0xb5, 0xe1);
    
    /// A slightly stronger text for caption lines.
    pub const CAPTION: TextFormatting = TextFormatting {
        style: TextStyle::Body,
        color: None,
        bold: true,
        italic: false,
    };

    /// A text style for normal monospace text.
    pub const MONOSPACE: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: None,
        bold: false,
        italic: false,
    };

    /// Text formatting to display addresses.
    pub const ADDRESS: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Some(Color32::from_rgb(0x98, 0x76, 0xaa)),
        bold: false,
        italic: false,
    };

    /// Text formatting to display a selected and highlighted address.
    pub const ADDRESS_SELECTED: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Some(Color32::from_rgb(0xc1, 0x96, 0xd8)),
        bold: true,
        italic: false,
    };

    /// Text formatting to display keywords.
    pub const KEYWORD: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Some(Color32::from_rgb(0xcc, 0x78, 0x32)),
        bold: true,
        italic: false,
    };

    /// Text formatting to display keywords.
    pub const KEYWORD_LOW: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Some(Color32::from_rgb(0xcc, 0x85, 0x4a)),
        bold: false,
        italic: false,
    };

    /// Text formatting to display a read-only numeric value.
    pub const VALUE_READ_ONLY: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Some(Color32::from_rgb(0x7a, 0x89, 0x95)),
        bold: false,
        italic: false,
    };

    /// Text formatting to display a writable numeric value.
    pub const VALUE_WRITABLE: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Some(Color32::from_rgb(0x68, 0x97, 0xbb)),
        bold: false,
        italic: false,
    };

    /// Text formatting to display a highlighted, being edited numeric value.
    pub const VALUE_HIGHLIGHTED: TextFormatting = TextFormatting {
        style: TextStyle::Monospace,
        color: Some(Color32::from_rgb(0x89, 0xc6, 0xf6)),
        bold: false,
        italic: false,
    };
}
