use std::path::PathBuf;

use ratatui::crossterm::event::KeyEvent;

use crate::mode::Mode;

#[derive(Clone, Debug)]
pub enum ExplorerAction {
    ChangeDirectory(PathBuf),
    ParentDirectory,
    SelectUp,
    SelectDown,
    SelectDirectory,
}

#[derive(Clone, Debug)]
pub enum AppAction {
    Quit,
    SwitchMode(Mode),
}

#[derive(Clone, Debug)]
pub enum KeyAction {
    Key(KeyEvent),
    ClearAndKey(KeyEvent),
    EscapeSequence,
}

#[derive(Clone, Debug)]
pub enum Action {
    ExplorerAct(ExplorerAction),
    AppAct(AppAction),
    KeyAct(KeyAction),
    Noop,
}
