use std::path::PathBuf;

use ratatui::crossterm::event::KeyEvent;

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
}
