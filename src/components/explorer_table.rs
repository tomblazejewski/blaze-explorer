use color_eyre::eyre::Result;
use ratatui::{
    prelude::*,
    style::{palette::tailwind, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use super::Component;

pub struct ExplorerTable {
    state: TableState,
    elements_list: Vec<String>,
}

impl ExplorerTable {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            elements_list: Vec::new(),
        }
    }
}

impl Component for ExplorerTable {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let str_paths = self.elements_list.clone();
        let widths = [Constraint::Percentage(60)];
        let header = ["Name"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);
        let rows = str_paths
            .into_iter()
            .map(|path_str| Row::new([path_str]))
            .collect::<Vec<Row>>();
        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(tailwind::BLUE.c400);
        let t = Table::new(rows, widths)
            .style(Style::new().blue())
            .block(Block::new().borders(Borders::ALL))
            .highlight_style(selected_style)
            .header(header);

        frame.render_stateful_widget(t, area, &mut self.state);
        Ok(())
    }
}
