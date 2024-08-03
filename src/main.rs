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
    let widths = str_paths
        .iter()
        .map(|_path_entry| Constraint::Length(15))
        .collect::<Vec<Constraint>>();
    let file_name_row = Row::new(str_paths);
    let rows = [file_name_row];
    let t = Table::new(rows, widths).style(Style::new().blue());
    Ok(Some(t))
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
