use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::App,
    mode::Mode,
    plugin::{telescope::Telescope, Plugin},
};

//Edit this function to set up the plugins
pub fn fetch_plugins(app: &mut App) -> Vec<Box<dyn Plugin>> {
    let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();

    let mut telescope_bindings = HashMap::new();
    telescope_bindings.insert(
        (
            Mode::Normal,
            vec![
                KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
            ],
        ),
        "OpenSFS".to_string(),
    );
    let telescope_plugin = Telescope::new(telescope_bindings);

    plugins.push(Box::new(telescope_plugin));

    plugins
}
