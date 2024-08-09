use std::path::{self, Path, PathBuf};

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
    current_path: PathBuf,
}

impl PathDisplay {
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::from("./"),
        }
    }

    pub fn update_path(&mut self, absolute_path: PathBuf) {
        self.current_path = absolute_path;
    }
}

impl Component for PathDisplay {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let path_paragraph =
            Paragraph::new(self.current_path.clone().to_str().unwrap().to_string())
                .block(Block::new().borders(Borders::ALL));
        let area = self.get_area(frame).unwrap().unwrap();
        frame.render_widget(path_paragraph, area);
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ChangeDirectory(path) => {
                self.update_path(path);
            }
            Action::Key(key) => {
                self.handle_key_events(key);
            }
            Action::ParentDirectory => {
                let parent_path = self.current_path.parent();
                if let Some(parent_path_found) = parent_path {
                    self.update_path(parent_path_found.to_owned());
                }
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
