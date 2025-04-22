use itertools::Itertools;

use super::command_utils::{copy_recursively, copy_to_clipboard, read_from_clipboard};
use crate::command::Command;

use super::command_utils::{create_backup_map, get_backup_dir, join_paths};

use crate::action::{Action, AppAction};
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::{collections::HashMap, path::PathBuf};
use std::{fmt, fs};

use crate::{app::App, app_context::AppContext, mode::Mode};

#[derive(Clone, PartialEq)]
pub struct DeleteSelection {
    pub affected_files: Option<Vec<PathBuf>>,
    backup_path: Option<HashMap<PathBuf, PathBuf>>,
}

/// Command used to delete files. Considers all selected items at the time of creating the struct.
impl DeleteSelection {
    pub fn new(mut ctx: AppContext) -> Self {
        let affected_files = ctx.explorer_manager.get_affected_paths();
        Self {
            affected_files,
            backup_path: None,
        }
    }
}
impl Command for DeleteSelection {
    /// Assign a backup path for each individual entry selected
    /// Move each of the entries to their designated backup path
    fn execute(&mut self, _app: &mut App) -> Option<Action> {
        if let Some(contents) = &self.affected_files {
            match &self.backup_path {
                None => {
                    let contents_map = contents
                        .iter()
                        .map(|f| (f.to_owned(), get_backup_dir(false)))
                        .collect::<HashMap<PathBuf, PathBuf>>();
                    self.backup_path = Some(contents_map);
                }
                Some(_contents) => {}
            }
            let result = contents
                .iter()
                .map(|f| {
                    let backup_path = self.backup_path.as_ref().unwrap().get(f).unwrap();
                    fs::rename(f, backup_path)
                })
                .collect::<Vec<io::Result<()>>>();
        };
        Some(Action::AppAct(AppAction::SwitchMode(Mode::Normal)))
    }

    fn undo(&mut self, _app: &mut App) -> Option<Action> {
        if let Some(contents) = &self.backup_path {
            let _ = contents
                .iter()
                .map(|(original_path, backup_path)| fs::rename(backup_path, original_path))
                .collect::<Vec<io::Result<()>>>();
            return None;
        };
        None
    }
    fn is_reversible(&self) -> bool {
        true
    }
}

impl Debug for DeleteSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeleteSelection")
            .field("to delete", &self.affected_files)
            .field("backup_path", &self.backup_path)
            .finish()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RenameActive {
    pub first_path: PathBuf,
    pub second_path: PathBuf,
    reversible: bool,
}

/// Rename currently selected file
impl RenameActive {
    pub fn new(first_path: PathBuf, new_name: String) -> Self {
        let second_path = first_path.parent().unwrap().join(new_name);
        Self {
            first_path,
            second_path,
            reversible: false,
        }
    }
}

impl Command for RenameActive {
    fn execute(&mut self, _app: &mut App) -> Option<Action> {
        match fs::rename(self.first_path.clone(), self.second_path.clone()) {
            Ok(_) => {
                self.reversible = true;
                None
            }
            Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                "Failed to rename {}: {}",
                self.first_path.display(),
                e
            )))),
        }
    }

    fn undo(&mut self, _app: &mut App) -> Option<Action> {
        match fs::rename(self.second_path.clone(), self.first_path.clone()) {
            Ok(_) => None,
            Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                "Failed to rename {}: {}",
                self.second_path.display(),
                e
            )))),
        }
    }
    fn is_reversible(&self) -> bool {
        self.reversible
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct CopyToClipboard {
    affected_files: Option<Vec<PathBuf>>,
}

impl CopyToClipboard {
    pub fn new(mut ctx: AppContext) -> Self {
        let affected_files = ctx.explorer_manager.get_affected_paths();
        Self { affected_files }
    }
}

impl Command for CopyToClipboard {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &self.affected_files {
            Some(affected_files) => {
                match copy_to_clipboard(
                    affected_files.iter().map(|x| x.to_str().unwrap()).collect(),
                ) {
                    Ok(()) => None,
                    Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                        "Error while copying: {:?}",
                        e
                    )))),
                }
            }
            None => None,
        };
        Some(Action::AppAct(AppAction::SwitchMode(Mode::Normal)))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct PasteFromClipboard {
    current_directory: PathBuf,
    source_files_map: Option<HashMap<PathBuf, PathBuf>>,
    reversible: bool,
}

