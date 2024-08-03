use ratatui::crossterm::event::KeyEvent;

pub enum Action {
    Quit,
    Key(KeyEvent),
    Noop,
}
