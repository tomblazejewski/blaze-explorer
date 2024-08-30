use ratatui::style::{Color, Style};

pub struct CustomTheme {
    pub search_result: Style,
    pub selected_row: Style,
    pub selected_row_telescope: Style,
    pub header: Style,
    pub focused_border: Style,
    pub unfocused_border: Style,
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            search_result: Style::default().bg(Color::Rgb(114, 135, 253)), //catpuccin lavender
            selected_row: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(255, 255, 255)),
            selected_row_telescope: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(255, 255, 255)),
            header: Style::default()
                .bg(Color::Rgb(255, 255, 255))
                .fg(Color::Rgb(0, 0, 0)),
            focused_border: Style::new().fg(Color::Rgb(255, 255, 255)),
            unfocused_border: Style::new().fg(Color::Rgb(49, 50, 68)),
        }
    }
}
