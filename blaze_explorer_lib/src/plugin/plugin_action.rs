use crate::command::Command;

#[derive(Clone, Debug)]
pub struct PluginAction {
    command: Box<dyn Command>,
}

impl PartialEq for PluginAction {
    fn eq(&self, other: &Self) -> bool {
        self.command == other.command.clone()
    }
}

impl PluginAction {
    pub fn new(command: Box<dyn Command>) -> PluginAction {
        PluginAction { command }
    }

    pub fn get_command(&self) -> Box<dyn Command> {
        self.command.clone()
    }
}
