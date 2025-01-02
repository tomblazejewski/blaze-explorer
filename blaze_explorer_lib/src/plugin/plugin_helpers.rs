#[macro_export]
macro_rules! create_plugin_action {
    // Case where the command takes arguments
    ($command:ident, $($args:expr),*) => {
        {
            let command = $command::new($($args),*);
            let plugin_action = PluginAction::new(Box::new(command));
            Action::PluginAct(plugin_action)
        }
    };

    // Case where the command takes no arguments
    ($command:ident) => {
        {
            let command = $command::new();
            let plugin_action = PluginAction::new(Box::new(command));
            Action::PluginAct(plugin_action)
        }
    };
}

use std::collections::HashMap;

use color_eyre::eyre::Result;
pub use create_plugin_action;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{
    action::{Action, AppAction},
    app::App,
    mode::Mode,
};

use super::{plugin_popup::PluginPopUp, Plugin};

pub enum PluginFetchResult {
    Err(Option<Action>),
    Ok(Box<dyn Plugin>),
}

pub fn access_plugin(app: &App, plugin_name: &str) -> PluginFetchResult {
    match app.plugins.get(plugin_name) {
        None => PluginFetchResult::Err(Some(Action::AppAct(AppAction::DisplayMessage(format!(
            "Failed to fetch the {} plugin when trying to open the popup",
            plugin_name
        ))))),
        Some(plugin) => PluginFetchResult::Ok(plugin.to_owned()),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DummyPlugin {
    plugin_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    popup_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    functionality_map: HashMap<String, Action>,
}
impl Plugin for DummyPlugin {
    fn get_plugin_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String> {
        self.plugin_bindings.to_owned()
    }

    fn get_popup_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String> {
        self.popup_bindings.to_owned()
    }

    fn get_functionality_map(&self) -> HashMap<String, Action> {
        self.functionality_map.to_owned()
    }
}

impl DummyPlugin {
    pub fn new() -> Self {
        Self {
            plugin_bindings: HashMap::new(),
            popup_bindings: HashMap::new(),
            functionality_map: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DummyPluginPopUp {
    keymap: HashMap<(Mode, Vec<KeyEvent>), Action>,
    should_quit: bool,
}

impl DummyPluginPopUp {
    pub fn new() -> Self {
        Self {
            keymap: HashMap::new(),
            should_quit: false,
        }
    }
}
impl PluginPopUp for DummyPluginPopUp {
    fn draw(&mut self, _frame: &mut Frame, _area: Rect) -> Result<()> {
        Ok(())
    }

    fn push_search_char(&mut self, ch: char) -> Option<Action> {
        None
    }

    fn drop_search_char(&mut self) -> Option<Action> {
        None
    }

    fn quit(&mut self) {
        self.should_quit = true
    }

    fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn erase_text(&mut self) -> Option<Action> {
        None
    }

    fn get_search_query(&self) -> String {
        "".to_string()
    }

    fn display_details(&self) -> String {
        "DummyPopUp".to_string()
    }

    fn get_own_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        self.keymap.clone()
    }
}
