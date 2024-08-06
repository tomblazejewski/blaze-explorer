use ratatui::crossterm::event::KeyEvent;

#[derive(Clone)]
pub enum Action {
    Quit,
    ChangeDirectory(String),
    ParentDirectory,
    Key(KeyEvent),
    Noop,
    SelectUp,
    SelectDown,
    EscapeSequence,
    SelectDirectory,
}
