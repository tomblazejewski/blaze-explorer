use std::collections::HashMap;

use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{action::Action, mode::Mode, plugin::plugin_popup::PluginPopUp, tools::center_rect};

#[derive(Debug, Clone, PartialEq)]
pub struct RenamePopUp {
    pub should_quit: bool,
    query: String,
    initial_name: String,
    extension: String,
    keymap: HashMap<(Mode, Vec<KeyEvent>), Action>,
}

impl PluginPopUp for RenamePopUp {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let query_area = center_rect(
            frame.size(),
            Constraint::Percentage(50),
            Constraint::Length(3),
        );
        let title = format!("Rename {}", self.initial_name);
        let query_block = Block::default().borders(Borders::ALL).title(title);
        let new_name = self.query.clone();
        let extension = self.extension.clone();
        let rename_field_output = format!("{}.{}", new_name, extension);
        let query_paragraph = Paragraph::new(rename_field_output);
        let query_paragraph = query_paragraph.block(query_block);

        frame.render_widget(Clear, area);
        frame.render_widget(query_paragraph, query_area);

        Ok(())
    }

    fn push_search_char(&mut self, ch: char) -> Option<Action> {
        self.query.push(ch);
        None
    }

    fn drop_search_char(&mut self) -> Option<Action> {
        self.query.pop();
        None
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn erase_text(&mut self) -> Option<Action> {
        self.query.clear();
        None
    }

    fn get_search_query(&self) -> String {
        self.query.clone()
    }

    fn display_details(&self) -> String {
        format!("Rename {}{}", self.initial_name, self.extension)
    }

    fn get_own_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        self.keymap.clone()
    }
}
