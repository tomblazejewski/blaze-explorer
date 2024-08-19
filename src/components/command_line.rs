use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tracing::info;

use crate::action::{Action, TextAction};

use super::Component;

pub struct CommandLine {
    command: String,
}

/// Struct used to process and display keys in command/search mode
impl CommandLine {
    pub fn new() -> Self {
        CommandLine {
            command: String::new(),
        }
    }

    pub fn append_char(&mut self, c: char) {
        self.command.push(c);
    }

    pub fn clear_command(&mut self) {
        self.command = String::new();
    }

    pub fn handle_text_action(&mut self, action: TextAction) -> Option<Action> {
        match action {
            TextAction::InsertKey(c) => self.append_char(c),
        }
        info!("Command is {:?}", self.command);
        None
    }

    pub fn unfocus(&mut self) {}

    pub fn focus(&mut self) {
        self.command = String::new();
    }
}

impl Component for CommandLine {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let text = Paragraph::new(self.command.clone()).block(Block::new().borders(Borders::ALL));
        frame.render_widget(text, area);
        Ok(())
    }
}
