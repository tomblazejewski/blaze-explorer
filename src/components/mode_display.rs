use color_eyre::eyre::Result;
use ratatui::{layout::Rect, Frame};

use crate::mode::Mode;

use super::Component;

struct ModeDisplay {
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
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::eyre::Result<()> {
        todo!()
    }
}
