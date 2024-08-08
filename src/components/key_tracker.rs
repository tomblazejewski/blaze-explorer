use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tracing::info;

use crate::action::Action;

use super::Component;

pub struct KeyTracker {
    keys_entered: Vec<KeyEvent>,
}

impl KeyTracker {
    pub fn new() -> Self {
        KeyTracker {
            keys_entered: Vec::new(),
        }
    }
    fn append_key(&mut self, key: KeyEvent) {
        info!("Appending {:?}", key);
        self.keys_entered.push(key)
    }
    fn clear_keys(&mut self) {
        info!("Clearing all keys");
        self.keys_entered = Vec::new();
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

impl Component for KeyTracker {
    fn get_area(&mut self, frame: &mut Frame) -> Result<Option<Rect>> {
        let main_box = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(85),
                Constraint::Percentage(5),
                Constraint::Percentage(10),
            ])
            .split(frame.size());
        let key_box = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(main_box[2]);
        Ok(Some(key_box[1]))
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Key(key) => {
                self.append_key(key);
            }
            Action::EscapeSequence => {
                self.clear_keys();
            }
            Action::ClearAndKey(key) => {
                self.clear_keys();
                self.append_key(key);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let key_chain = self.get_key_chain();
        let key_paragraph = Paragraph::new(key_chain).block(Block::new().borders(Borders::ALL));
        let area = self.get_area(frame).unwrap().unwrap();

        frame.render_widget(key_paragraph, area);
        Ok(())
    }
}
