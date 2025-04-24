use std::{collections::HashMap, fmt::Debug};

use color_eyre::eyre::Result;
use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect};

use crate::{action::Action, line_entry::LineEntry, mode::Mode, query::Query};

use super::plugin_popup::PluginPopUp;

#[derive(Debug, Clone, PartialEq)]
pub struct BasePopUp {
    pub should_quit: bool,
    pub query: Query,
    pub keymap: HashMap<(Mode, Vec<KeyEvent>), Action>,
}

pub trait PopupBehavior {
    fn popup_title(&self) -> String;
    fn confirm_action(&self, query: String) -> Action;
    fn display_details(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericPopUp<T: PopupBehavior> {
    pub base: BasePopUp,
    pub behavior: T,
}

impl<T: PopupBehavior + Clone + Debug + PartialEq + 'static> PluginPopUp for GenericPopUp<T> {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let query_area = crate::tools::center_rect(
            frame.size(),
            ratatui::layout::Constraint::Percentage(50),
            ratatui::layout::Constraint::Length(3),
        );
        let title = self.behavior.popup_title();
        let query_block = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title(title);
        let query_paragraph =
            ratatui::widgets::Paragraph::new(self.base.query.get_contents()).block(query_block);

        frame.render_widget(ratatui::widgets::Clear, query_area);
        frame.render_widget(query_paragraph, query_area);
        Ok(())
    }

    fn push_search_char(&mut self, ch: char) -> Option<Action> {
        self.base.query.append_char(ch);
        None
    }

    fn drop_search_char(&mut self) -> Option<Action> {
        self.base.query.drop_char();
        None
    }

    fn quit(&mut self) {
        self.base.should_quit = true;
    }

    fn should_quit(&self) -> bool {
        self.base.should_quit
    }

    fn erase_text(&mut self) -> Option<Action> {
        self.base.query.clear_contents();
        None
    }

    fn get_search_query(&self) -> String {
        self.base.query.get_contents()
    }

    fn display_details(&self) -> String {
        self.behavior.display_details()
    }

    fn get_own_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action> {
        self.base.keymap.clone()
    }

    fn get_default_action(&self) -> Box<fn(KeyEvent) -> Option<Action>> {
        Box::new(crate::plugin::plugin_helpers::get_push_on_char_action)
    }

    fn confirm_result(&mut self) -> Option<Action> {
        self.quit();
        Some(self.behavior.confirm_action(self.get_search_query()))
    }
}
