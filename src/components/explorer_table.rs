use chrono::{offset::Utc, DateTime};
use layout::Alignment;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{
    fs,
    path::{self, Path},
};

use color_eyre::eyre::Result;
use ratatui::{
    prelude::*,
    style::{palette::tailwind, Style},
    widgets::*,
    Frame,
};

use crate::history_stack::directory_history::DirectoryHistory;
use crate::{mode::Mode, themes::CustomTheme};

use super::Component;

#[derive(Debug, Clone, PartialEq)]
pub struct FileData {
    pub id: usize,
    filename: String,
    size: u64,
    modified: Option<DateTime<Utc>>,
}

impl FileData {
    pub fn contains(&self, query: &str) -> bool {
        self.filename.contains(query)
    }
}
pub(crate) const SUFFIXES: [&str; 5] = ["B", "K", "M", "G", "T"];
pub fn format_file_size(size: u64) -> String {
    let mut size = size as f32;
    for suffix in SUFFIXES {
        if size > 1024.0 {
            size /= 1024.0;
        } else {
            return format!("{:.2}{}", size, suffix);
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

fn highlight_search_result(line_text: String, query: &str, highlighted_style: Style) -> Line {
    if line_text.contains(query) {
        let splits = line_text.split(&query);
        let chunks = splits.into_iter().map(|c| Span::from(c.to_owned()));
        let pattern = Span::styled(query, highlighted_style);
        itertools::intersperse(chunks, pattern)
            .collect::<Vec<Span>>()
            .into()
    } else {
        Line::from(line_text)
    }
}

pub fn jump_highlight(
    line_text: String,
    query: &str,
    char: char,
    query_style: Style,
    char_style: Style,
) -> Line {
    if line_text.contains(&query) {
        let mut splits = line_text.split(&query);
        let beginning = Span::from(splits.next().unwrap().to_string());
        let query_span = Span::styled(query, query_style);
        let mut remainder = splits.remainder().unwrap_or("").to_string();
        if !remainder.is_empty() {
            remainder.remove(0);
        }
        let char_span = Span::styled(char.to_string(), char_style);
        let remainder_span = Span::from(remainder);
        Line::from(vec![beginning, query_span, char_span, remainder_span])
    } else {
        Line::from(line_text)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GlobalStyling {
    HighlightSearch(String), //highlight background of the text with the search query
    HighlightJump(String, HashMap<char, usize>), //highlight foreground text with search query
    //+ the char used for the jumping action
    None, //no styling
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExplorerTable {
    state: TableState,
    current_path: PathBuf,
    elements_list: Vec<FileData>,
    mode: Mode,
    selected_ids: Option<Vec<usize>>,
    theme: CustomTheme,
    focused: bool,
    styling: GlobalStyling,
    plugin_display: Option<String>,
    directory_history: DirectoryHistory,
}
impl Default for ExplorerTable {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplorerTable {
    pub fn new() -> Self {
        let starting_path = path::absolute("./").unwrap();
        Self {
            state: TableState::default().with_selected(0),
            current_path: starting_path,
            elements_list: Vec::new(),
            mode: Mode::Normal,
            selected_ids: None,
            theme: CustomTheme::default(),
            focused: true,
            styling: GlobalStyling::None,
            plugin_display: None,
            directory_history: DirectoryHistory::default(),
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

    pub fn update_path(&mut self, path: PathBuf, selected: Option<String>) {
        self.current_path = path;
        let elements = get_file_data(&self.current_path);
        self.elements_list = elements;
        if let Some(to_select) = selected {
            let position_of_prev = self
                .elements_list
                .iter()
                .position(|x| x.filename.as_str() == to_select.as_str());
            self.state.select(position_of_prev);
        } else {
            self.state = TableState::default().with_selected(0);
        }
    }

    fn refresh_contents(&mut self) {
        //get currently selected item
        if self.elements_list.is_empty() {
            self.state.select(None);
            return;
        }
        let mut selected = self.state.selected().unwrap();
        let selected_element_path = self.elements_list[selected].filename.clone();
        // if that element still exists, select it once more
        let elements = get_file_data(&self.current_path);
        self.elements_list = elements;
        if let Some(index) = self
            .elements_list
            .iter()
            .position(|x| x.filename == selected_element_path)
        {
            self.state.select(Some(index));
            return;
        }
        //otherwise, select an item with the same id unless it was the last item
        if selected >= self.elements_list.len() {
            selected = self.elements_list.len() - 1;
        }
        self.state.select(Some(selected));
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

    pub fn select_directory(&self) -> Option<PathBuf> {
        if let Some(index) = self.state.selected() {
            let chosen_element = &self.elements_list[index];
            let created_path = Path::new(&self.current_path).join(&chosen_element.filename);
            Some(created_path)
        } else {
            None
        }
    }

    pub fn jump_to_id(&mut self, id: usize) {
        self.state.select(Some(id));
    }

    pub fn update_search_query(&mut self) {
        self.search_elements();
    }

    pub fn get_search_phrase(&self) -> Option<String> {
        match &self.styling {
            GlobalStyling::HighlightSearch(query) => Some(query.clone()),
            _ => None,
        }
    }

    pub fn next_search_result(&mut self) {
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
        let element_ids = if let GlobalStyling::HighlightSearch(query) = &self.styling {
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

    pub fn find_elements(&self, query: &str) -> Vec<FileData> {
        self.elements_list
            .iter()
            .filter(|x| x.filename.contains(query))
            .map(|x| x.clone())
            .collect()
    }

    pub fn show_in_folder(&mut self, path: PathBuf) {
        // split the path by the last slash separator to get the folder and filename
        let folder = path.parent().unwrap();
        let filename = path.file_name().unwrap();
        self.update_path(
            folder.to_path_buf(),
            Some(filename.to_str().unwrap().to_string()),
        );
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

    pub fn get_current_path(&self) -> PathBuf {
        self.current_path.clone()
    }

    pub fn get_selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn get_selected_string(&self) -> Option<String> {
        if let Some(index) = self.state.selected() {
            // quick-fix to avoid panic when launching the app
            if index >= self.elements_list.len() {
                return None;
            }
            let chosen_element = &self.elements_list[index];
            Some(chosen_element.filename.clone())
        } else {
            None
        }
    }

    pub fn clear_search_query(&mut self) {
        self.set_styling(GlobalStyling::None)
    }

    pub fn get_selected_files(&self) -> Option<Vec<PathBuf>> {
        //currently just take the selection  of the table
        //in the future - take all selected items in the visual mode
        if let Some(path) = self.select_directory() {
            return Some(vec![path]);
        }
        None
    }

    pub fn set_styling(&mut self, styling: GlobalStyling) {
        self.styling = styling;
        self.update_search_query();
    }

    pub fn set_plugin_display(&mut self, plugin_display: Option<String>) {
        self.plugin_display = plugin_display
    }

    pub fn get_directory_history(&mut self) -> &mut DirectoryHistory {
        &mut self.directory_history
    }
}

impl Component for ExplorerTable {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // get table block
        self.refresh_contents();
        let widths = [
            Constraint::Percentage(5),
            Constraint::Fill(1),
            Constraint::Length(6),
            Constraint::Length(20),
        ];
        let header = ["", "Name", "Size", "Last modified"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1)
            .style(self.theme.header);
        let (query, inverted_map) = match &self.styling {
            GlobalStyling::HighlightJump(query, map) => {
                (query.clone(), map.iter().map(|(k, v)| (*v, *k)).collect())
            }
            GlobalStyling::HighlightSearch(query) => (query.clone(), HashMap::new()),
            GlobalStyling::None => (String::new(), HashMap::new()),
        };
        let rows = match self.elements_list.is_empty() {
            false => {
                let line_numbers =
                    get_line_numbers(self.elements_list.len(), self.state.selected().unwrap() + 1);
                self.elements_list
                    .iter()
                    .zip(line_numbers)
                    .map(|(element, row_number)| {
                        Row::new([
                            Cell::from(Text::from(row_number).alignment(Alignment::Right)),
                            {
                                match self.styling {
                                    GlobalStyling::None => Line::from(element.filename.clone()),
                                    GlobalStyling::HighlightSearch(_) => highlight_search_result(
                                        element.filename.clone(),
                                        &query,
                                        self.theme.search_result,
                                    ),
                                    GlobalStyling::HighlightJump(_, _) => jump_highlight(
                                        element.filename.clone(),
                                        &query,
                                        inverted_map.get(&element.id).unwrap_or(&' ').to_owned(),
                                        self.theme.highlight_query.clone(),
                                        self.theme.highlight_jump_char.clone(),
                                    ),
                                }
                            }
                            .into(),
                            format_file_size(element.size).into(),
                            format_last_time(&element.modified).into(),
                        ])
                        .style(Style::new().bg(match &self.selected_ids {
                            Some(selected_ids) => match selected_ids.contains(&element.id) {
                                true => tailwind::BLACK,
                                false => tailwind::BLACK,
                            },
                            None => tailwind::BLACK,
                        }))
                        .fg(tailwind::WHITE)
                    })
                    .collect::<Vec<Row>>()
            }
            true => Vec::new(),
        };
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
        let plugin_span = match &self.plugin_display {
            Some(plugin) => Span::styled(
                plugin,
                Style::default()
                    .bg(tailwind::GREEN.c200)
                    .fg(tailwind::BLACK),
            ),
            None => Span::from(String::from("")),
        };

        let status_line = match self.focused {
            true => Line::from(vec![mode_span, plugin_span, path_span]),
            false => Line::from(vec![path_span]),
        };

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
