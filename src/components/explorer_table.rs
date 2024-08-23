use chrono::{offset::Utc, DateTime};
use layout::Alignment;
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};
use style::Styled;
use tracing::info;

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::*,
    style::{palette::tailwind, Modifier, Style},
    widgets::*,
    Frame,
};

use crate::{
    action::{Action, ExplorerAction},
    action_agent::ActionAgent,
    mode::Mode,
    themes::CustomTheme,
};

use super::Component;

#[derive(Debug)]
pub struct FileData {
    id: usize,
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
        .enumerate()
        .map(|(id, (file_name, file_size, modified))| {
            let modified_time: Option<DateTime<Utc>>;
            if let Ok(system_time) = modified {
                modified_time = Some(system_time.into());
            } else {
                modified_time = None;
            };
            FileData {
                id,
                filename: file_name,
                size: file_size,
                modified: modified_time,
            }
        })
        .collect::<Vec<FileData>>();
    data
}

fn get_line_numbers(n_lines: usize, current_line: usize) -> Vec<String> {
    info!("{}, {}", n_lines, current_line);
    //create all string labels before the selected line
    let before_selected = (1..current_line)
        .rev()
        .map(|number| number.to_string())
        .collect::<Vec<String>>();
    let mut current_lines = before_selected;
    let n_lines_after = n_lines - current_line;
    let after_selected_iter = (1..n_lines_after + 1).map(|number| number.to_string());
    let current_line_string = format!("{} ", current_line);
    current_lines.append(&mut vec![current_line_string]);
    current_lines.extend(after_selected_iter);
    current_lines
}

fn highlight_search_result(
    line_text: String,
    query: Option<&str>,
    highlighted_style: Style,
) -> Line {
    if query.is_none() {
        return Line::from(line_text);
    }
    let query = query.unwrap();
    if line_text.contains(&query) {
        let splits = line_text.split(&query);
        let chunks = splits.into_iter().map(|c| Span::from(c.to_owned()));
        let pattern = Span::styled(query.clone(), highlighted_style);
        itertools::intersperse(chunks, pattern)
            .collect::<Vec<Span>>()
            .into()
    } else {
        Line::from(line_text)
    }
}
pub struct ExplorerTable {
    state: TableState,
    current_path: PathBuf,
    elements_list: Vec<FileData>,
    mode: Mode,
    search_phrase: Option<String>,
    selected_ids: Option<Vec<usize>>,
    theme: CustomTheme,
    focused: bool,
}
impl ExplorerTable {
    pub fn new() -> Self {
        Self {
            state: TableState::default().with_selected(0),
            current_path: PathBuf::from("./"),
            elements_list: Vec::new(),
            mode: Mode::Normal,
            search_phrase: None,
            selected_ids: None,
            theme: CustomTheme::default(),
            focused: true,
        }
    }

    pub fn get_line_labels(&mut self) -> Vec<String> {
        let selected_row = self.state.selected().unwrap() + 1;
        let mut before_selected = (selected_row - 1..0)
            .map(|number| number.to_string())
            .collect::<Vec<String>>();
        let list_length = self.elements_list.len();
        let last_element = list_length - selected_row + 1;
        let after_selected_iter = (1..last_element).map(|number| number.to_string());
        let selected_row = format!("{} ", selected_row);
        before_selected.append(&mut vec![selected_row]);
        before_selected.extend(after_selected_iter);
        before_selected
    }

