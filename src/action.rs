use ratatui::crossterm::event::KeyEvent;

#[derive(Clone)]
pub enum Action {
    Quit,
    ChangeDirectory(String),
    Key(KeyEvent),
    Noop,
    SelectUp,
    SelectDown,
}
