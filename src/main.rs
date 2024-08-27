use logging::initialize_logging;
use ratatui::{
    crossterm::{
        terminal::{enable_raw_mode, EnterAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
};
use std::error::Error;
use std::io::stdout;
mod app;
use app::App;
mod action;
mod action_agent;
mod components;
mod focus;
mod input_machine;
mod key_handler;
mod line_entry;
mod logging;
mod mode;
mod popup;
mod telescope;
mod telescope_query;
mod themes;
mod tools;
fn main() -> Result<(), Box<dyn Error>> {
    initialize_logging()?;
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut app = App::new().unwrap();
    let _ = app.run();

    Ok(())
}
