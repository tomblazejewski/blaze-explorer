use super::HistoryStack;
use crate::{
    command::Command,
    components::component_helpers::{Numbering, get_line_numbers},
    core_features::preview::Previewable,
};

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

impl Previewable for CommandHistory {
    fn collect_data(&self) -> Vec<String> {
        let mut commands = self.future_commands.clone();
        commands.append(&mut self.past_commands.clone());
        commands
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
    }

    fn get_numbering(&self) -> Numbering {
        Numbering::VimLike
    }

    fn get_line_numbers(&self) -> Option<Vec<String>> {
        let n_lines = self.collect_data().len();
        get_line_numbers(n_lines, self.future_commands.len(), self.get_numbering())
    }
}

mod tests {
    use crate::command::{Quit, ResetStyling};

    use super::*;

    #[test]
    fn test_correct_collect_data() {
        let mut command_history = CommandHistory::new();
        let reset_styling_command = Box::new(ResetStyling::new());
        let quit_command = Box::new(Quit::new());
        command_history.perform(reset_styling_command);
        command_history.perform(quit_command);
        assert_eq!(command_history.collect_data(), vec!["ResetStyling", "Quit"]);
    }
}
