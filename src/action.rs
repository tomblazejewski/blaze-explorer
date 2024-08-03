use ratatui::crossterm::event::KeyEvent;

pub enum Action {
    Quit,
    ChangeDirectory(String),
    Key(KeyEvent),
    Noop,
    SelectUp,
    SelectDown,
}
