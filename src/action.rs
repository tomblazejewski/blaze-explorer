use std::path::PathBuf;

use ratatui::crossterm::event::KeyEvent;

use crate::mode::Mode;

#[derive(Clone, Debug)]
pub enum Action {
    Quit,
    ChangeDirectory(PathBuf),
    ParentDirectory,
    Key(KeyEvent),
    Noop,
    SelectUp,
    SelectDown,
    EscapeSequence,
    SelectDirectory,
    ClearAndKey(KeyEvent),
    SwitchMode(Mode),
}
