use color_eyre::eyre::Result;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{action_agent::ActionAgent, mode::Mode};

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
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::eyre::Result<()> {
        let mode_paragraph =
            Paragraph::new(self.mode.to_string()).block(Block::default().borders(Borders::NONE));
        frame.render_widget(mode_paragraph, area);
        Ok(())
    }
}
impl ActionAgent for ModeDisplay {
    fn update(&mut self, action: crate::action::Action) -> Result<Option<crate::action::Action>> {
        match action {
            crate::action::Action::AppAct(crate::action::AppAction::SwitchMode(mode)) => {
                self.switch_mode(mode);
            }
            _ => {}
        }
        Ok(None)
    }
}
