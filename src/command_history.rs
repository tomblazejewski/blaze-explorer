use crate::command::Command;

#[derive(Debug)]
pub struct CommandHistory {
    past_commands: Vec<Box<dyn Command>>,
    future_commands: Vec<Box<dyn Command>>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            past_commands: Vec::new(),
            future_commands: Vec::new(),
        }
    }

    pub fn push_command(&mut self, command: Box<dyn Command>) {
        self.past_commands.push(command);
        self.future_commands.clear();
    }

    pub fn pop_command(&mut self) -> Option<Box<dyn Command>> {
        let popped_command = self.past_commands.pop();
        if let Some(boxed_command) = &popped_command {
            self.future_commands.push(boxed_command.clone());
        }

        popped_command
    }
}
