use std::path::PathBuf;

#[derive(Debug)]
pub struct DirectoryHistory {
    past_directories: Vec<PathBuf>,
    future_directories: Vec<PathBuf>,
}

impl Default for DirectoryHistory {
    fn default() -> Self {
        Self::new()
    }
}
impl DirectoryHistory {
    pub fn new() -> Self {
        Self {
            past_directories: Vec::new(),
            future_directories: Vec::new(),
        }
    }

    pub fn perform(&mut self, directory: PathBuf) {
        self.past_directories.push(directory);
        self.future_directories.clear();
    }

    pub fn undo(&mut self) -> Option<PathBuf> {
        let popped_directory = self.past_directories.pop();
        if let Some(directory) = &popped_directory {
            self.future_directories.push(directory.clone());
        }
        popped_directory
    }
}
