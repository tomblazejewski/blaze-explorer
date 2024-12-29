use std::{
    fmt::Display,
    fs::read_to_string,
    path::Path,
    time::{Duration, Instant},
};

use color_eyre::eyre::Result;
use ratatui::{
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use rust_search::SearchBuilder;

use crate::{
    action::{Action, AppAction},
    telescope::{AppContext, TelescopeResult, TelescopeSearch},
};

#[derive(Debug, Clone, PartialEq)]
pub struct SearchFileshereSearch {
    absolute_directory: String,
    results: Vec<SearchFilesHereResult>,
    last_search_timing: Option<Duration>,
}

impl SearchFileshereSearch {
    pub fn new(ctx: AppContext) -> Self {
        Self {
            absolute_directory: ctx.current_directory.display().to_string(),
            results: Vec::new(),
            last_search_timing: None,
        }
    }
}
impl TelescopeSearch for SearchFileshereSearch {
    fn search(&mut self, query: String) {
        let start = Instant::now();
        self.results = SearchBuilder::default()
            .location(self.absolute_directory.clone())
            .search_input(query)
            .limit(1000) // results to return
            // .strict()
            .ignore_case()
            .hidden()
            .build()
            .map(SearchFilesHereResult::new)
            .collect::<Vec<SearchFilesHereResult>>();
        self.last_search_timing = Some(start.elapsed());
    }

    fn confirm_result(&mut self, id: usize) -> Option<Action> {
        let result = &self.results[id];
        let path = Path::new(&result.path).to_path_buf();
        Some(Action::AppAct(AppAction::ShowInFolder(path)))
    }

    fn get_results_list(&self) -> Vec<String> {
        self.results
            .iter()
            .map(|r| r.display())
            .collect::<Vec<String>>()
    }

    fn display(&self) -> String {
        let elapsed = match &self.last_search_timing {
            Some(d) => (d.as_millis() as f64 / 1000.0).to_string(),
            None => "".to_string(),
        };
        format!("Search here - {}", elapsed)
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

    fn n_results(&self) -> usize {
        self.results.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
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
