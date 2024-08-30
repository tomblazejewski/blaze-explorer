use std::path::PathBuf;

use ratatui::crossterm::event::KeyEvent;

use crate::mode::Mode;

#[derive(Clone, Debug, PartialEq)]
pub enum ExplorerAction {
    ChangeDirectory(PathBuf),
    ParentDirectory,
    SelectUp,
    SelectDown,
    SelectDirectory,
    UpdateSearchQuery(String),
    ClearSearchQuery,
    NextSearchResult,
    ShowInFolder(PathBuf),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppAction {
    Quit,
    SwitchMode(Mode),
    CancelKeybind,
    ConfirmSearchQuery,
    ConfirmCommand,
    OpenPopup,
    ShowInFolder(PathBuf),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextAction {
    InsertKey(char),
    EraseText,
    DropKey,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    ExplorerAct(ExplorerAction),
    AppAct(AppAction),
    TextAct(TextAction),
    Noop,
    TelescopeAct(TelescopeAction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TelescopeAction {
    ConfirmResult,
    PushSearchChar(char),
    DropSearchChar,
    NextResult,
    PreviousResult,
    Quit,
    EraseText,
    UpdateSearchQuery(String),
}
