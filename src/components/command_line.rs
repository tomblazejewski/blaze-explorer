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
    line_entry::LineEntry,
    mode::Mode,
};

use super::Component;

pub struct CommandLine {
    contents: String,
    focused: bool,
}

impl LineEntry for CommandLine {
    fn pop_contents(&mut self) -> String {
        self.contents.drain(..).collect()
    }

    fn append_char(&mut self, c: char) {
        self.contents.push(c);
    }

    fn clear_contents(&mut self) {
        self.contents = String::new();
    }

    fn drop_char(&mut self) {
        self.contents.pop();
    }

    fn remove_char(&mut self) -> Option<Action> {
        if self.contents.is_empty() {
            Some(Action::AppAct(AppAction::SwitchMode(Mode::Normal)))
        } else {
            self.drop_char();
            Some(Action::ExplorerAct(ExplorerAction::UpdateSearchQuery(
                self.contents.clone(),
            )))
        }
    }
}

/// Struct used to process and display keys in command/search mode
impl CommandLine {
    pub fn new() -> Self {
        CommandLine {
            contents: String::new(),
            focused: false,
        }
    }

    pub fn handle_text_action(&mut self, action: TextAction) -> Option<Action> {
        match action {
            TextAction::InsertKey(c) => self.append_char(c),
            TextAction::EraseText => self.clear_contents(),
            TextAction::DropKey => return self.remove_char(),
        }
        Some(Action::ExplorerAct(ExplorerAction::UpdateSearchQuery(
            self.contents.clone(),
        )))
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn focus(&mut self) {
        self.focused = true;
        self.contents = String::new();
    }
}

impl Component for CommandLine {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let text = Paragraph::new(self.contents.clone()).block(Block::new());
        frame.render_widget(text, area);
        match self.focused {
            true => frame.set_cursor(area.x + self.contents.len() as u16, area.y + 1),
            false => {}
        }
        Ok(())
    }
}
