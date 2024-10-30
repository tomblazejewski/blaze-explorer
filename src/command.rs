use chrono::offset;
use directories::ProjectDirs;

use crate::action::{AppAction, PopupType};
use crate::app::ExitResult;
use crate::components::explorer_manager::SplitDirection;
use crate::popup::ActionInput;
use crate::{action::Action, line_entry::LineEntry};
use std::fmt::Debug;
use std::process::Command as ProcessCommand;
use std::{collections::HashMap, fs, path::PathBuf};
use std::{fmt, io};

use crate::{
    app::App,
    mode::Mode,
    popup::{PopUp, TelescopeWindow},
    telescope::AppContext,
};

pub trait Command: CommandClone {
    fn execute(&mut self, app: &mut App) -> Option<Action>;
    fn undo(&mut self, _app: &mut App) -> Option<Action> {
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
    pub fn new(mut _ctx: AppContext, path: PathBuf) -> Self {
        Self { new_path: path }
    }
}

impl Command for ChangeDirectory {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.update_path(self.new_path.clone(), None);
        None
    }
}

#[derive(Clone, Debug)]
pub struct ParentDirectory {}

impl ParentDirectory {
    pub fn new(mut _ctx: AppContext) -> Self {
        ParentDirectory {}
    }
}

impl Command for ParentDirectory {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.go_up();
        None
    }
}

#[derive(Clone, Debug)]
pub struct SelectUp {}

impl SelectUp {
    pub fn new(_ctx: AppContext) -> Self {
        Self {}
    }
}
impl Command for SelectUp {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.previous();
        None
    }
}

#[derive(Clone, Debug)]
pub struct SelectDown {}

impl SelectDown {
    pub fn new(_ctx: AppContext) -> Self {
        Self {}
    }
}
impl Command for SelectDown {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.next();
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
            path: ctx.explorer_manager.select_directory(),
        }
    }
}

impl Command for SelectDirectory {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &self.path {
            Some(path) => match path.is_dir() {
                true => app.update_path(path.clone(), None),
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
    pub fn new(_ctx: AppContext, query: String) -> Self {
        Self { query }
    }
}

impl Command for UpdateSearchQuery {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.update_search_query(self.query.clone());
        None
    }
}

#[derive(Clone, Debug)]
pub struct ClearSearchQuery {}

impl ClearSearchQuery {
    pub fn new(_ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for ClearSearchQuery {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.clear_search_query();
        None
    }
}

#[derive(Clone, Debug)]
pub struct NextSearchResult {}

impl NextSearchResult {
    pub fn new(_ctx: AppContext) -> Self {
        Self {}
    }
}
impl Command for NextSearchResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.next_search_result();
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
            current_file_path: ctx.explorer_manager.select_directory().unwrap().clone(),
            target_path: path,
        }
    }
}

impl Command for ShowInFolder {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.popup = PopUp::None;
        app.explorer_manager
            .show_in_folder(self.target_path.clone());
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
    pub fn new(_ctx: AppContext, mode: Mode) -> Self {
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
pub struct ConfirmCommand {
    command: String,
}

impl ConfirmCommand {
    pub fn new(ctx: AppContext) -> Self {
        Self {
            command: ctx.command,
        }
    }
}
impl Command for ConfirmCommand {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        if !self.command.is_empty() && self.command.chars().nth(0).unwrap() == '!' {
            Some(Action::AppAct(crate::action::AppAction::TerminalCommand(
                self.command[1..].to_string(),
            )))
        } else {
            app.execute_command(self.command.clone())
        }
    }
}

#[derive(Clone, Debug)]
pub struct DisplayMessage {
    message: String,
}

impl DisplayMessage {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
impl Command for DisplayMessage {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.command_line.command_line_message(self.message.clone());
        None
    }
}

#[derive(Clone, Debug)]
pub struct OpenPopup {
    popup: PopupType,
}

impl OpenPopup {
    pub fn new(popup: PopupType) -> Self {
        Self { popup }
    }
}
impl Command for OpenPopup {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &self.popup {
            PopupType::None => app.popup = PopUp::None,
            PopupType::Telescope => {
                app.popup = PopUp::TelescopePopUp(TelescopeWindow::new(app.get_app_context()))
            }
            PopupType::Rename => {
                app.popup =
                    PopUp::InputPopUp(ActionInput::<RenameActive>::new(app.get_app_context()))
            }
        }
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
    fn execute(&mut self, _app: &mut App) -> Option<Action> {
        None
    }
}

#[derive(Clone)]
pub struct DeleteSelection {
    affected_files: Option<Vec<PathBuf>>,
    backup_path: Option<HashMap<PathBuf, PathBuf>>,
}

/// Command used to delete files. Considers all selected items at the time of creating the struct.
impl DeleteSelection {
    pub fn new(mut ctx: AppContext) -> Self {
        let affected_files = ctx.explorer_manager.get_selected_files();
        Self {
            affected_files,
            backup_path: None,
        }
    }
}
impl Command for DeleteSelection {
    fn execute(&mut self, _app: &mut App) -> Option<Action> {
        match &self.affected_files {
            Some(contents) => {
                match &self.backup_path {
                    None => {
                        let contents_map = contents
                            .iter()
                            .map(|f| (f.to_owned(), backup_dir()))
                            .collect::<HashMap<PathBuf, PathBuf>>();
                        self.backup_path = Some(contents_map);
                    }
                    Some(_contents) => {}
                }
                let _ = contents
                    .iter()
                    .map(|f| {
                        let backup_path = self.backup_path.as_ref().unwrap().get(f).unwrap();
                        remove_path(f, backup_path)
                    })
                    .collect::<Vec<()>>();
            }
            None => {}
        };
        None
    }