    pub fn go_up(&mut self) {
        let prev_folder = self.current_path.file_name().map(|name| name.to_owned());
        if let Some(prev_folder_name) = prev_folder {
            let prev_folder_string = prev_folder_name.to_str().unwrap();
            let new_absolute_path = self.current_path.parent().unwrap().to_owned();
            self.update_path(new_absolute_path);
            let position_of_prev = self
                .elements_list
                .iter()
                .position(|x| x.filename.as_str() == prev_folder_string);
            self.state.select(position_of_prev);
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

    fn update_search_query(&mut self, new_query: String) {
        if !new_query.is_empty() {
            self.search_phrase = Some(new_query)
        } else {
            self.search_phrase = None
        }
        self.search_elements();
    }

    fn next_search_result(&mut self) {
        if let Some(selected_ids) = &self.selected_ids {
            if selected_ids.len() < 2 {
                return;
            }
            let i = match self.state.selected() {
                Some(i) => {
                    // select the next search result whose id is bigger than id of selected element
                    // if there is no such element then start from id 0 and check again
                    let mut j = i;
                    loop {
                        j += 1;
                        if j == self.elements_list.len() {
                            j = 0;
                        }
                        if selected_ids.contains(&j) {
                            break;
                        }
                    }
                    j
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }
    pub fn search_elements(&mut self) {
        let element_ids = if let Some(query) = &self.search_phrase {
            Some(
                self.elements_list
                    .iter()
                    .filter(|x| x.filename.contains(query))
                    .map(|x| x.id)
                    .collect::<Vec<usize>>(),
            )
        } else {
            None
        };
        self.selected_ids = element_ids;
    }
    pub fn explorer_action(&mut self, explorer_act: ExplorerAction) -> Option<Action> {
        match explorer_act {
            ExplorerAction::SelectDirectory => {
                let target_directory = self.select_directory();
                if let Some(found_directory) = target_directory {
                    return Some(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                        found_directory,
                    )));
                } else {
                    return None;
                }
            }
            ExplorerAction::ChangeDirectory(path) => {
                self.update_path(path);
            }
            ExplorerAction::ParentDirectory => {
                self.go_up();
            }
            ExplorerAction::SelectUp => self.previous(),
            ExplorerAction::SelectDown => self.next(),
            ExplorerAction::UpdateSearchQuery(query) => self.update_search_query(query),
            ExplorerAction::ClearSearchQuery => self.search_phrase = None,
            ExplorerAction::NextSearchResult => self.next_search_result(),
        }
        None
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode
    }
    pub fn focus(&mut self) {
        self.focused = true
    }

    pub fn unfocus(&mut self) {
        self.focused = false
    }
}

impl Component for ExplorerTable {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // get table block
        let widths = [
            Constraint::Percentage(5),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
        ];
        let header = ["", "Name", "Size", "Last modified"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1)
            .style(self.theme.header);
        let line_numbers =
            get_line_numbers(self.elements_list.len(), self.state.selected().unwrap() + 1);
        let rows = self
            .elements_list
            .iter()
            .zip(line_numbers)
            .map(|(element, row_number)| {
                Row::new([
                    Cell::from(Text::from(row_number).alignment(Alignment::Right)),
                    highlight_search_result(
                        element.filename.clone(),
                        self.search_phrase.as_deref(),
                        self.theme.search_result,
                    )
                    .into(),
                    format_file_size(element.size).into(),
                    format_last_time(&element.modified).into(),
                ])
                .style(Style::new().bg(match &self.selected_ids {
                    Some(selected_ids) => {
                        if selected_ids.contains(&element.id) {
                            tailwind::BLACK
                        } else {
                            tailwind::BLACK
                        }
                    }
                    None => tailwind::BLACK,
                }))
                .fg(tailwind::WHITE)
            })
            .collect::<Vec<Row>>();
        let style = match self.focused {
            true => self.theme.focused_border,
            false => self.theme.unfocused_border,
        };
        let t = Table::new(rows, widths)
            // .style(self.theme.selected_frame)
            .style(style)
            .block(Block::new().borders(Borders::ALL))
            .highlight_style(self.theme.selected_row)
            .header(header);

        // get paragraph block
        let mode_span = Span::styled(
            self.mode.to_string(),
            Style::default()
                .bg(if self.mode == Mode::Normal {
                    tailwind::BLUE.c200
                } else {
                    tailwind::VIOLET.c200
                })
                .fg(tailwind::BLACK),
        );
        let path_span = Span::from(self.current_path.to_str().unwrap());

        let status_line = Line::from(vec![mode_span, path_span]);

        //divide the available area into one for the table and one for the paragraph
        let explorer_area_blocks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(area);
        frame.render_stateful_widget(t, explorer_area_blocks[0], &mut self.state);
        frame.render_widget(status_line, explorer_area_blocks[1]);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_numbers() {
        let current_line = 3_usize;
        let line_length = 6_usize;

        let result = get_line_numbers(line_length, current_line);

        let expected_result = vec![
            String::from("2"),
            String::from("1"),
            String::from("3 "),
            String::from("1"),
            String::from("2"),
            String::from("3"),
        ];
        assert_eq!(result, expected_result);
    }
}
