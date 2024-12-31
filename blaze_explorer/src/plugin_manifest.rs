use libloading::{Library, Symbol};
use std::collections::HashMap;

use ratatui::crossterm::event::KeyEvent;

use blaze_explorer_lib::{app::App, mode::Mode, plugin::Plugin};

type BindingsMap = HashMap<(Mode, Vec<KeyEvent>), String>;
fn collect_plugin(lib: &Library, custom_bindings: Option<BindingsMap>) -> Box<dyn Plugin> {
    let custom_bindings = custom_bindings.unwrap_or_default();
    let get_plugin: Symbol<extern "Rust" fn(BindingsMap) -> Box<dyn Plugin>> =
        unsafe { lib.get(b"get_plugin").unwrap() };
    get_plugin(custom_bindings)
}

pub fn fetch_plugins(
    _app: &mut App,
    lib_map: &HashMap<String, Library>,
) -> HashMap<String, Box<dyn Plugin>> {
    let mut plugins = HashMap::new();
    for lib in lib_map.values() {
        let plugin = collect_plugin(lib, None);
        let display = plugin.display_details();
        plugins.insert(display, plugin);
    }

    plugins
}
