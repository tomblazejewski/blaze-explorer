use ratatui::{
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Constraint,
    prelude::*,
    widgets::{Block, Borders, Row, Table, TableState},
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
    table_state: TableState,
}

impl App {
    fn new() -> Self {
        Self {
            current_path: String::from("./"),
            elements_list: Vec::new(),
            selected_elements_list: Vec::new(),
            table_state: TableState::default().with_selected(0),
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

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.elements_list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut app = App::new();
    app.update_path(String::from("./"));
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') => app.next(),
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen);
    disable_raw_mode()?;
    Ok(())
}
fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(f.size());

    render_table(f, app, chunks[1]);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let str_paths = app.elements_list.clone();
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

    f.render_stateful_widget(t, area, &mut app.table_state);
}
