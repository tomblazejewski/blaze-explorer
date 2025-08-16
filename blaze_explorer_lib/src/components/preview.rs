use crate::tools::center_rect;
use ratatui::{Frame, prelude::*, widgets::*};

use super::{Component, component_helpers::Numbering};

///A trait allowing the struct to display its contents in a preview component.
pub trait Previewable {
    ///Returns the data to be displayed
    fn collect_data(&self) -> Vec<String>;
    ///Returns the type of numbering that should be used
    fn get_numbering(&self) -> Numbering;
    ///Returns the line numbers
    fn get_line_numbers(&self) -> Option<Vec<String>>;
}

impl Previewable for PreviewWindow {
    fn collect_data(&self) -> Vec<String> {
        todo!()
    }
    fn get_numbering(&self) -> Numbering {
        todo!()
    }
    fn get_line_numbers(&self) -> Option<Vec<String>> {
        todo!()
    }
}

pub struct PreviewWindow {
    data: Vec<String>,
    numbering: Numbering,
}

impl Component for PreviewWindow {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::eyre::Result<()> {
        let rows = match self.get_line_numbers() {
            None => self
                .collect_data()
                .into_iter()
                .map(|x| Row::new(vec![Line::from(x.clone())]))
                .collect::<Vec<Row>>(),
            Some(line_numbers) => self
                .collect_data()
                .into_iter()
                .zip(line_numbers.clone())
                .map(|(x, y)| Row::new(vec![Line::from(x.clone()), Line::from(y.clone())]))
                .collect::<Vec<Row>>(),
        };
        let widths = match self.get_line_numbers() {
            None => vec![Constraint::Percentage(100)],
            Some(_) => vec![Constraint::Percentage(5), Constraint::Percentage(95)],
        };
        let t = Table::new(rows, widths).block(Block::new().borders(Borders::ALL));
        frame.render_widget(t, area);
        Ok(())
    }
}

mod tests {
    use crate::components::{component_helpers::get_line_numbers, preview::Numbering};
}
