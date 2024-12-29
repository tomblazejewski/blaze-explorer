use crate::mode::Mode;
use std::collections::HashMap;
use std::fmt::Debug;

use ratatui::crossterm::event::KeyEvent;

use crate::{action::Action, app::App};

pub mod plugin_action;
pub mod plugin_helpers;
pub mod plugin_popup;
pub mod sfs_telescope;
pub mod telescope;
pub mod telescope_commands;
pub trait Plugin: PluginSuper {
    fn display_details(&self) -> String {
        String::from("Not implemented")
    }
    fn attach_functionality(
        &self,
        app: &mut App,
    ) -> HashMap<String, Box<fn(&mut App) -> Option<Action>>>;
    fn get_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action>;
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
