use std::fmt::Debug;

use color_eyre::eyre::Result;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{action::Action, command::Command};
pub trait PluginPopUp: PluginPopUpClone {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action>;

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;

    fn confirm_result(&mut self) -> Option<Action> {
        None
    }

    fn next_result(&mut self) {}

    fn previous_result(&mut self) {}

    fn update_search_query(&mut self, _query: String) {}

    fn push_search_char(&mut self, ch: char);

    fn drop_search_char(&mut self);

    fn quit(&mut self);

    fn should_quit(&self) -> bool;

    fn erase_text(&mut self);

    fn get_search_query(&self) -> String;

    fn destruct(&self) -> Option<Box<dyn Command>> {
        None
    }

    fn context(&self) -> String {
        String::new()
    }

    fn display_details(&self) -> String;
}

pub trait PluginPopUpClone: Debug {
    fn clone_box(&self) -> Box<dyn PluginPopUp>;
}

impl<T> PluginPopUpClone for T
where
    T: 'static + PluginPopUp + Clone + Debug + PartialEq,
{
    fn clone_box(&self) -> Box<dyn PluginPopUp> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn PluginPopUp> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn PluginPopUp> {
    //FIXME: how to implement this better?
    fn eq(&self, other: &Self) -> bool {
        *self.context() == *other.context()
    }
}
