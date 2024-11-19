use tracing::info;

use super::HistoryStack;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DirectoryDetails {
    pub directory: PathBuf,
    pub selected: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DirectoryHistory {
    current_directory: Option<DirectoryDetails>,
    past_directories: Vec<DirectoryDetails>,
    future_directories: Vec<DirectoryDetails>,
}

impl Default for DirectoryHistory {
    fn default() -> Self {
        Self::new()
    }
}
impl HistoryStack<DirectoryDetails> for DirectoryHistory {
    fn new() -> Self {
        Self {
            current_directory: None,
            past_directories: Vec::new(),
            future_directories: Vec::new(),
        }
    }

    fn perform(&mut self, directory_details: DirectoryDetails) {
        let cd = self.current_directory.take();
        if let Some(cd) = cd {
            self.past_directories.push(cd);
        }
        self.current_directory = Some(directory_details);
        self.future_directories.clear();
    }

    fn undo(&mut self) -> Option<DirectoryDetails> {
        let popped_directory = self.past_directories.pop();
        if let Some(directory) = &popped_directory {
            self.future_directories
                .push(self.current_directory.take().unwrap());
            self.current_directory = Some(directory.clone());
        }
        popped_directory
    }

    fn redo(&mut self) -> Option<DirectoryDetails> {
        let popped_directory = self.future_directories.pop();
        if let Some(directory) = &popped_directory {
            self.past_directories
                .push(self.current_directory.take().unwrap());
            self.current_directory = Some(directory.clone());
        }
        popped_directory
    }
}