    fn undo(&mut self, _app: &mut App) -> Option<Action> {
        match &self.backup_path {
            Some(contents) => {
                let _ = contents
                    .iter()
                    .map(|(original_path, backup_path)| move_path(backup_path, original_path))
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

impl Debug for DeleteSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeleteSelection")
            .field("to delete", &self.affected_files)
            .field("backup_path", &self.backup_path)
            .finish()
    }
}

fn move_recursively(from: &PathBuf, to: &PathBuf) -> io::Result<()> {
    // Create the destination directory
    fs::create_dir_all(to)?;

    // Iterate over the directory entries
    if !from.is_dir() {
        let dst_path = to.join(from.file_name().unwrap());
        fs::rename(from, &dst_path)?;
        return Ok(());
    }
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        let src_path = entry.path();
        let dst_path = to.join(entry.file_name());

        // If the entry is a directory, call the function recursively
        if file_type.is_dir() {
            move_recursively(&src_path, &dst_path)?;
        } else {
            // If it's a file, copy it
            fs::rename(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
fn move_path(from: &PathBuf, to: &PathBuf) -> Result<(), std::io::Error> {
    //create the to directory
    fs::rename(from, to)?;
    Ok(())
}
fn remove_path(path: &PathBuf, backup_path: &PathBuf) {
    //write contents to a file so this can be recovered later on
    move_recursively(path, &PathBuf::from(backup_path.clone())).unwrap();
    if path.is_dir() {
        fs::remove_dir_all(path).unwrap();
    }
}

fn backup_dir() -> PathBuf {
    let mut backup_name = format!(
        "backup_{}",
        offset::Local::now().format("%d_%h_%Y_%H_%M_%S_%3f")
    );
    backup_name += ".blzbkp";
    let proj_dir = ProjectDirs::from("", "", "blaze_explorer").unwrap();
    proj_dir.cache_dir().join(backup_name)
}

#[derive(Clone, Debug)]
pub struct RenameActive {
    pub first_path: PathBuf,
    pub second_path: Option<PathBuf>,
    reversible: bool,
}

/// Rename currently selected file
impl RenameActive {
    // pub fn new(ctx: AppContext, new_name: String) -> Self {
    //     let first_path = ctx.explorer_manager.select_directory().unwrap();
    //     let second_path = first_path.parent().unwrap().join(new_name);
    //     Self {
    //         first_path,
    //         second_path,
    //         reversible: false,
    //     }
    // }

    pub fn default(mut ctx: AppContext) -> Self {
        let first_path = ctx.explorer_manager.select_directory().unwrap();
        let second_path = None;
        Self {
            first_path,
            second_path,
            reversible: false,
        }
    }
    pub fn update_command_context(&mut self, new_path: String) {
        let new_path = self.first_path.parent().unwrap().join(new_path);
        self.second_path = Some(new_path);
        self.reversible = true;
    }
}

fn rename_path(first_path: PathBuf, second_path: PathBuf) {
    Result::expect(
        fs::rename(first_path.clone(), second_path),
        format!("Failed to rename {}", first_path.to_str().unwrap()).as_str(),
    );
}

impl Command for RenameActive {
    fn execute(&mut self, _app: &mut App) -> Option<Action> {
        rename_path(self.first_path.clone(), self.second_path.clone().unwrap());
        self.reversible = true;
        None
    }

    fn undo(&mut self, _app: &mut App) -> Option<Action> {
        rename_path(self.second_path.clone().unwrap(), self.first_path.clone());
        None
    }
    fn is_reversible(&self) -> bool {
        self.reversible
    }
}
#[derive(Clone, Debug)]
pub struct TerminalCommand {
    command: String,
}

impl TerminalCommand {
    pub fn new(_ctx: AppContext, command: String) -> Self {
        Self { command }
    }
}

fn parse_shell_command(input: String) -> (String, Option<Vec<String>>) {
    let chars = input.chars().peekable();
    let mut command = String::new();
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;

    for c in chars {
        match c {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current_arg.is_empty() {
                    if command.is_empty() {
                        command = current_arg.clone();
                    } else {
                        args.push(current_arg.clone());
                    }
                    current_arg.clear();
                }
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    if !current_arg.is_empty() {
        if command.is_empty() {
            command = current_arg;
        } else {
            args.push(current_arg);
        }
    }

    // Return the command and None if no arguments were parsed
    if args.is_empty() {
        (command, None)
    } else {
        (command, Some(args))
    }
}

impl Command for TerminalCommand {
    fn execute(&mut self, _app: &mut App) -> Option<Action> {
        let command_args = parse_shell_command(self.command.clone());
        let output = match command_args {
            (command, Some(args)) => ProcessCommand::new(command).args(args).output(),
            (command, None) => ProcessCommand::new(command).output(),
        };
        let output_message = match output {
            Ok(output) => {
                let output = output.stdout;
                String::from_utf8(output).unwrap()
            }
            Err(err) => {
                format!("Failed to execute command '{}': {}", self.command, err)
            }
        };

        Some(Action::AppAct(crate::action::AppAction::DisplayMessage(
            output_message,
        )))
    }
}
#[derive(Clone, Debug)]
pub struct OpenNeovimHere {
    path: PathBuf,
}

impl OpenNeovimHere {
    pub fn new(mut ctx: AppContext) -> Self {
        let path = ctx.explorer_manager.get_current_path();
        Self { path }
    }
}

impl Command for OpenNeovimHere {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.exit_status = Some(ExitResult::OpenNeovim(self.path.clone()));
        app.should_quit = true;
        None
    }
}

#[derive(Clone, Debug)]
pub struct SplitVertically {}

impl SplitVertically {
    pub fn new(mut _ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for SplitVertically {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.split_vertically_action();
        None
    }
}
#[derive(Clone, Debug)]
pub struct SplitHorizontally {}

impl SplitHorizontally {
    pub fn new(mut _ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for SplitHorizontally {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.split_horizontally_action();
        None
    }
}

#[derive(Clone, Debug)]
pub struct DeleteSplit {}

impl DeleteSplit {
    pub fn new(mut _ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for DeleteSplit {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        let should_quit = app.explorer_manager.delete_split();
        app.should_quit = should_quit;
        Some(Action::AppAct(AppAction::SwitchMode(Mode::Normal)))
    }
}
#[derive(Clone, Debug)]
pub struct FocusUp {}

impl FocusUp {
    pub fn new(mut _ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for FocusUp {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.move_focus(SplitDirection::Up);
        None
    }
}
#[derive(Clone, Debug)]
pub struct FocusDown {}

impl FocusDown {
    pub fn new(mut _ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for FocusDown {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.move_focus(SplitDirection::Down);
        None
    }
}
#[derive(Clone, Debug)]
pub struct FocusLeft {}

impl FocusLeft {
    pub fn new(mut _ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for FocusLeft {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.move_focus(SplitDirection::Left);
        None
    }
}
#[derive(Clone, Debug)]
pub struct FocusRight {}

impl FocusRight {
    pub fn new(mut _ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for FocusRight {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.move_focus(SplitDirection::Right);
        None
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

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
        let _ = app.handle_new_actions();
        assert_eq!(
            app.explorer_manager.get_current_path(),
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
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_selected(), Some(1));
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
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_selected(), Some(1));
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::SelectUp));
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_selected(), Some(0));
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
        let _ = app.handle_new_actions();
        assert_eq!(
            app.explorer_manager.get_search_phrase(),
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
        let _ = app.handle_new_actions();
        assert_eq!(
            app.explorer_manager.get_search_phrase(),
            Some(String::from("test_query"))
        );
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ClearSearchQuery));
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_search_phrase(), None)
    }

    #[test]
    fn test_parent_directory() {
        let mut app = App::new().unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                PathBuf::from("tests/folder_1"),
            )));
        let _ = app.handle_new_actions();
        assert_eq!(
            app.explorer_manager.get_current_path(),
            PathBuf::from("tests/folder_1")
        );

        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ParentDirectory));
        let _ = app.handle_new_actions();

        assert_eq!(
            app.explorer_manager.get_current_path(),
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
        let _ = app.handle_new_actions();
        let mut delete_selection = DeleteSelection::new(app.get_app_context());
        let before = delete_selection.affected_files.clone();

        delete_selection.execute(&mut app);
        thread::sleep(Duration::from_secs(5));
        let _ = delete_selection.undo(&mut app);

        let duplicate_selection = DeleteSelection::new(app.get_app_context());
        let after = duplicate_selection.affected_files.clone();
        assert_eq!(before, after);
    }

    #[test]
    fn test_parse_shell_command() {
        let command_string = "git".to_string();
        let (command, args) = parse_shell_command(command_string);
        assert_eq!(command, "git");
        assert_eq!(args, None);

        let command_string = "git blame".to_string();
        let (command, args) = parse_shell_command(command_string);
        assert_eq!(command, "git");
        assert_eq!(args, Some(vec!["blame".to_string()]));

        let command_string = format!(r#"git commit -m "{}""#, "commit message");
        let (command, args) = parse_shell_command(command_string);
        assert_eq!(command, "git");
        assert_eq!(
            args,
            Some(vec![
                "commit".to_string(),
                "-m".to_string(),
                "commit message".to_string()
            ])
        );
    }
}
