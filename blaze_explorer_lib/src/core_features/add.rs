use crate::{command::file_commands::AddDir, plugin::plugin_action::PluginAction};
use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;
use ratatui::{
    Frame,
    crossterm::event::KeyEvent,
    layout::{Constraint, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{
    action::Action,
    app::App,
    create_plugin_action,
    input_machine::input_machine_helpers::convert_str_to_events,
    line_entry::LineEntry,
    mode::Mode,
    plugin::{
        plugin_commands::{PluginConfirmResult, PluginDropSearchChar, PluginQuit},
        plugin_helpers::get_push_on_char_action,
        plugin_popup::PluginPopUp,
    },
    query::Query,
    tools::center_rect,
};
pub fn open_add_popup(app: &mut App) -> Option<Action> {
    let mut ctx = app.get_app_context();
    let dir = match ctx.explorer_manager.select_directory() {
        Some(dir) => dir.clone(),
        //Unable to set the selected to None
        None => {
            panic!("Could not get the starting directory from the explorer manager")
        }
    };
    let popup = Box::new(AddPopUp::new(dir));
    app.attach_popup(popup);

    None
}

fn get_add_popup_keymap() -> HashMap<(Mode, Vec<KeyEvent>), Action> {
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
pub struct AddPopUp {
    pub should_quit: bool,
    query: Query,
    current_dir: PathBuf,
    keymap: HashMap<(Mode, Vec<KeyEvent>), Action>,
}

impl AddPopUp {
    pub fn new(dir: PathBuf) -> Self {
        let query = Query::new("".to_string(), "".to_string());
        Self {
            should_quit: false,
            query,
            current_dir: dir,
            keymap: get_add_popup_keymap(),
        }
    }
}

impl PluginPopUp for AddPopUp {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let query_area = center_rect(
            frame.size(),
            Constraint::Percentage(50),
            Constraint::Length(3),
        );
        let title = "Add dir";
        let query_block = Block::default().borders(Borders::ALL).title(title);
        let rename_field_output = self.get_search_query();
        let query_paragraph = Paragraph::new(rename_field_output);
        let query_paragraph = query_paragraph.block(query_block);

        frame.render_widget(Clear, query_area);
        frame.render_widget(query_paragraph, query_area);

        Ok(())
    }

    fn push_search_char(&mut self, ch: char) -> Option<Action> {
        self.query.append_char(ch);
        None
    }

    fn drop_search_char(&mut self) -> Option<Action> {
        self.query.drop_char();
        None
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn erase_text(&mut self) -> Option<Action> {
        self.query.clear_contents();
        None
    }

    fn get_search_query(&self) -> String {
        self.query.get_contents()
    }

    fn display_details(&self) -> String {
        "Add dir".to_string()
    }

    fn get_own_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        self.keymap.clone()
    }

    fn get_default_action(&self) -> Box<fn(KeyEvent) -> Option<Action>> {
        Box::new(get_push_on_char_action)
    }
    fn confirm_result(&mut self) -> Option<Action> {
        self.quit();
        Some(create_plugin_action!(
            AddDir,
            self.current_dir.clone(),
            self.get_search_query()
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::testing_utils::create_custom_testing_folder;

    use super::*;
    #[test]
    fn test_add_popup() {
        let mut app = App::new().unwrap();
        let test_folder = create_custom_testing_folder(vec!["test_folder/"]).unwrap();
        let root_dir = test_folder.root_dir.path().to_path_buf();
        app.update_path(root_dir.clone(), Some("test_folder".to_string()));
        open_add_popup(&mut app);
        let expected_popup: Box<dyn PluginPopUp> = Box::new(AddPopUp::new(root_dir.clone()));
        let actual_popup = app.popup.unwrap();
        assert!(expected_popup == actual_popup.clone());
    }
}
