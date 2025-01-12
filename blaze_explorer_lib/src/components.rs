pub mod command_line;
pub mod explorer_manager;
pub mod explorer_table;

use color_eyre::Result;
use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
}
