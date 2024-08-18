pub mod command_line;
pub mod explorer_table;
pub mod key_tracker;
pub mod mode_display;
pub mod path_display;

use crate::action::Action;
use color_eyre::Result;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

pub trait Component {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
}
