use super::HistoryStack;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectoryDetails {
    pub directory: PathBuf,
    pub selected: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
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

mod tests {
    use std::{env, path};

    use super::*;

    #[test]
    fn test_history_stack() {
        let mut history = DirectoryHistory::new();
        let starting_path = env::current_dir().unwrap();
        history.perform(DirectoryDetails {
            directory: starting_path.clone(),
            selected: Some("src".into()),
        });
        let new_path = path::absolute("tests/").unwrap();
        history.perform(DirectoryDetails {
            directory: new_path.clone(),
            selected: None,
        });

        let third_path = path::absolute("src/debug").unwrap();
        history.perform(DirectoryDetails {
            directory: third_path.clone(),
            selected: None,
        });
        let undo = history.undo();
        assert_eq!(undo.unwrap().directory, new_path.clone());
        let undo = history.undo();
        assert_eq!(undo.unwrap().directory, starting_path.clone());
        let redo = history.redo();
        assert_eq!(redo.unwrap().directory, new_path.clone());

        let past_directories = history.past_directories.clone();
        let future_directories = history.future_directories.clone();
        let current_directory = history.current_directory.clone();
        assert_eq!(
            past_directories,
            vec![DirectoryDetails {
                directory: starting_path,
                selected: Some("src".into()),
            }]
        );
        assert_eq!(
            future_directories,
            vec![DirectoryDetails {
                directory: third_path,
                selected: None,
            }]
        );
        assert_eq!(
            current_directory,
            Some(DirectoryDetails {
                directory: new_path,
                selected: None,
            })
        );
    }
}
