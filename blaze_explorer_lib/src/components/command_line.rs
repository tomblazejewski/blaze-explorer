use color_eyre::eyre::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph},
    Frame,
};
use std::cmp::min;

use crate::{
    action::{Action, AppAction, ExplorerAction, TextAction},
    line_entry::LineEntry,
    mode::Mode,
};

use super::Component;

#[derive(Clone, Debug, PartialEq)]
pub struct CommandLine {
    contents: String,
    focused: bool,
    message_queue: Option<Vec<String>>,
    pub current_message: Option<Vec<String>>,
    line_limit: u8,
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
    fn get_contents(&self) -> String {
        self.contents.clone()
    }

    fn set_contents(&mut self, contents: String) {
        self.contents = contents;
    }
}

/// Struct used to process and display keys in command/search mode
impl CommandLine {
    pub fn new() -> Self {
        CommandLine {
            contents: String::new(),
            focused: false,
            message_queue: None,
            current_message: None,
            line_limit: 30,
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

    /// Clear contents of the command line and save a message to a message_queue field, so that
    /// they can be displayed later on
    pub fn command_line_message(&mut self, msg: String) {
        self.clear_contents();
        let lines = msg.lines().map(|f| f.to_string()).collect::<Vec<String>>();
        self.message_queue = Some(lines);
        self.get_message_batch();
    }

    /// get a batch consisting of line_limit and pop it off the message. If the remaining message
    /// is empty, make it None
    pub fn get_message_batch(&mut self) {
        let current_message = self.message_queue.clone();
        match current_message {
            None => self.current_message = None,
            Some(mut msg) => {
                let limit = min(self.line_limit as usize, msg.len());
                let batch = msg.drain(0..limit as usize).collect::<Vec<String>>();
                if msg.is_empty() {
                    self.message_queue = None;
                } else {
                    self.message_queue = Some(msg);
                }

                self.current_message = Some(batch);
            }
        }
    }
}

impl Default for CommandLine {
    fn default() -> Self {
        Self::new()
    }
}
impl Component for CommandLine {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut actual_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
            .split(area)[1];

        match &self.current_message {
            None => {
                //reduce the area to just one line
                let text = Paragraph::new(self.contents.clone()).block(Block::new());
                frame.render_widget(Clear, actual_area);
                frame.render_widget(text, actual_area);
                if self.focused {
                    frame.set_cursor(
                        actual_area.x + self.contents.len() as u16,
                        actual_area.y + 1,
                    )
                }
            }
            Some(contents) => {
                actual_area = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Fill(1),
                        Constraint::Length(contents.len() as u16),
                    ])
                    .split(area)[1];
                let lines = contents
                    .iter()
                    .cloned()
                    .map(Line::from)
                    .collect::<Vec<Line>>();
                let paragraph = Paragraph::new(Text::from(lines)).block(Block::new());
                frame.render_widget(Clear, actual_area);
                frame.render_widget(paragraph, actual_area);
            }
        }
        Ok(())
    }
}
