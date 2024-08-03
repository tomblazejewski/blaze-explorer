use ratatui::{
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Constraint,
    prelude::*,
    widgets::{Block, Borders, Row, Table},
};
use std::{
    error::Error,
    fs,
    io::{self, stdout},
};
use style::palette::tailwind;

struct App {
    current_path: String,
    elements_list: Vec<String>,
    selected_elements_list: Vec<String>,
}

impl App {
    fn new() -> Self {
        Self {
            current_path: String::from("./"),
            elements_list: Vec::new(),
            selected_elements_list: Vec::new(),
        }
    }

    fn update_path(&mut self, path: String) {
        self.current_path = path.clone();
        let paths = fs::read_dir(path).unwrap();

        let str_paths = paths
            .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        self.elements_list = str_paths;
        self.selected_elements_list = Vec::new();
    }
}

fn obtain_filenames_table<'a>() -> io::Result<Option<Table<'a>>> {
    let paths = fs::read_dir("./").unwrap();

    let str_paths = paths
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    let widths = str_paths
        .iter()
        .map(|_path_entry| Constraint::Length(15))
        .collect::<Vec<Constraint>>();
    let rows = str_paths
        .into_iter()
        .map(|path_str| Row::new([path_str]))
        .collect::<Vec<Row>>();
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(tailwind::BLUE.c400);
    let t = Table::new(rows, widths)
        .style(Style::new().blue())
        .block(Block::new().borders(Borders::ALL))
        .highlight_style(selected_style);
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
