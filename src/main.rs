use blaze_explorer_core::app::{App, ExitResult};
use blaze_explorer_core::logging::initialize_logging;
mod plugin_manifest;
use directories::ProjectDirs;
use libloading::os::windows::Library;
use libloading::Library;
use plugin_manifest::fetch_plugins;
use ratatui::crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::process::Command;
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

fn collect_libs(lib_names: Vec<String>) -> Vec<Library> {
    let plugin_dir = ProjectDirs::from("", "", "blaze_explorer_plugins");
    match plugin_dir {
        None => {
            info!("No plugin folder found. Running without plugins");
            vec![]
        }
        Some(plugin_dir) => {
            let mut libs = vec![];
            for lib_name in lib_names {
                libs = Vec::new();
                let lib_path = plugin_dir.data_dir();
                let path_ending = format!("{}/target/debug/{}.dll", lib_name, lib_name);
                let dll_path = lib_path.join(path_ending);
                match dll_path.exists() {
                    true => {
                        let lib = unsafe { Library::new(dll_path.to_str().unwrap()).unwrap() };
                        libs.push(lib)
                    }
                    false => {
                        info!("Plugin {} not found/not compiled", lib_name);
                    }
                }
            }
            libs
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    initialize_logging()?;
    let mut app = App::new().unwrap();
    let lib = unsafe {
        Library::new(
            "C:/Users/tomas/OneDrive/Projects/blaze_telescope/target/debug/blaze_telescope.dll",
        )
        .unwrap()
    };
    let plugins = fetch_plugins(&mut app, &lib);
    app.attach_plugins(plugins);
    let mut cold_start = true;
    loop {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let result = app.run(cold_start);
        cold_start = false;
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        match result {
            Ok(ExitResult::Quit) => break,
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
    std::mem::drop(app);
    Ok(())
}
