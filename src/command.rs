use crate::{action::Action, line_entry::LineEntry};
use ::std::fmt::Debug;
use core::panic;
use std::{collections::HashMap, error::Error, fs, io::Write, ops::Rem, path::PathBuf};

use crate::{
    app::App,
    mode::Mode,
    popup::{PopUp, PopUpWindow},
    telescope::AppContext,
};

pub trait Command: CommandClone {
    fn execute(&mut self, app: &mut App) -> Option<Action>;
    fn undo(&mut self, app: &mut App) -> Option<Action> {
        None
    }
    fn is_reversible(&self) -> bool {
        false
    }
}

pub trait CommandClone: Debug {
    fn clone_box(&self) -> Box<dyn Command>;
}

impl<T> CommandClone for T
where
    T: 'static + Command + Clone + Debug,
{
    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct ChangeDirectory {
    new_path: PathBuf,
}

impl ChangeDirectory {
    pub fn new(mut ctx: AppContext, path: PathBuf) -> Self {
        Self { new_path: path }
    }
}

impl Command for ChangeDirectory {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_table.update_path(self.new_path.clone());
        None
    }
}

#[derive(Clone, Debug)]
pub struct ParentDirectory {
    new_path: Option<PathBuf>,
}

impl ParentDirectory {
    pub fn new(mut ctx: AppContext) -> Self {
        let current_path = ctx.explorer_table.get_current_path();
        Self {
            new_path: current_path.parent().map(|name| name.to_owned()),
        }
    }
}

impl Command for ParentDirectory {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &self.new_path {
            Some(new_path) => app.explorer_table.update_path(new_path.clone()),
            _ => {}
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct SelectUp {}

impl SelectUp {
    pub fn new(ctx: AppContext) -> Self {
        Self {}
    }
}
impl Command for SelectUp {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_table.previous();
        None
    }
}

#[derive(Clone, Debug)]
pub struct SelectDown {}

impl SelectDown {
    pub fn new(ctx: AppContext) -> Self {
        Self {}
    }
}
impl Command for SelectDown {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_table.next();
        None
    }
}

#[derive(Clone, Debug)]
pub struct SelectDirectory {
    path: Option<PathBuf>,
}

impl SelectDirectory {
    pub fn new(mut ctx: AppContext) -> Self {
        Self {
            path: ctx.explorer_table.select_directory(),
        }
    }
}

impl Command for SelectDirectory {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &self.path {
            Some(path) => match path.is_dir() {
                true => app.update_path(path.clone()),
                false => app.open_default(path.clone()),
            },
            None => {}
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct UpdateSearchQuery {
    query: String,
}

impl UpdateSearchQuery {
    pub fn new(ctx: AppContext, query: String) -> Self {
        Self { query }
    }
}

impl Command for UpdateSearchQuery {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_table.update_search_query(self.query.clone());
        None
    }
}

#[derive(Clone, Debug)]
pub struct ClearSearchQuery {}

impl ClearSearchQuery {
    pub fn new(ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for ClearSearchQuery {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_table.clear_search_query();
        None
    }
}

#[derive(Clone, Debug)]
pub struct NextSearchResult {}

impl NextSearchResult {
    pub fn new(ctx: AppContext) -> Self {
        Self {}
    }
}
impl Command for NextSearchResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_table.next_search_result();
        None
    }
}

#[derive(Clone, Debug)]
pub struct ShowInFolder {
    current_file_path: PathBuf,
    target_path: PathBuf,
}

impl ShowInFolder {
    pub fn new(mut ctx: AppContext, path: PathBuf) -> Self {
        Self {
            current_file_path: ctx.explorer_table.select_directory().unwrap().clone(),
            target_path: path,
        }
    }
}

impl Command for ShowInFolder {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup = PopUp::None;
        app.explorer_table.show_in_folder(self.target_path.clone());
        None
    }
}

#[derive(Clone, Debug)]
pub struct Quit {}

impl Quit {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for Quit {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.should_quit = true;
        None
    }
}

#[derive(Clone, Debug)]
pub struct SwitchMode {
    mode: Mode,
}

