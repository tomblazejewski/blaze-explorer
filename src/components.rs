pub mod explorer_table;

use crate::action::Action;
use color_eyre::Result;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

pub trait Component {
    fn init(&mut self, area: Rect) -> Result<()> {
        let _ = area;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let _ = key;
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        let _ = action;
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
}
