use libloading::{Error, Library, Symbol};
use std::collections::HashMap;
use tracing::info;

use ratatui::crossterm::event::KeyEvent;

use blaze_explorer_lib::{app::App, mode::Mode, plugin::Plugin};

type BindingsMap = HashMap<(Mode, Vec<KeyEvent>), String>;
fn collect_plugin(lib: &Library, custom_bindings: Option<BindingsMap>) -> Option<Box<dyn Plugin>> {
    let custom_bindings = custom_bindings.unwrap_or_default();
    let get_plugin: Result<Symbol<extern "Rust" fn(BindingsMap) -> Box<dyn Plugin>>, Error> =
        unsafe { lib.get(b"get_plugin") };
    match get_plugin {
        Ok(plugin) => Some(plugin(custom_bindings)),
        Err(err) => {
            panic!("Failed to load plugin: {}", err);
        }
    }
}

pub fn fetch_plugins(lib_map: &HashMap<String, Library>) -> HashMap<String, Box<dyn Plugin>> {
    let mut plugins = HashMap::new();
    for lib in lib_map.values() {
        let plugin = collect_plugin(lib, None);
        if let Some(plugin) = plugin {
            let display = plugin.display_details();
            plugins.insert(display, plugin);
        };
    }

    plugins
}
