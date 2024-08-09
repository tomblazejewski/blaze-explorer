use chrono::{offset::Utc, DateTime};
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

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
    modified: Option<DateTime<Utc>>,
}

pub(crate) const SUFFIXES: [&str; 5] = ["B", "K", "M", "G", "T"];
pub fn format_file_size(size: u64) -> String {
    let mut size = size as f32;
    for suffix in SUFFIXES {
        if size > 1024.0 {
            size /= 1024.0;
        } else {
            return format!("{:.2}{}", size, suffix.to_string());
        }
    }
    String::from("0")
}
pub fn format_last_time(last_time: &Option<DateTime<Utc>>) -> String {
    if let Some(datetime) = last_time {
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        String::from("")
    }
}
pub fn get_file_data(path: &PathBuf) -> Vec<FileData> {
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
            let modified_time: Option<DateTime<Utc>>;
            if let Ok(system_time) = modified {
                modified_time = Some(system_time.into());
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
    current_path: PathBuf,
    elements_list: Vec<FileData>,
}

impl ExplorerTable {
    pub fn new() -> Self {
        Self {
            state: TableState::default().with_selected(0),
            current_path: PathBuf::from("./"),
            elements_list: Vec::new(),
        }
    }

    pub fn go_up(&mut self) {
        let prev_folder = &self.current_path.file_name();
        if let &Some(prev_folder_name) = prev_folder {
            let prev_folder_string = prev_folder_name.to_str().unwrap();
            let new_absolute_path = self.current_path.parent().unwrap().to_owned();
            let position_of_prev = self
                .elements_list
                .iter()
                .position(|x| x.filename.as_str() == prev_folder_string);
            self.state.select(position_of_prev);
            self.update_path(new_absolute_path);
        }
    }

    pub fn update_path(&mut self, path: PathBuf) {
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

    pub fn select_directory(&mut self) -> Option<PathBuf> {
        if let Some(index) = self.state.selected() {
            let chosen_element = &self.elements_list[index];
            let created_path = Path::new(&self.current_path).join(&chosen_element.filename);
            if created_path.is_dir() {
                Some(created_path)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Component for ExplorerTable {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let widths = [
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ];
        let header = ["Name", "Size", "Last modified"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);
        let rows = self
            .elements_list
            .iter()
            .map(|element| {
                Row::new([
                    element.filename.clone(),
                    format_file_size(element.size),
                    format_last_time(&element.modified),
                ])
            })
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
            Action::SelectDirectory => {
                let target_directory = self.select_directory();
                if let Some(found_directory) = target_directory {
                    return Ok(Some(Action::ChangeDirectory(found_directory)));
                } else {
                    return Ok(None);
                }
            }
            Action::ChangeDirectory(path) => {
                self.update_path(path);
            }
            Action::ParentDirectory => {
                self.go_up();
            }
            Action::SelectUp => self.previous(),
            Action::SelectDown => self.next(),
            _ => {}
        }
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
