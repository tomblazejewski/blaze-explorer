use std::fs;

use ratatui::widgets::TableState;

pub struct App {
    pub current_path: String,
    pub elements_list: Vec<String>,
    pub selected_elements_list: Vec<String>,
    pub table_state: TableState,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_path: String::from("./"),
            elements_list: Vec::new(),
            selected_elements_list: Vec::new(),
            table_state: TableState::default().with_selected(0),
        }
    }

    pub fn update_path(&mut self, path: String) {
        self.current_path = path.clone();
        let paths = fs::read_dir(path).unwrap();

        let str_paths = paths
            .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        self.elements_list = str_paths;
        self.selected_elements_list = Vec::new();
    }

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.elements_list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}
