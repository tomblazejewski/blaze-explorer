use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tracing::info;
use tui_textbox::{Textbox, TextboxState};

use crate::{
    action::{Action, TextAction},
    action_agent::ActionAgent,
    key_handler::KeyHandler,
};

use super::Component;

pub struct CommandLine {
    command: String,
    state: TextboxState,
}

/// Struct used to process and display keys in command/search mode
impl CommandLine {
    pub fn new() -> Self {
        CommandLine {
            command: String::new(),
            state: TextboxState::default(),
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
            TextAction::InsertKey(c) => match c.code {
                KeyCode::Char(c) => self.append_char(c),
                _ => {}
            },
        }
        None
    }
}

impl KeyHandler for CommandLine {
    fn append_key_event(&mut self, new_event: KeyEvent) {
        self.state
            .handle_events(new_event.code, new_event.modifiers);
    }
    fn clear_key_events(&mut self) {
        self.state = TextboxState::default();
    }
}
impl Component for CommandLine {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let textbox = Textbox::default();

        frame.render_stateful_widget(textbox, area, &mut self.state);
        Ok(())
    }
}
