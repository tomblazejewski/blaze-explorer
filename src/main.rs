use ratatui::{
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Constraint,
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table},
};
use std::error::Error;
use std::io::stdout;
use style::palette::tailwind;
mod app;
use app::App;

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
                        KeyCode::Char('k') => app.previous(),
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
    let widths = [Constraint::Percentage(60)];
    let header = ["Name"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);
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
        .highlight_style(selected_style)
        .header(header);

    f.render_stateful_widget(t, area, &mut app.table_state);
}
