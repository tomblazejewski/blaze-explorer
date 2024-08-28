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

pub struct SearchFileshereSearch {
    absolute_directory: String,
    results: Vec<SearchFilesHereResult>,
}

impl SearchFileshereSearch {
    pub fn new(ctx: AppContext) -> Self {
        Self {
            absolute_directory: ctx.current_directory.display().to_string(),
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

    fn get_results_list(&self) -> Vec<String> {
        self.results
            .iter()
            .map(|r| r.display())
            .collect::<Vec<String>>()
    }

    fn display(&self) -> String {
        "Search in files here".to_string()
    }

    fn preview_result(&self, some_id: Option<usize>, frame: &mut Frame, area: Rect) -> Result<()> {
        let preview_block = Block::default().borders(Borders::ALL).title("Preview");
        match some_id {
            Some(id) => return self.results[id].preview(frame, area, preview_block),
            None => {
                frame.render_widget(Paragraph::default().block(preview_block), area);
            }
        };
        Ok(())
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

    fn preview(&self, frame: &mut Frame, area: Rect, preview_block: Block) -> Result<()> {
        //Render a preview of the contents of the file
        let contents = read_to_string(&self.path).unwrap_or("Could not read the file".to_string());
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
