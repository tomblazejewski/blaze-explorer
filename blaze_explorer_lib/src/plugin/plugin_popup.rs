use std::{collections::HashMap, fmt::Debug};

use color_eyre::eyre::Result;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{
    action::Action, app::App, app_input_machine::get_none_action, command::Command, mode::Mode,
};
pub trait PluginPopUp: PluginPopUpClone {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;

    fn confirm_result(&mut self) -> Option<Action> {
        None
    }

    fn next_result(&mut self) -> Option<Action> {
        None
    }

    fn previous_result(&mut self) -> Option<Action> {
        None
    }

    fn update_search_query(&mut self, _query: String) -> Option<Action> {
        None
    }

    fn push_search_char(&mut self, ch: char) -> Option<Action>;

    fn drop_search_char(&mut self) -> Option<Action>;

    fn quit(&mut self);

    fn should_quit(&self) -> bool;

    fn erase_text(&mut self) -> Option<Action>;

    fn get_search_query(&self) -> String;

    fn destruct(&self) -> Option<Box<dyn Command>> {
        None
    }

    fn context(&self) -> String {
        String::new()
    }

    fn display_details(&self) -> String;

    fn get_default_action(&self) -> Box<fn(KeyEvent) -> Option<Action>> {
        Box::new(get_none_action)
    }
    fn get_own_keymap(&self) -> HashMap<(Mode, Vec<KeyEvent>), Action>;

    fn update_app(&mut self, _app: &mut App) {}
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
