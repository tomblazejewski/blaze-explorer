pub mod sfs_telescope;
use std::{
    fmt::Display,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};
use rust_search::SearchBuilder;

use crate::{
    action::{Action, ExplorerAction},
    components::Component,
    telescope_query::TelescopeQuery,
};

pub struct AppContext {
    current_directory: PathBuf,
}

pub trait TelescopeSearch {
    /// Perform necessary actions to return the search results
    fn search(&mut self, query: String);

    fn get_results_list(&self) -> Vec<String>;

    /// Determine what happens when the user confirms a result
    fn confirm_result(&mut self, id: usize) -> Option<Action>;

    //Create a telescope search instance from a collection of initial params
    fn new(ctx: AppContext) -> Self;

    fn preview_result(&self, id: usize, frame: &mut Frame, area: Rect) -> Result<()>;
}

pub trait TelescopeResult {
    // What is displayed in the result list on the left
    fn display(&self) -> String;
    // What is rendered in the preview area when the user selects a result
    fn preview(&self, frame: &mut Frame, area: Rect) -> Result<()>;

    fn from<S: ToString + Display>(s: S) -> Self;
}
pub struct Telescope<T>
where
    T: TelescopeSearch,
{
    pub query: TelescopeQuery,
    pub search: T,
    pub table_state: TableState,
}

impl<T> Telescope<T> where T: TelescopeSearch + Display {}

pub trait PopUpComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
    fn handle_action(&mut self, action: Action) -> Option<Action>;
    fn new(search_context: AppContext) -> Self;
}
impl<T> PopUpComponent for Telescope<T>
where
    T: TelescopeSearch + Display,
{
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        //split the area vertically 60/40
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);
        //split the left chunk into results and query, leaving one line for query
        let list_query_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(chunks[0]);
        let result_area = list_query_split[0];
        let query_area = list_query_split[1];
        let preview_area = chunks[1];
        let results_block = Block::default().borders(Borders::ALL).title("Results");
        let query_block = Block::default()
            .borders(Borders::ALL)
            .title(format!("{} Search", self.search));

        // this type is responsible for rendering the query block - this is just a paragraph with
        // the query
        let query_paragraph = Paragraph::new(self.query.contents.clone());
        let query_paragraph = query_paragraph.block(query_block);

        frame.render_widget(query_paragraph, query_area);

        //create a table from the vector of results
        self.search.search(self.query.contents.clone());

        let results = self.search.get_results_list();
        let rows = results
            .into_iter()
            .map(|r| Row::new([Cell::from(r)]))
            .collect::<Vec<Row>>();

        let widths = [Constraint::Percentage(100)];
        let table = Table::new(rows, widths).block(results_block);
        frame.render_stateful_widget(table, result_area, &mut self.table_state);

        //render the preview - this is handled by the result type (or at least for now)
        if let Some(id) = self.table_state.selected() {
            self.search.preview_result(id, frame, area)?;
        }

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Option<Action> {
        match action {
            _ => {}
        }
        None
    }

    fn new(search_context: AppContext) -> Self {
        Self {
            query: TelescopeQuery::new(),
            search: T::new(search_context),
            table_state: TableState::default().with_selected(0),
        }
    }
}
