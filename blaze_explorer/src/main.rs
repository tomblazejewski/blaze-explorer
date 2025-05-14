use blaze_explorer_lib::app::{App, ExitResult};
use blaze_explorer_lib::logging::initialize_logging;
mod plugin_manifest;
use libloading::Library;
use plugin_manifest::fetch_plugins;
use ratatui::crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::rc::Rc;
use std::{env::set_current_dir, io::stdout};
use std::{error::Error, path::PathBuf};
use tracing::info;

fn bring_app_back(app: &mut App) {
    app.exit_status = None;
    app.should_quit = false;
}
fn open_neovim(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    set_current_dir(path)?;
    let _output = Command::new("nvim").status()?;
    Ok(())
}

fn collect_libs() -> HashMap<String, Rc<Library>> {
    let mut lib_map = HashMap::new();
    let plugins_folder_location = "../blaze_plugins";
    let paths = fs::read_dir(plugins_folder_location).unwrap();
    for path in paths {
        let lib_name = path
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let dll_path = PathBuf::from(format!(
            "../blaze_plugins/{}/target/debug/{}.dll",
            lib_name, lib_name
        ));
        match dll_path.exists() {
            true => {
                let lib = unsafe { Library::new(dll_path.to_str().unwrap()).unwrap() };
                lib_map.insert(lib_name, Rc::new(lib));
            }
            false => {
                info!("Plugin {} not found/not compiled", lib_name);
            }
        }
    }
    lib_map
}

fn main() -> Result<(), Box<dyn Error>> {
    initialize_logging()?;
    let lib_map = collect_libs();
    {
        let mut app = App::new().unwrap();
        let plugins = fetch_plugins(&lib_map);
        app.attach_plugins(&plugins);
        let mut cold_start = true;
        loop {
            stdout().execute(EnterAlternateScreen)?;
            enable_raw_mode()?;
            let result = app.run(cold_start);
            cold_start = false;
            stdout().execute(LeaveAlternateScreen)?;
            disable_raw_mode()?;
            match result {
                Ok(ExitResult::Quit) => {
                    let output_message = app.destruct();
                    println!("{}", output_message);
                    break;
                }
                Ok(ExitResult::OpenNeovim(path)) => {
                    open_neovim(&path)?;
                    bring_app_back(&mut app);
                }
                Err(e) => {
                    println!("{}", e);
                }
                _ => {}
            }
        }
        Ok(())
    }
}
