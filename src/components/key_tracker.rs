use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tracing::info;

use crate::{action::Action, action_agent::ActionAgent};

use super::Component;
use crate::key_handler::KeyHandler;

pub struct KeyTracker {
    keys_entered: Vec<KeyEvent>,
}

impl KeyTracker {
    pub fn new() -> Self {
        KeyTracker {
            keys_entered: Vec::new(),
        }
    }
    pub fn get_key_chain(&self) -> String {
        let key_chain = self
            .keys_entered
            .iter()
            .map(|key_event| match key_event.code {
                KeyCode::Char(char_) => char_,
                _ => '!',
            })
            .collect::<String>();
        key_chain
    }
}

impl KeyHandler for KeyTracker {
    fn append_key_event(&mut self, new_event: KeyEvent) {
        self.keys_entered.push(new_event);
    }

    fn clear_key_events(&mut self) {
        self.keys_entered.clear();
    }
}
impl Component for KeyTracker {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let key_chain = self.get_key_chain();
        let key_paragraph = Paragraph::new(key_chain).block(Block::new().borders(Borders::ALL));

        frame.render_widget(key_paragraph, area);
        Ok(())
    }
}
impl ActionAgent for KeyTracker {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            _ => {}
        }
        Ok(None)
    }
}