impl SwitchMode {
    pub fn new(ctx: AppContext, mode: Mode) -> Self {
        Self { mode }
    }
}
impl Command for SwitchMode {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &self.mode {
            Mode::Normal => app.enter_normal_mode(),
            Mode::Command => app.enter_command_mode(),
            Mode::Search => app.enter_search_mode(),
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct ConfirmSearchQuery {}

impl ConfirmSearchQuery {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for ConfirmSearchQuery {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.confirm_search_query()
    }
}

#[derive(Clone, Debug)]
pub struct ConfirmCommand {}

impl ConfirmCommand {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for ConfirmCommand {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.execute_command()
    }
}

#[derive(Clone, Debug)]
pub struct OpenPopup {}

impl OpenPopup {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for OpenPopup {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup = PopUp::TelescopePopUp(PopUpWindow::new(app.get_app_context()));
        None
    }
}

#[derive(Clone, Debug)]
pub struct InsertKey {
    ch: char,
}

impl InsertKey {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }
}
impl Command for InsertKey {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.command_line.append_char(self.ch);
        None
    }
}

#[derive(Clone, Debug)]
pub struct EraseText {}

impl EraseText {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for EraseText {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.command_line.clear_contents();
        None
    }
}

#[derive(Clone, Debug)]
pub struct DropKey {}

impl DropKey {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for DropKey {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.command_line.remove_char()
    }
}

#[derive(Clone, Debug)]
pub struct TelescopeConfirmResult {}

impl TelescopeConfirmResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeConfirmResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup.confirm_result()
    }
}

#[derive(Clone, Debug)]
pub struct TelescopeNextResult {}

impl TelescopeNextResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeNextResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup.next_result();
        None
    }
}

#[derive(Clone, Debug)]
pub struct TelescopePreviousResult {}

impl TelescopePreviousResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopePreviousResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup.previous_result();
        None
    }
}

#[derive(Clone, Debug)]
pub struct TelescopeUpdateSearchQuery {
    query: String,
}

impl TelescopeUpdateSearchQuery {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}
impl Command for TelescopeUpdateSearchQuery {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup.update_search_query(self.query.clone());
        None
    }
}

#[derive(Clone, Debug)]
pub struct TelescopePushSearchChar {
    ch: char,
}

impl TelescopePushSearchChar {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }
}

impl Command for TelescopePushSearchChar {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup.push_search_char(self.ch)
    }
}

#[derive(Clone, Debug)]
pub struct TelescopeDropSearchChar {}

impl TelescopeDropSearchChar {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeDropSearchChar {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup.drop_search_char()
    }
}

#[derive(Clone, Debug)]
pub struct TelescopeQuit {}

impl TelescopeQuit {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeQuit {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup.quit();
        None
    }
}

#[derive(Clone, Debug)]
pub struct TelescopeEraseText {}

impl TelescopeEraseText {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeEraseText {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup.erase_text()
    }
}

#[derive(Clone, Debug)]
pub struct Noop {}

