use crate::{action::Action, line_entry::LineEntry};
use std::{error::Error, fs, io::Write, path::PathBuf};

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
    fn is_revertable(&self) -> bool {
        false
    }
}

pub trait CommandClone {
    fn clone_box(&self) -> Box<dyn Command>;
}

impl<T> CommandClone for T
where
    T: 'static + Command + Clone,
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ParentDirectory {
    old_path: PathBuf,
    new_path: Option<PathBuf>,
}

impl ParentDirectory {
    pub fn new(mut ctx: AppContext) -> Self {
        let current_path = ctx.explorer_table.select_directory().unwrap().clone();
        Self {
            old_path: current_path.clone(),
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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
                true => app.explorer_table.update_path(path.clone()),
                false => app.open_default(path.clone()),
            },
            None => {}
        }
        None
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Delete {
    path: PathBuf,
    contents: Option<Vec<u8>>,
    revertible: bool,
}

impl Delete {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            contents: None,
            revertible: false,
        }
    }
}

impl Command for Delete {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        if self.path.exists() {
            self.contents = Some(fs::read(&self.path).unwrap());
            fs::remove_file(&self.path).expect("Coult not delete file");
            self.revertible = true;
        }
        None
    }

    fn undo(&mut self, app: &mut App) -> Option<Action> {
        let mut file = fs::File::create(&self.path).expect("Failed to create the file");
        let backup_data = self.contents.clone().expect("No backup data");
        file.write_all(&backup_data);
        None
    }
}
