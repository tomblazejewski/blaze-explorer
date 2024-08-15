use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tracing::info;
use tui_textbox::{Textbox, TextboxState};

use crate::action::{Action, KeyAction};

use super::Component;

pub struct CommandLine {
    command: String,
    state: TextboxState,
}

impl CommandLine {
    pub fn new() -> Self {
        CommandLine {
            command: String::new(),
            state: TextboxState::default(),
        }
    }
}

impl Component for CommandLine {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::KeyAct(KeyAction::Key(key)) => {
                self.state.handle_events(key.code, key.modifiers);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let textbox = Textbox::default();

        frame.render_stateful_widget(textbox, area, &mut self.state);
        Ok(())
    }
}