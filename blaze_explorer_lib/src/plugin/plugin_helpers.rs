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

pub use create_plugin_action;

use crate::{
    action::{Action, AppAction},
    app::App,
};

use super::Plugin;

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
