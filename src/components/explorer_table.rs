use std::fs;

use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::*,
    style::{palette::tailwind, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::action::Action;

use super::Component;

pub struct ExplorerTable {
    state: TableState,
    elements_list: Vec<String>,
}

impl ExplorerTable {
    pub fn new() -> Self {
        Self {
            state: TableState::default().with_selected(0),
            elements_list: Vec::new(),
        }
    }

    pub fn update_elements(&mut self, path: String) {
        let paths = fs::read_dir(path).unwrap();

        let str_paths = paths
            .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        self.elements_list = str_paths;
        self.state = TableState::default().with_selected(0);
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.elements_list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.elements_list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl Component for ExplorerTable {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let str_paths = self.elements_list.clone();
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

        frame.render_stateful_widget(t, area, &mut self.state);
        Ok(())
    }
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ChangeDirectory(path) => {
                self.update_elements(path);
            }
            Action::SelectUp => self.previous(),
            Action::SelectDown => self.next(),
            _ => {}
        }
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('j') => {
                self.update(Action::SelectDown);
            }
            KeyCode::Char('k') => {
                self.update(Action::SelectUp);
            }
            _ => {}
        };
        Ok(None)
    }
}