/// Paste files from clipboard
/// Undoing this action removes the pasted files from the given directory, while redoing the action
/// will paste the exact same files back (even if clipboard contents have changed)
impl PasteFromClipboard {
    pub fn new(mut ctx: AppContext) -> Self {
        let current_directory = ctx.explorer_manager.get_current_path();
        Self {
            current_directory,
            source_files_map: None,
            reversible: false,
        }
    }
}

impl Command for PasteFromClipboard {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        // Retrieve file paths to copy
        // There are two options:
        // 1. Pasting for the first time - take the paths from clipboard
        // 2. Pasting again - take the paths from [source_files] field of the struct

        let (copy_map, should_delete) = match &self.source_files_map {
            Some(map) => (map.to_owned(), true),
            None => {
                //Read from clipboard and join paths
                let paths_to_copy = match read_from_clipboard() {
                    Ok(paths) => paths,
                    Err(e) => {
                        return Some(Action::AppAct(AppAction::DisplayMessage(format!(
                            "Could not read from clipboard: {}",
                            e
                        ))));
                    }
                };
                // represents the source file and target file
                let map = paths_to_copy
                    .clone()
                    .into_iter()
                    .zip(join_paths(paths_to_copy.clone(), &self.current_directory))
                    .collect::<HashMap<PathBuf, PathBuf>>();
                (map, false)
            }
        };
        for (source_path, target_path) in copy_map.iter() {
            match copy_recursively(source_path, target_path) {
                Ok(_) => {
                    if should_delete {
                        let _ = fs::remove_dir_all(source_path);
                    }
                }
                Err(e) => {
                    return Some(Action::AppAct(AppAction::DisplayMessage(format!(
                        "Error while copying: {:?}",
                        e
                    ))));
                }
            }
        }

        if self.source_files_map.is_none() {
            let backup_path =
                create_backup_map(copy_map.values().cloned().collect::<Vec<PathBuf>>());
            let inverted_map = backup_path
                .iter()
                .map(|(k, v)| (v.to_owned(), k.to_owned()))
                .collect::<HashMap<PathBuf, PathBuf>>();
            self.source_files_map = Some(inverted_map);
        }
        self.reversible = true;

