use std::path::{self, Path};

use color_eyre::eyre::Result;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use ratatui::{
    layout::{Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::action::Action;

use super::Component;

pub struct PathDisplay {
    current_path: String,
}

impl PathDisplay {
    pub fn new() -> Self {
        Self {
            current_path: String::new(),
        }
    }

    pub fn update_absolute_path(&mut self, absolute_path: String) {
        self.current_path = absolute_path;
    }
}

impl Component for PathDisplay {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let path_paragraph =
            Paragraph::new(self.current_path.clone()).block(Block::new().borders(Borders::ALL));
        let area = self.get_area(frame).unwrap().unwrap();
        frame.render_widget(path_paragraph, area);
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ChangeDirectory(path) => {
                let absolute_path = path::absolute(path).unwrap().to_str().unwrap().to_string();
                self.update_absolute_path(absolute_path);
            }
            Action::Key(key) => {
                self.handle_key_events(key);
            }
            Action::ParentDirectory => {
                let new_path = Path::new(&self.current_path)
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                self.update_absolute_path(new_path);
            }
            _ => {}
        }

        Ok(None)
    }

    fn get_area(&mut self, frame: &mut Frame) -> Result<Option<Rect>> {
        let main_box = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(85),
                Constraint::Percentage(5),
                Constraint::Percentage(10),
            ])
            .split(frame.size());
        Ok(Some(main_box[2]))
    }
}
