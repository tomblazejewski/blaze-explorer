use super::command_utilities::{
    copy_to_backup, copy_to_clipboard, move_from_clipboard, read_from_clipboard, remove_if_folder,
    rename_recursively,
};
use crate::command::Command;

use super::command_utils::get_backup_dir;

use crate::action::{Action, AppAction};
use std::fmt::Debug;
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
        match rename_recursively(self.first_path.clone(), self.second_path.clone()) {
            Ok(_) => {
                //attempt to remove the old path
                match remove_if_folder(self.first_path.clone()) {
                    Ok(_) => {
                        self.reversible = true;
                        None
                    }
                    Err(e) => {
                        remove_if_folder(self.second_path.clone()).unwrap();
                        Some(Action::AppAct(AppAction::DisplayMessage(format!(
                            "Failed to remove the original path {}: {}",
                            self.first_path.display(),
                            e
                        ))))
                    }
                }
            }
            Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                "Failed to rename {}: {}",
                self.first_path.display(),
                e
            )))),
        }
    }

    fn undo(&mut self, _app: &mut App) -> Option<Action> {
        match rename_recursively(self.second_path.clone(), self.first_path.clone()) {
            Ok(_) => match remove_if_folder(self.second_path.clone()) {
                Ok(_) => None,
                Err(e) => {
                    remove_if_folder(self.first_path.clone()).unwrap();
                    Some(Action::AppAct(AppAction::DisplayMessage(format!(
                        "Failed to remove the original path {}: {}",
                        self.second_path.display(),
                        e
                    ))))
                }
            },
            Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                "Failed to rename {}: {}",
                self.first_path.display(),
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
    source_files: Option<Vec<PathBuf>>,
}

impl PasteFromClipboard {
    pub fn new(mut ctx: AppContext) -> Self {
        let current_directory = ctx.explorer_manager.get_current_path();
        Self {
            current_directory,
            source_files: None,
        }
    }
}

impl Command for PasteFromClipboard {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        // If this is executed for the first time, get file list from the clipboard. If
        // this is redone, file list is already attached to the struct.
        if self.source_files.is_none() {
            let files = match read_from_clipboard() {
                Ok(files) => files,
                Err(e) => {
                    return Some(Action::AppAct(AppAction::DisplayMessage(format!(
                        "Error while pasting: {:?}",
                        e
                    ))));
                }
            };
            // Backup the files and save to source files
            let backup_dir = get_backup_dir(false);
            copy_to_backup(files.clone(), backup_dir);
            self.source_files = Some(files);
        };
        let files = match &self.source_files {
            None => {
                //pasting for the first time
                return Some(Action::AppAct(AppAction::DisplayMessage(format!(
                    "Unexpected error occurred - could not retrieve copied files from clipboard"
                ))));
            }

            Some(files) => files.clone(),
        };
        match move_from_clipboard(files, self.current_directory.clone()) {
            Ok(()) => None,
            Err(e) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                "Error while pasting: {:?}",
                e
            )))),
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Result, Write};
    use std::{
        collections::VecDeque,
        env,
        fs::{File, create_dir_all},
        path, thread,
        time::Duration,
    };

    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use tempdir::TempDir;

    use crate::{action::ExplorerAction, testing_utils::create_testing_folder};
    #[test]
    fn test_rename_active() {
        let mut app = App::new().unwrap();
        let current_path = env::current_dir().unwrap();
        let test_path = current_path.parent().unwrap().join("tests");
        app.update_path(test_path.clone(), Some("sheet.csv".to_string()));
        let apparent_current_path = test_path.join("sheet.csv");
        let mut rename_active =
            RenameActive::new(apparent_current_path.clone(), "sheet1.csv".to_string());
        rename_active.execute(&mut app);
        let expected_path = test_path.join("sheet1.csv");
        app.explorer_manager.refresh_contents();
        assert_eq!(
            app.explorer_manager.select_directory().unwrap(),
            expected_path
        );
        rename_active.undo(&mut app);
        app.explorer_manager.refresh_contents();
        assert_eq!(
            app.explorer_manager.select_directory().unwrap(),
            apparent_current_path
        );
    }
    #[test]
    fn test_rename_active_folder() {
        let mut app = App::new().unwrap();
        let current_path = env::current_dir().unwrap();
        let test_path = current_path.parent().unwrap().join("tests");
        app.update_path(test_path.clone(), Some("folder_1".to_string()));
        let apparent_current_path = test_path.join("folder_1");
        let mut rename_active =
            RenameActive::new(apparent_current_path.clone(), "folder_2".to_string());
        rename_active.execute(&mut app);
        let expected_path = test_path.join("folder_2");
        app.explorer_manager.refresh_contents();
        assert_eq!(
            app.explorer_manager.select_directory().unwrap(),
            expected_path
        );
        rename_active.undo(&mut app);
        app.explorer_manager.refresh_contents();
        assert_eq!(
            app.explorer_manager.select_directory().unwrap(),
            apparent_current_path
        );
    }

    #[test]
    fn test_write_read_clipboard() {
        let mut app = App::new().unwrap();
        let current_path = env::current_dir().unwrap();
        let test_path = current_path.parent().unwrap().join("tests");
        app.update_path(test_path.clone(), Some("sheet.csv".to_string()));
        let mut copy_selection = CopyToClipboard::new(app.get_app_context());
        copy_selection.execute(&mut app);
        let folder_1 = test_path.join("folder_1");
        app.update_path(folder_1.clone(), None);
        let mut paste_selection = PasteFromClipboard::new(app.get_app_context());
        paste_selection.execute(&mut app);
        let expected_path = folder_1.join("sheet.csv");
        app.explorer_manager.refresh_contents();
        app.explorer_manager.next();
        assert_eq!(
            app.explorer_manager.select_directory().unwrap(),
            expected_path
        );
        let mut del_action = DeleteSelection::new(app.get_app_context());
        del_action.execute(&mut app);
        app.explorer_manager.refresh_contents();
        app.move_directory(current_path, None);
    }

    #[test]
    fn test_write_delete_read_clipboard() {
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
    fn test_delete_command_new() {
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
        print!("Settting path to {:?}", starting_path);
        app.move_directory(starting_path, None);
    }
}