        // sort pasted file_names/dir_names alphabetically so that the action can select the first
        // one
        let first_file_path = copy_map.values().sorted().next().unwrap().to_owned();
        Some(Action::AppAct(AppAction::ShowInFolder(first_file_path)))
    }

    fn undo(&mut self, app: &mut App) -> Option<Action> {
        let mut result = Ok(());
        for (backup_path, target_path) in self.source_files_map.as_ref().unwrap().iter() {
            match fs::rename(target_path, backup_path) {
                Ok(_) => (),
                Err(e) => {
                    result = Err(e);
                }
            }
        }
        match result {
            Ok(()) => None,
            Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                "Error while copying: {:?}",
                e
            )))),
        }
    }

    fn is_reversible(&self) -> bool {
        self.reversible
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AddDir {
    new_dir: PathBuf,
    is_folder: bool,
    reversible: bool,
}

impl AddDir {
    pub fn new(current_dir: PathBuf, mut name: String) -> Self {
        let mut is_folder = false;
        if name.ends_with('\\') || name.ends_with('/') {
            //remove the last char
            name = name[0..name.len() - 1].to_string();
            is_folder = true;
        }
        Self {
            new_dir: current_dir.join(name),
            is_folder,
            reversible: false,
        }
    }
}

impl Command for AddDir {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match self.is_folder {
            true => match fs::create_dir(&self.new_dir) {
                Ok(()) => {
                    self.reversible = true;
                    None
                }
                Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                    "Error while creating directory: {:?}",
                    e
                )))),
            },
            false => match File::create(&self.new_dir) {
                Ok(file) => {
                    self.reversible = true;
                    None
                }
                Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                    "Error while creating directory: {:?}",
                    e
                )))),
            },
        }
    }

    fn undo(&mut self, _app: &mut App) -> Option<Action> {
        match self.is_folder {
            true => match fs::remove_dir_all(&self.new_dir) {
                Ok(()) => None,
                Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                    "Error while removing directory: {:?}",
                    e
                )))),
            },
            false => match fs::remove_file(&self.new_dir) {
                Ok(()) => None,
                Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                    "Error while removing directory: {:?}",
                    e
                )))),
            },
        }
    }
    fn is_reversible(&self) -> bool {
        self.reversible
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    use crate::{
        action::ExplorerAction,
        testing_utils::{create_custom_testing_folder, create_testing_folder},
    };
    #[test]
    fn test_rename_active() {
        let mut app = App::new().unwrap();
        let testing_folder = create_testing_folder().unwrap();
        let path_to_rename = testing_folder.root_dir.path().to_path_buf();
        let new_name = "new_folder_name".to_string();
        let mut rename_active = RenameActive::new(path_to_rename.clone(), new_name.clone());
        rename_active.execute(&mut app);
        assert!(!path_to_rename.exists());
        for file_path in testing_folder.file_list.iter() {
            assert!(!file_path.exists());
        }
        assert!(path_to_rename.parent().unwrap().join(&new_name).exists());
        assert!(rename_active.is_reversible());
        rename_active.undo(&mut app);
        assert!(path_to_rename.exists());
        for file_path in testing_folder.file_list.iter() {
            assert!(file_path.exists());
        }
        assert!(!path_to_rename.parent().unwrap().join(&new_name).exists());
    }
    #[test]
    fn test_write_read_clipboard() {
        // Write a file to clipboard, move to another directory and paste it. Ensure the
        // new file was found in the new directory.
        let mut app = App::new().unwrap();
        let current_path = env::current_dir().unwrap();
        let testing_folder = create_testing_folder().unwrap();
        //copy file1.txt
        let file_to_copy = testing_folder.file_list[0].clone();
        app.update_path(
            file_to_copy.parent().unwrap().to_path_buf(),
            Some(
                file_to_copy
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
        );
        let mut copy_selection = CopyToClipboard::new(app.get_app_context());
        copy_selection.execute(&mut app);
        let folder_2 = testing_folder.dir_list[2].clone();
        app.update_path(folder_2.clone(), None);
        let mut paste_selection = PasteFromClipboard::new(app.get_app_context());
        let paste_action = paste_selection.execute(&mut app);
        let expected_action = Some(Action::AppAct(AppAction::ShowInFolder(
            folder_2.join(file_to_copy.file_name().unwrap()),
        )));
        assert_eq!(paste_action, expected_action);
        assert!(folder_2.join(file_to_copy.file_name().unwrap()).exists());

        //Ensure undoing removes the file
        let result = paste_selection.undo(&mut app);
        assert!(result.is_none(), "{:?}", result);
        assert!(!folder_2.join(file_to_copy.file_name().unwrap()).exists());
        app.move_directory(current_path, None);
    }
    #[test]
    fn test_write_read_clipboard_folder() {
        // Write a file to clipboard, move to another directory and paste it. Ensure the
        // new file was found in the new directory.
        let mut app = App::new().unwrap();
        let current_path = env::current_dir().unwrap();
        let testing_folder = create_testing_folder().unwrap();
        let file_to_copy = testing_folder.dir_list[2].clone();
        app.update_path(
            file_to_copy.parent().unwrap().to_path_buf(),
            Some(
                file_to_copy
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
        );
        let mut copy_selection = CopyToClipboard::new(app.get_app_context());
        copy_selection.execute(&mut app);
        let folder_to_paste = testing_folder.dir_list[0].clone();
        app.update_path(folder_to_paste.clone(), None);
        let mut paste_selection = PasteFromClipboard::new(app.get_app_context());
        let _ = paste_selection.execute(&mut app);
        assert!(
            folder_to_paste
                .join(file_to_copy.file_name().unwrap())
                .exists()
        );

        //Ensure undoing removes the file
        let _ = paste_selection.undo(&mut app);
        assert!(
            !folder_to_paste
                .join(file_to_copy.file_name().unwrap())
                .exists()
        );
        app.move_directory(current_path, None);
    }
    #[test]
    fn test_write_delete_read_clipboard() {
        // Ensure a display action is issued when trying to paste a deleted file.
        // TODO: remove the dependendcy on the test folder
        let mut app = App::new().unwrap();
        let current_path = env::current_dir().unwrap();
        let test_path = current_path.parent().unwrap().join("tests");
        app.update_path(test_path.clone(), Some("sheet.csv".to_string()));
        let mut copy_selection = CopyToClipboard::new(app.get_app_context());
        let mut delete_selection = DeleteSelection::new(app.get_app_context());
        copy_selection.execute(&mut app);
        delete_selection.execute(&mut app);
        let folder_1 = test_path.join("folder_1");
        app.update_path(folder_1.clone(), None);
        let mut paste_selection = PasteFromClipboard::new(app.get_app_context());
        let paste_action = paste_selection.execute(&mut app);
        match paste_action {
            Some(Action::AppAct(AppAction::DisplayMessage(_))) => {}
            _ => panic!("Expected a display action"),
        };
        app.update_path(test_path.clone(), Some("sheet.csv".to_string()));
        delete_selection.undo(&mut app);
        app.move_directory(current_path, None);
    }

    #[test]
    fn test_delete_command() {
        let mut app = App::new().unwrap();
        let testing_folder = create_testing_folder().unwrap();
        let starting_path = env::current_dir().unwrap();
        //enter the temp_dir path

        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                testing_folder.root_dir.path().to_path_buf(),
            )));
        let _ = app.handle_new_actions();
        let mut delete_selection = DeleteSelection::new(app.get_app_context());
        //set files to delete
        let to_delete = vec![
            testing_folder.file_list[0].clone(),
            testing_folder.file_list[2].clone(),
        ];
        for path in to_delete.iter() {
            assert!(path.exists());
        }
        //assert these files exist
        delete_selection.affected_files = Some(to_delete.clone());
        delete_selection.execute(&mut app);
        //assert these files do not exist
        for path in to_delete.iter() {
            assert!(!path.exists());
        }
        let _ = delete_selection.undo(&mut app);
        //assert the files exist again
        for path in to_delete.iter() {
            assert!(path.exists(), "File {:?} does not exist", path);
        }
        app.move_directory(starting_path, None);
    }

    #[test]
    fn test_add_dir() {
        let temp_dir = create_custom_testing_folder(Vec::new()).unwrap();
        let root_dir = temp_dir.root_dir.path().to_path_buf();
        let mut app = App::new().unwrap();
        app.explorer_manager.update_path(root_dir.clone(), None);
        let mut add_dir = AddDir::new(root_dir.clone(), "new_dir/".to_string());
        assert!(!add_dir.is_reversible());
        let result = add_dir.execute(&mut app);
        assert!(result.is_none());
        let new_dir = root_dir.join("new_dir");
        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
        assert!(add_dir.is_reversible());

        let _ = add_dir.undo(&mut app);
        assert!(!new_dir.exists());

        //test creating twice
        let result = add_dir.execute(&mut app);
        assert!(result.is_none());
        let new_dir = root_dir.join("new_dir");
        assert!(new_dir.exists());
        assert!(new_dir.is_dir());

        let result_repeat = add_dir.execute(&mut app);
        assert!(result_repeat.is_some());

        let mut add_dir = AddDir::new(root_dir.clone(), "text_file.txt".to_string());

        assert!(!add_dir.is_reversible());
        add_dir.execute(&mut app);
        let new_file = root_dir.join("text_file.txt");
        assert!(new_file.exists());
        assert!(new_file.is_file());
        assert!(add_dir.is_reversible());

        let _ = add_dir.undo(&mut app);
        assert!(!new_file.exists());
    }
}
