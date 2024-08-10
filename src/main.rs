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
mod key_combination;
use app::App;
mod action;
mod components;
mod logging;
mod mode;
fn main() -> Result<(), Box<dyn Error>> {
    initialize_logging()?;
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut app = App::new().unwrap();
    let _ = app.run();

    Ok(())
}
