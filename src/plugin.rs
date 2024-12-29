use std::collections::HashMap;

use crate::{action::Action, app::App};

pub mod plugin_popup;
pub mod sfs_telescope;
pub mod telescope;
pub mod telescope_commands;
pub trait Plugin {
    fn display_details(&self) -> String {
        String::from("Not implemented")
    }
    fn attach_functionality(
        &self,
        app: &mut App,
    ) -> HashMap<String, Box<fn(&mut App) -> Option<Action>>>;
}
