use color_eyre::eyre::Result;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::mode::Mode;

use super::Component;

#[derive(Debug)]
pub struct ModeDisplay {
    mode: Mode,
}
impl ModeDisplay {
    pub fn new() -> Self {
        Self { mode: Mode::Normal }
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}
impl Component for ModeDisplay {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mode_paragraph =
            Paragraph::new(self.mode.to_string()).block(Block::default().borders(Borders::NONE));
        frame.render_widget(mode_paragraph, area);
        Ok(())
    }
}
