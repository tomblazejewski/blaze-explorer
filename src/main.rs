use ratatui::{
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Constraint,
    prelude::*,
    widgets::{Row, Table},
};
use std::{
    error::Error,
    fs,
    io::{self, stdout},
};

fn obtain_filenames_table<'a>() -> io::Result<Option<Table<'a>>> {
    let paths = fs::read_dir("./").unwrap();

    let str_paths = paths
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    let file_name_row = Row::new(str_paths);
    let rows = [file_name_row];
    let widths = [Constraint::Length(5)];
    Ok(Some(Table::new(rows, widths)))
}

fn main() -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    loop {
        let t = obtain_filenames_table().unwrap().unwrap();
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(t, area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen);
    disable_raw_mode()?;
    Ok(())
}
