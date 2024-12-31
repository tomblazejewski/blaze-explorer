use super::HistoryStack;
use crate::command::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct CommandHistory {
    past_commands: Vec<Box<dyn Command>>,
    future_commands: Vec<Box<dyn Command>>,
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryStack<Box<dyn Command>> for CommandHistory {
    fn new() -> Self {
        Self {
            past_commands: Vec::new(),
            future_commands: Vec::new(),
        }
    }

    fn perform(&mut self, command: Box<dyn Command>) {
        self.past_commands.push(command);
        self.future_commands.clear();
    }

    fn undo(&mut self) -> Option<Box<dyn Command>> {
        let popped_command = self.past_commands.pop();
        if let Some(boxed_command) = &popped_command {
            self.future_commands.push(boxed_command.clone());
        }

        popped_command
    }

    fn redo(&mut self) -> Option<Box<dyn Command>> {
        let popped_command = self.future_commands.pop();
        if let Some(boxed_command) = &popped_command {
            self.past_commands.push(boxed_command.clone());
        }
        popped_command
    }
}
