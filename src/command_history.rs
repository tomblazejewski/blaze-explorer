use crate::command::Command;

#[derive(Debug)]
pub struct CommandHistory {
    past_commands: Vec<Box<dyn Command>>,
    future_commands: Vec<Box<dyn Command>>,
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            past_commands: Vec::new(),
            future_commands: Vec::new(),
        }
    }

    pub fn perform(&mut self, command: Box<dyn Command>) {
        self.past_commands.push(command);
        self.future_commands.clear();
    }

    pub fn undo(&mut self) -> Option<Box<dyn Command>> {
        let popped_command = self.past_commands.pop();
        if let Some(boxed_command) = &popped_command {
            self.future_commands.push(boxed_command.clone());
        }

        popped_command
    }

    pub fn redo(&mut self) -> Option<Box<dyn Command>> {
        let popped_command = self.future_commands.pop();
        if let Some(boxed_command) = &popped_command {
            self.past_commands.push(boxed_command.clone());
        }
        popped_command
    }
}
