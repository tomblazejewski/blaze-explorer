use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    widgets::{Block, Borders, Clear, Table},
};

use crate::tools::center_rect;

use super::Component;

///A trait allowing the struct to display its contents in a preview component.
pub trait Previewable {
    ///Returns the data to be displayed
    fn collect_data(&self) -> Vec<String>;
    ///Returns the type of numbering that should be used
    fn get_numbering(&self) -> Numbering;
    ///Returns the line numbers
    fn get_line_numbers(&self) -> Vec<String>;
}

pub enum Numbering {
    None,   //don't render the numbers at all
    Simple, // label from 0 to n
    VimLike, //selected row is 0, lines are numbered according to how far they are from the
            //selected row
}
pub struct PreviewWindow {
    data: Vec<String>,
    numbering: Numbering,
}

impl Component for PreviewWindow {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::eyre::Result<()> {
        Ok(())
    }
}
