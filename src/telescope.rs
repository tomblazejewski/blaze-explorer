pub mod sfs_telescope;
use std::{fmt::Display, path::PathBuf};

use color_eyre::eyre::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};
use sfs_telescope::SearchFileshereSearch;

use crate::{
    action::{Action, AppAction, ExplorerAction, TelescopeAction},
    telescope_query::TelescopeQuery,
    themes::CustomTheme,
};

pub struct AppContext {
    current_directory: PathBuf,
}

impl AppContext {
    pub fn new(current_directory: PathBuf) -> Self {
        Self { current_directory }
    }
}

pub trait TelescopeSearch {
    /// Perform necessary actions to return the search results
    fn search(&mut self, query: String);

    fn get_results_list(&self) -> Vec<String>;

    /// Determine what happens when the user confirms a result
    fn confirm_result(&mut self, id: usize) -> Option<Action>;

    fn preview_result(&self, id: Option<usize>, frame: &mut Frame, area: Rect) -> Result<()>;

    fn display(&self) -> String;

    fn n_results(&self) -> usize;
}

pub trait TelescopeResult {
    // What is displayed in the result list on the left
    fn display(&self) -> String;
    // What is rendered in the preview area when the user selects a result
    fn preview(&self, frame: &mut Frame, area: Rect, preview_block: Block) -> Result<()>;

    fn from<S: ToString + Display>(s: S) -> Self;
}
pub struct Telescope {
    pub query: TelescopeQuery,
    pub search: Box<dyn TelescopeSearch>,
    pub table_state: TableState,
    theme: CustomTheme,
}

impl Telescope {
    fn confirm_result(&mut self) -> Option<Action> {
        if let Some(id) = self.table_state.selected() {
            return self.search.confirm_result(id);
        }
        None
    }

    fn next_result(&mut self) {
        let n_results = self.search.n_results();
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == n_results - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
    fn previous_result(&mut self) {
        let n_results = self.search.n_results();
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    n_results - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}
pub trait PopUpComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
    fn handle_action(&mut self, action: Action) -> Option<Action>;
    fn new(search_context: AppContext) -> Self;
}
impl PopUpComponent for Telescope {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        //split the area vertically 60/40
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);
        //split the left chunk into results and query, leaving one line for query
        let list_query_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(3)])
            .split(chunks[0]);
        let result_area = list_query_split[0];
        let query_area = list_query_split[1];
        let preview_area = chunks[1];
        let results_block = Block::default().borders(Borders::ALL).title("Results");
        let query_block = Block::default()
            .borders(Borders::ALL)
            .title(self.search.display());

        // this type is responsible for rendering the query block - this is just a paragraph with
        // the query
        let query_paragraph = Paragraph::new(self.query.contents.clone());
        let query_paragraph = query_paragraph.block(query_block);

        frame.render_widget(query_paragraph, query_area);

        //create a table from the vector of results

        let rows = (*self.search)
            .get_results_list()
            .clone()
            .into_iter()
            .map(|r| Row::new([Cell::from(r)]))
            .collect::<Vec<Row>>();

        match (self.table_state.selected(), rows.is_empty()) {
            (None, false) => self.table_state.select(Some(0)),
            (Some(_), true) => self.table_state.select(None),
            _ => {}
        }
        let widths = [Constraint::Percentage(100)];
        let table = Table::new(rows, widths)
            .block(results_block)
            .highlight_style(self.theme.selected_row_telescope);
        frame.render_stateful_widget(table, result_area, &mut self.table_state);

        //render the preview - this is handled by the result type (or at least for now)
        self.search
            .preview_result(self.table_state.selected(), frame, preview_area)?;

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Option<Action> {
        if let Action::TelescopeAct(action) = action {
            match action {
                TelescopeAction::ConfirmResult => return self.confirm_result(),
                TelescopeAction::NextResult => {
                    self.next_result();
                }
                TelescopeAction::PreviousResult => self.previous_result(),
                TelescopeAction::UpdateSearchQuery(query) => self.search.search(query),
                action => return self.query.handle_text_action(action),
            }
        }
        None
    }

    fn new(search_context: AppContext) -> Self {
        Self {
            query: TelescopeQuery::new(),
            search: Box::new(SearchFileshereSearch::new(search_context)),
            table_state: TableState::default(),
            theme: CustomTheme::default(),
        }
    }
}
