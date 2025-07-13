pub mod command_line;
pub mod explorer_manager;
pub mod explorer_table;
pub mod preview;

use color_eyre::Result;
use ratatui::{Frame, layout::Rect};

pub trait Component {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
}
