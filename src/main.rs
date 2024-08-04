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
mod components;
fn main() -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut app = App::new().unwrap();
    app.update_path(String::from("./"));
    let _ = app.run();

    Ok(())
}
