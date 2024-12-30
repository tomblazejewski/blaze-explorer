use libloading::Library;
use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use blaze_explorer_core::{app::App, mode::Mode, plugin::Plugin};

pub fn fetch_plugins(_app: &mut App, lib: &Library) -> Vec<Box<dyn Plugin>> {
    let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();

    let mut telescope_bindings = HashMap::new();
    let plugin_getter: libloading::Symbol<
        extern "Rust" fn(HashMap<(Mode, Vec<KeyEvent>), String>) -> Box<dyn Plugin>,
    > = unsafe { lib.get(b"get_telescope_plugin").unwrap() };
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
    let telescope_plugin = plugin_getter(telescope_bindings);
    plugins.push(telescope_plugin);
    plugins
}
