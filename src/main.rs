#![feature(reentrant_lock)]
#![feature(str_split_remainder)]
use logging::initialize_logging;
use plugin_manifest::fetch_plugins;
use ratatui::crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::process::Command;
use std::{env::set_current_dir, io::stdout};
use std::{error::Error, path::PathBuf};
mod app;
use app::{App, ExitResult};
mod action;
mod app_input_machine;
mod command;
mod components;
mod explorer_helpers;
mod flash_input_machine;
mod function_helpers;
mod git_helpers;
mod history_stack;
mod input_machine;
mod line_entry;
mod logging;
mod mode;
mod plugin;
//mod popup;
mod plugin_manifest;
mod simple_input_machine;
mod telescope;
mod telescope_query;
mod themes;
mod tools;

fn bring_app_back(app: &mut App) {
    app.exit_status = None;
    app.should_quit = false;
}
fn open_neovim(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    set_current_dir(path)?;
    let _output = Command::new("nvim").status()?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    initialize_logging()?;
    let mut app = App::new().unwrap();
    let plugins = fetch_plugins(&mut app);
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
    Ok(())
}
