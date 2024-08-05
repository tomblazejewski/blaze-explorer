use std::{fs, path::Path, time::SystemTime};

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

pub struct FileData {
    filename: String,
    size: u64,
    modified: Option<SystemTime>,
}

pub(crate) const SUFFIXES: [&str; 5] = ["B", "K", "M", "G", "T"];
pub fn format_file_size(mut size: u64) -> String {
    for suffix in SUFFIXES {
        if size > 1024 {
            size /= 1024;
        } else {
            return format!("{}{}", size, suffix.to_string());
        }
    }
    String::from("0")
}
pub fn get_file_data(path: &String) -> Vec<FileData> {
    let paths = fs::read_dir(path).unwrap();
    let dir_entries = paths.map(|entry| entry.unwrap());
    let data = dir_entries
        .map(|entry| {
            (
                entry.file_name().to_str().unwrap().to_string(),
                entry.metadata().unwrap().len(),
                entry.metadata().unwrap().modified(),
            )
        })
        .map(|(file_name, file_size, modified)| {
            let modified_time: Option<SystemTime>;
            if let Ok(system_time) = modified {
                modified_time = Some(system_time);
            } else {
                modified_time = None;
            };
            FileData {
                filename: file_name,
                size: file_size,
                modified: modified_time,
            }
        })
        .collect::<Vec<FileData>>();
    data
}
pub struct ExplorerTable {
    state: TableState,
    current_path: String,
    elements_list: Vec<FileData>,
}

impl ExplorerTable {
    pub fn new() -> Self {
        Self {
            state: TableState::default().with_selected(0),
            current_path: String::from(""),
            elements_list: Vec::new(),
        }
    }

    pub fn go_up(&mut self) {
        let prev_path = Path::new(&self.current_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let new_path = Path::new(&self.current_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        self.update_path(new_path);

        let position_of_prev = self
            .elements_list
            .iter()
            .position(|x| &x.filename == &prev_path);
        self.state.select(position_of_prev);
    }

    pub fn update_path(&mut self, path: String) {
        self.current_path = path;
        let elements = get_file_data(&self.current_path);
        self.elements_list = elements;
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
        let widths = [Constraint::Percentage(40), Constraint::Percentage(10)];
        let header = ["Name", "Name2"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);
        let rows = self
            .elements_list
            .iter()
            .map(|element| Row::new([element.filename.clone(), format_file_size(element.size)]))
            .collect::<Vec<Row>>();
        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(tailwind::BLUE.c400);
        let t = Table::new(rows, widths)
            .style(Style::new().blue())
            .block(Block::new().borders(Borders::ALL))
            .highlight_style(selected_style)
            .header(header);

        let area = self.get_area(frame).unwrap().unwrap();
        frame.render_stateful_widget(t, area, &mut self.state);
        Ok(())
    }
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ChangeDirectory(path) => {
                self.update_path(path);
            }
            Action::ParentDirectory => {
                self.go_up();
            }
            Action::SelectUp => self.previous(),
            Action::SelectDown => self.next(),
            Action::Key(key) => {
                if let Ok(Some(action)) = self.handle_key_events(key) {
                    return Ok(Some(action));
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('j') => return Ok(Some(Action::SelectDown)),
            KeyCode::Char('k') => return Ok(Some(Action::SelectUp)),
            KeyCode::Enter => {
                if let Some(index) = self.state.selected() {
                    let chosen_element = &self.elements_list[index];
                    let created_path = Path::new(&self.current_path).join(&chosen_element.filename);
                    if created_path.is_dir() {
                        let new_path_str = created_path.to_str().unwrap().to_string();
                        return Ok(Some(Action::ChangeDirectory(new_path_str)));
                    }
                }
            }
            KeyCode::Backspace => {
                return Ok(Some(Action::ParentDirectory));
            }

            _ => {}
        };
        Ok(None)
    }

    fn get_area(&mut self, frame: &mut Frame) -> Result<Option<Rect>> {
        let main_box = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(85),
                Constraint::Percentage(5),
                Constraint::Percentage(10),
            ])
            .split(frame.size());
        Ok(Some(main_box[0]))
    }
}
