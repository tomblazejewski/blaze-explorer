use ratatui::style::{Color, Style, palette::tailwind};

#[derive(Clone, Debug, PartialEq)]
pub struct CustomTheme {
    pub row: Style,
    pub search_result: Style,
    pub selected_row: Style,
    pub selected_row_telescope: Style,
    pub header: Style,
    pub focused_border: Style,
    pub unfocused_border: Style,
    pub highlight_query: Style,
    pub highlight_jump_char: Style,
    pub marked_row: Style,
    pub marked_selected_row: Style,
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            row: Style::new().bg(tailwind::BLACK).fg(tailwind::WHITE),
            search_result: Style::default().bg(Color::Rgb(114, 135, 253)), //catpuccin lavender
            selected_row: Style::default().bg(Color::Rgb(49, 50, 68)),
            selected_row_telescope: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(255, 255, 255)),
            header: Style::default()
                .bg(Color::Rgb(255, 255, 255))
                .fg(Color::Rgb(0, 0, 0)),
            focused_border: Style::new().fg(Color::Rgb(255, 255, 255)),
            unfocused_border: Style::new().fg(Color::Rgb(49, 50, 68)),
            highlight_query: Style::new().fg(Color::Rgb(128, 0, 128)),
            highlight_jump_char: Style::new().fg(Color::Rgb(255, 255, 0)),
            marked_row: Style::new().bg(Color::Rgb(100, 149, 237)),
            marked_selected_row: Style::new().bg(Color::Rgb(106, 90, 205)),
        }
    }
}
