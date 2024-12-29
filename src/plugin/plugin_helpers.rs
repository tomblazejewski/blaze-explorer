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
