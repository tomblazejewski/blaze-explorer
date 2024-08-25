use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tracing::info;

use crate::{
    action::{Action, AppAction, ExplorerAction, TextAction},
    mode::Mode,
};

use super::Component;

pub struct CommandLine {
    command: String,
    focused: bool,
}

/// Struct used to process and display keys in command/search mode
impl CommandLine {
    pub fn new() -> Self {
        CommandLine {
            command: String::new(),
            focused: false,
        }
    }

    pub fn pop_command(&mut self) -> String {
        self.command.drain(..).collect()
    }

    pub fn append_char(&mut self, c: char) {
        self.command.push(c);
    }

    pub fn clear_command(&mut self) {
        self.command = String::new();
    }

    pub fn remove_char(&mut self) -> Option<Action> {
        if self.command.is_empty() {
            Some(Action::AppAct(AppAction::SwitchMode(Mode::Normal)))
        } else {
            self.command.pop();
            Some(Action::ExplorerAct(ExplorerAction::UpdateSearchQuery(
                self.command.clone(),
            )))
        }
    }

    pub fn handle_text_action(&mut self, action: TextAction) -> Option<Action> {
        match action {
            TextAction::InsertKey(c) => self.append_char(c),
            TextAction::EraseText => self.clear_command(),
            TextAction::DropKey => return self.remove_char(),
        }
        Some(Action::ExplorerAct(ExplorerAction::UpdateSearchQuery(
            self.command.clone(),
        )))
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn focus(&mut self) {
        self.focused = true;
        self.command = String::new();
    }
}

impl Component for CommandLine {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let text = Paragraph::new(self.command.clone()).block(Block::new());
        frame.render_widget(text, area);
        match self.focused {
            true => frame.set_cursor(area.x + self.command.len() as u16, area.y + 1),
            false => {}
        }
        Ok(())
    }
}
