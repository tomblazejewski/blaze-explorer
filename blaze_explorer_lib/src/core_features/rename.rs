use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    action::{Action, AppAction},
    app::App,
    command::RenameActive,
    create_plugin_action,
    input_machine::input_machine_helpers::convert_str_to_events,
    line_entry::LineEntry,
    mode::Mode,
    plugin::{
        plugin_action::PluginAction,
        plugin_commands::{PluginConfirmResult, PluginDropSearchChar, PluginQuit},
        plugin_helpers::get_push_on_char_action,
        plugin_popup::PluginPopUp,
    },
    query::Query,
    tools::center_rect,
};
pub fn open_rename_popup(app: &mut App) -> Option<Action> {
    let mut ctx = app.get_app_context();
    let dir = match ctx.explorer_manager.select_directory() {
        Some(dir) => dir.clone(),
        //Unable to set the selected to None
        None => {
            panic!("Could not get the starting directory from the explorer manager")
        }
    };
    let popup = Box::new(RenamePopUp::new(dir));
    app.attach_popup(popup);

    None
}

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
    query: Query,
    initial_path: PathBuf,
    initial_name: String,
    extension: String,
    keymap: HashMap<(Mode, Vec<KeyEvent>), Action>,
}

impl RenamePopUp {
    pub fn new(dir: PathBuf) -> Self {
        let extension = match dir.extension() {
            Some(path) => format!(".{}", path.to_str().unwrap()),
            None => "".to_string(),
        };
        let initial_name = dir.file_name().unwrap().to_str().unwrap().to_string();
        let query = Query::new("".to_string(), format!(".{}", extension.clone()));
        Self {
            should_quit: false,
            query,
            initial_path: dir,
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
        let rename_field_output = self.get_search_query();
        let query_paragraph = Paragraph::new(rename_field_output);
        let query_paragraph = query_paragraph.block(query_block);

        frame.render_widget(Clear, area);
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
        format!("Rename {}{}", self.initial_name, self.extension)
    }

    fn get_own_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        self.keymap.clone()
    }

    fn get_default_action(&self) -> Box<fn(KeyEvent) -> Option<Action>> {
        Box::new(get_push_on_char_action)
    }
    fn confirm_result(&mut self) -> Option<Action> {
        Some(create_plugin_action!(
            RenameActive,
            self.initial_path.clone(),
            self.get_search_query()
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path};

    use super::*;
    #[test]
    fn test_open_rename_popup() {
        let mut app = App::new().unwrap();
        let current_path = env::current_dir().unwrap();
        let test_path = current_path.join("../tests");
        app.update_path(test_path.clone(), Some("folder_1".to_string()));
        open_rename_popup(&mut app);
        let expected_dir = test_path.join("folder_1");
        let expected_popup: Box<dyn PluginPopUp> = Box::new(RenamePopUp::new(expected_dir));
        let actual_popup = app.popup.unwrap();
        assert!(expected_popup == actual_popup.clone());
    }
}
