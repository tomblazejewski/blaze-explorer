use crate::{action::Action, line_entry::LineEntry};
use std::{error::Error, path::PathBuf};

use crate::{
    app::App,
    mode::Mode,
    popup::{PopUp, PopUpWindow},
    telescope::AppContext,
};

pub trait Command {
    fn execute(&self, app: &mut App) -> Option<Action>;
    fn undo(&self, app: &mut App) -> Option<Action> {
        None
    }
    fn is_revertable(&self) -> bool {
        false
    }
}

pub struct ChangeDirectory {
    new_path: PathBuf,
}

impl ChangeDirectory {
    pub fn new(mut ctx: AppContext, path: PathBuf) -> Self {
        Self { new_path: path }
    }
}

impl Command for ChangeDirectory {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.explorer_table.update_path(self.new_path.clone());
        None
    }
}

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
    fn execute(&self, app: &mut App) -> Option<Action> {
        match &self.new_path {
            Some(new_path) => app.explorer_table.update_path(new_path.clone()),
            _ => {}
        }
        None
    }
}

pub struct SelectUp {}

impl SelectUp {
    pub fn new(ctx: AppContext) -> Self {
        Self {}
    }
}
impl Command for SelectUp {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.explorer_table.previous();
        None
    }
}

pub struct SelectDown {}

impl SelectDown {
    pub fn new(ctx: AppContext) -> Self {
        Self {}
    }
}
impl Command for SelectDown {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.explorer_table.next();
        None
    }
}

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
    fn execute(&self, app: &mut App) -> Option<Action> {
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

pub struct UpdateSearchQuery {
    query: String,
}

impl UpdateSearchQuery {
    pub fn new(ctx: AppContext, query: String) -> Self {
        Self { query }
    }
}

impl Command for UpdateSearchQuery {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.explorer_table.update_search_query(self.query.clone());
        None
    }
}

pub struct ClearSearchQuery {}

impl ClearSearchQuery {
    pub fn new(ctx: AppContext) -> Self {
        Self {}
    }
}

impl Command for ClearSearchQuery {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.explorer_table.clear_search_query();
        None
    }
}

pub struct NextSearchResult {}

impl NextSearchResult {
    pub fn new(ctx: AppContext) -> Self {
        Self {}
    }
}
impl Command for NextSearchResult {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.explorer_table.next_search_result();
        None
    }
}

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
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup = PopUp::None;
        app.explorer_table.show_in_folder(self.target_path.clone());
        None
    }
}

pub struct Quit {}

impl Quit {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for Quit {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.should_quit = true;
        None
    }
}

pub struct SwitchMode {
    mode: Mode,
}

impl SwitchMode {
    pub fn new(ctx: AppContext, mode: Mode) -> Self {
        Self { mode }
    }
}
impl Command for SwitchMode {
    fn execute(&self, app: &mut App) -> Option<Action> {
        match &self.mode {
            Mode::Normal => app.enter_normal_mode(),
            Mode::Command => app.enter_command_mode(),
            Mode::Search => app.enter_search_mode(),
        }
        None
    }
}

pub struct ConfirmSearchQuery {}

impl ConfirmSearchQuery {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for ConfirmSearchQuery {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.confirm_search_query()
    }
}

pub struct ConfirmCommand {}

impl ConfirmCommand {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for ConfirmCommand {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.execute_command()
    }
}

pub struct OpenPopup {}

impl OpenPopup {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for OpenPopup {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup = PopUp::TelescopePopUp(PopUpWindow::new(app.get_app_context()));
        None
    }
}

pub struct InsertKey {
    ch: char,
}

impl InsertKey {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }
}
impl Command for InsertKey {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.command_line.append_char(self.ch);
        None
    }
}

pub struct EraseText {}

impl EraseText {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for EraseText {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.command_line.clear_contents();
        None
    }
}

pub struct DropKey {}

impl DropKey {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for DropKey {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.command_line.remove_char()
    }
}

pub struct TelescopeConfirmResult {}

impl TelescopeConfirmResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeConfirmResult {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup.confirm_result()
    }
}

pub struct TelescopeNextResult {}

impl TelescopeNextResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeNextResult {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup.next_result();
        None
    }
}

pub struct TelescopePreviousResult {}

impl TelescopePreviousResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopePreviousResult {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup.previous_result();
        None
    }
}

pub struct TelescopeUpdateSearchQuery {
    query: String,
}

impl TelescopeUpdateSearchQuery {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}
impl Command for TelescopeUpdateSearchQuery {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup.update_search_query(self.query.clone());
        None
    }
}

pub struct TelescopePushSearchChar {
    ch: char,
}

impl TelescopePushSearchChar {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }
}

impl Command for TelescopePushSearchChar {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup.push_search_char(self.ch)
    }
}

pub struct TelescopeDropSearchChar {}

impl TelescopeDropSearchChar {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeDropSearchChar {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup.drop_search_char()
    }
}

pub struct TelescopeQuit {}

impl TelescopeQuit {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeQuit {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup.quit();
        None
    }
}

pub struct TelescopeEraseText {}

impl TelescopeEraseText {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeEraseText {
    fn execute(&self, app: &mut App) -> Option<Action> {
        app.popup.erase_text()
    }
}

pub struct Noop {}

impl Noop {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for Noop {
    fn execute(&self, app: &mut App) -> Option<Action> {
        None
    }
}
