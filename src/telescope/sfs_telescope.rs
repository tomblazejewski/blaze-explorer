use std::{fmt::Display, fs::read_to_string, path::Path};

use color_eyre::eyre::Result;
use ratatui::{
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use rust_search::SearchBuilder;

use crate::action::{Action, ExplorerAction};

use super::{AppContext, TelescopeResult, TelescopeSearch};

struct SearchFileshereSearch {
    absolute_directory: String,
    results: Vec<SearchFilesHereResult>,
}

impl SearchFileshereSearch {
    pub fn new(absolute_directory: String) -> Self {
        Self {
            absolute_directory,
            results: Vec::new(),
        }
    }
}
impl TelescopeSearch for SearchFileshereSearch {
    fn search(&mut self, query: String) {
        self.results = SearchBuilder::default()
            .location(self.absolute_directory.clone())
            .search_input(query)
            .limit(10) // results to return
            .strict()
            .ignore_case()
            .hidden()
            .build()
            .map(SearchFilesHereResult::new)
            .collect::<Vec<SearchFilesHereResult>>();
    }

    fn confirm_result(&mut self, id: usize) -> Option<Action> {
        let result = &self.results[id];
        Some(Action::ExplorerAct(ExplorerAction::ShowInFolder(
            Path::new(&result.path).to_path_buf(),
        )))
    }

    fn new(ctx: AppContext) -> Self {
        Self {
            absolute_directory: ctx.current_directory.display().to_string(),
            results: Vec::new(),
        }
    }

    fn get_results_list(&self) -> Vec<String> {
        self.results
            .iter()
            .map(|r| r.display())
            .collect::<Vec<String>>()
    }

    fn preview_result(&self, id: usize, frame: &mut Frame, area: Rect) -> Result<()> {
        self.results[id].preview(frame, area)
    }
}

impl Display for SearchFileshereSearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Search in files here")
    }
}
struct SearchFilesHereResult {
    path: String,
}

impl SearchFilesHereResult {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl TelescopeResult for SearchFilesHereResult {
    fn display(&self) -> String {
        self.path.clone()
    }

    fn preview(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        //Render a preview of the contents of the file
        let preview_block = Block::default().borders(Borders::ALL).title("Preview");
        let contents = read_to_string(&self.path).unwrap();
        let lines = contents.lines().map(Line::from).collect::<Vec<Line>>();
        let paragraph = Paragraph::new(Text::from(lines)).block(preview_block);

        frame.render_widget(paragraph, area);
        Ok(())
    }

    fn from<S>(s: S) -> Self
    where
        S: ToString + Display,
    {
        Self {
            path: s.to_string(),
        }
    }
}
