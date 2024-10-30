pub mod command_line;
pub mod explorer_manager;
pub mod explorer_table;
pub mod key_tracker;
pub mod mode_display;

use color_eyre::Result;
use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
}
