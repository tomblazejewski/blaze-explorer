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
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppAction {
    Quit,
    SwitchMode(Mode),
    CancelKeybind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextAction {
    InsertKey(char),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    ExplorerAct(ExplorerAction),
    AppAct(AppAction),
    TextAct(TextAction),
    Noop,
}
