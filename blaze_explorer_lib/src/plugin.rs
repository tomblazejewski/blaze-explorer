use crate::mode::Mode;
use std::collections::HashMap;
use std::fmt::Debug;

use ratatui::crossterm::event::KeyEvent;

use crate::{action::Action, app::App};

pub mod plugin_action;
pub mod plugin_helpers;
pub mod plugin_popup;

fn build_keymap(
    functionality_map: HashMap<String, Action>,
    bindings_map: HashMap<(Mode, Vec<KeyEvent>), String>,
) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
    let mut output_map = HashMap::new();
    for (key, value) in bindings_map {
        match functionality_map.get(&value) {
            Some(action) => {
                output_map.insert((key).clone(), action.clone());
            }
            None => {}
        }
    }
    output_map
}
pub trait Plugin: PluginSuper {
    fn display_details(&self) -> String {
        String::from("Not implemented")
    }
    fn get_plugin_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String>;
    fn get_popup_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String>;
    fn get_all_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String> {
        let plugin_bindings = self.get_plugin_bindings();
        let popup_bindings = self.get_popup_bindings();
        let mut all_bindings = HashMap::new();
        all_bindings.extend(plugin_bindings);
        all_bindings.extend(popup_bindings);
        all_bindings
    }
    fn get_functionality_map(&self) -> HashMap<String, Action>;
    fn get_plugin_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        let functionality_map = self.get_functionality_map();
        let plugin_bindings = self.get_plugin_bindings();
        build_keymap(functionality_map, plugin_bindings)
    }
    fn get_popup_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        let functionality_map = self.get_functionality_map();
        let popup_bindings = self.get_popup_bindings();
        build_keymap(functionality_map, popup_bindings)
    }
    fn get_all_keymaps(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        let plugin_keymap = self.get_plugin_keymap();
        let popup_keymap = self.get_popup_keymap();
        let mut all_keymap = HashMap::new();
        all_keymap.extend(plugin_keymap);
        all_keymap.extend(popup_keymap);
        all_keymap
    }
}

pub trait PluginSuper: Debug {
    fn clone_box(&self) -> Box<dyn Plugin>;
}

impl<T> PluginSuper for T
where
    T: Plugin + Clone + Debug + 'static,
{
    fn clone_box(&self) -> Box<dyn Plugin> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Plugin> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