impl Noop {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for Noop {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateDir {
    File(PathBuf, Vec<u8>),
    Dir(PathBuf, Vec<Box<StateDir>>),
}

/// Save all the contents and structure of a given directory to a state.
pub fn read_to_state_dir(path: PathBuf) -> StateDir {
    match path.is_dir() {
        false => match fs::read(path.clone()) {
            Ok(contents) => StateDir::File(path.clone(), contents),
            Err(err) => panic!("Error with path: {:?}: {}", path, err),
        },
        true => {
            // traverse through all subdirectories
            let mut state_dir = Vec::new();

            for entry in fs::read_dir(path.clone()).unwrap() {
                match entry {
                    Ok(entry) => state_dir.push(Box::new(read_to_state_dir(entry.path()))),
                    Err(err) => panic!("Error with path: {:?}: {}", path, err),
                }
            }
            StateDir::Dir(path, state_dir)
        }
    }
}

//Unload contents of a statedir to the filesystem
pub fn write_state_dir(state_dir: StateDir) -> Result<(), std::io::Error> {
    match state_dir {
        StateDir::File(path, contents) => {
            fs::write(path, contents)?;
            Ok(())
        }
        StateDir::Dir(path, state_dir) => {
            fs::create_dir_all(path.clone())?;
            for state in state_dir {
                write_state_dir(*state)?
            }
            Ok(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeleteSelection {
    contents_map: Option<HashMap<PathBuf, StateDir>>,
}

/// Command used to delete files. Considers all selected items at the time of creating the struct.
impl DeleteSelection {
    pub fn new(ctx: AppContext) -> Self {
        let affected_files = ctx.explorer_table.get_selected_files();
        let contents_map = affected_files.map(|files| {
            files
                .iter()
                .map(|f| (f.clone(), read_to_state_dir(f.to_owned())))
                .collect::<HashMap<PathBuf, StateDir>>()
        });
        Self {
            contents_map: contents_map.clone(),
        }
    }
}

fn remove_path(path: PathBuf) {
    match path.is_dir() {
        true => {
            fs::remove_dir_all(&path)
                .expect(format!("Failed to delete {}", path.to_str().unwrap()).as_str());
        }
        false => {
            fs::remove_file(&path)
                .expect(format!("Failed to delete {}", path.to_str().unwrap()).as_str());
        }
    }
}

impl Command for DeleteSelection {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &self.contents_map {
            Some(contents) => {
                contents.keys().for_each(|f| remove_path(f.clone()));
            }
            None => {}
        };
        None
    }

    fn undo(&mut self, app: &mut App) -> Option<Action> {
        match &self.contents_map {
            Some(contents) => {
                let _ = contents
                    .values()
                    .map(|state_dir| write_state_dir(state_dir.to_owned()))
                    .collect::<Result<Vec<()>, std::io::Error>>();
                return None;
            }
            None => {}
        };
        None
    }
    fn is_reversible(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use fs::File;

    use crate::action::ExplorerAction;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_change_directory() {
        let mut app = App::new().unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                PathBuf::from("tests/"),
            )));
        app.handle_new_actions();
        assert_eq!(
            app.explorer_table.get_current_path(),
            PathBuf::from("tests/")
        );
    }

    #[test]
    fn test_select_up() {
        let mut app = App::new().unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                PathBuf::from("tests/"),
            )));
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::SelectDown));
        app.handle_new_actions();
        assert_eq!(app.explorer_table.get_selected(), Some(1));
    }
    #[test]
    fn test_select_down() {
        let mut app = App::new().unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                PathBuf::from("tests/"),
            )));
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::SelectDown));
        app.handle_new_actions();
        assert_eq!(app.explorer_table.get_selected(), Some(1));
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::SelectUp));
        app.handle_new_actions();
        assert_eq!(app.explorer_table.get_selected(), Some(0));
    }

    #[test]
    fn test_update_search_query() {
        let mut app = App::new().unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                PathBuf::from("tests/"),
            )));
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::UpdateSearchQuery(
                "test_query".into(),
            )));
        app.handle_new_actions();
        assert_eq!(
            app.explorer_table.get_search_phrase(),
            Some(String::from("test_query"))
        );
    }
    #[test]
    fn test_clear_search_query() {
        let mut app = App::new().unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                PathBuf::from("tests/"),
            )));
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::UpdateSearchQuery(
                "test_query".into(),
            )));
        app.handle_new_actions();
        assert_eq!(
            app.explorer_table.get_search_phrase(),
            Some(String::from("test_query"))
        );
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ClearSearchQuery));
        app.handle_new_actions();
        assert_eq!(app.explorer_table.get_search_phrase(), None)
    }

    #[test]
    fn test_parent_directory() {
        let mut app = App::new().unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                PathBuf::from("tests/folder_1"),
            )));
        app.handle_new_actions();
        assert_eq!(
            app.explorer_table.get_current_path(),
            PathBuf::from("tests/folder_1")
        );

        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ParentDirectory));
        app.handle_new_actions();

        assert_eq!(
            app.explorer_table.get_current_path(),
            PathBuf::from("tests")
        );
    }
    #[test]
    fn test_delete() {
        let mut app = App::new().unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                PathBuf::from("tests/folder_1"),
            )));
        app.handle_new_actions();
        let mut delete_selection = DeleteSelection::new(app.get_app_context());
        let before = delete_selection.contents_map.clone();

        delete_selection.execute(&mut app);
        thread::sleep(Duration::from_secs(5));
        let _ = delete_selection.undo(&mut app);

        let mut duplicate_selection = DeleteSelection::new(app.get_app_context());
        let after = duplicate_selection.contents_map.clone();
        assert_eq!(
            before.unwrap().values().collect::<Vec<_>>(),
            after.unwrap().values().collect::<Vec<_>>()
        );
    }
}
