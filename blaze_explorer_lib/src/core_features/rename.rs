use std::collections::HashMap;

use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    action::Action,
    create_plugin_action,
    input_machine::input_machine_helpers::convert_str_to_events,
    mode::Mode,
    plugin::{
        plugin_action::PluginAction,
        plugin_commands::{PluginConfirmResult, PluginDropSearchChar, PluginQuit},
        plugin_helpers::get_push_on_char_action,
        plugin_popup::PluginPopUp,
    },
    tools::center_rect,
};

fn get_rename_popup_keymap() -> HashMap<(Mode, Vec<KeyEvent>), Action> {
    let mut keymap = HashMap::new();
    keymap.insert(
        (Mode::PopUp, convert_str_to_events("<Esc>")),
        create_plugin_action!(PluginQuit),
    );
    keymap.insert(
        (Mode::PopUp, convert_str_to_events("<BS>")),
        create_plugin_action!(PluginDropSearchChar),
    );
    keymap.insert(
        (Mode::PopUp, convert_str_to_events("<CR>")),
        create_plugin_action!(PluginConfirmResult),
    );
    keymap
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenamePopUp {
    pub should_quit: bool,
    query: String,
    initial_name: String,
    extension: String,
    keymap: HashMap<(Mode, Vec<KeyEvent>), Action>,
}

impl RenamePopUp {
    pub fn new(initial_name: String, extension: String) -> Self {
        Self {
            should_quit: false,
            query: "".to_string(),
            initial_name,
            extension,
            keymap: get_rename_popup_keymap(),
        }
    }
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

    fn get_default_action(&self) -> Box<fn(KeyEvent) -> Option<Action>> {
        Box::new(get_push_on_char_action)
    }
}
