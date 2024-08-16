use ratatui::crossterm::event::KeyEvent;

/// A trait to handle pressing of keys
/// Examples include a class to handle key press when in normal mode
/// As well as a class to handle key press when in search mode/command mode
pub trait KeyHandler {
    fn append_key_event(&mut self, new_event: KeyEvent);
}
