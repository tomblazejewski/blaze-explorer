use ratatui::style::{Color, Style};

pub struct CustomTheme {
    pub search_result: Style,
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            search_result: Style::default().bg(Color::Rgb(114, 135, 253)), //catpuccin lavender
        }
    }
}
