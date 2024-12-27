use std::fmt::Debug;

use ratatui::{crossterm::event::KeyEvent, layout::Rect};

use crate::{action::Action, command::Command};
pub trait PluginPopUp: PluginPopUpClone {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action>;
    fn should_quit(&self) -> bool;
    fn destruct(&self) -> Option<Box<dyn Command>>;
    fn display_details(&self) -> String;
    fn draw(&mut self, frame: &mut ratatui::Frame, area: Rect);
    fn context(&self) -> String;
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
        self.context() == other.context()
    }
}
