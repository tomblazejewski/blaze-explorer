use ratatui::crossterm::event::KeyEvent;
use std::any::Any;
use std::fmt::Debug;

use super::Action;

#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    SimpleAction(Action),
    CompositeAction(Box<dyn ActionBuilder>),
}

impl ActionType {
    pub fn resolve_action(&self, key_sequence: Vec<KeyEvent>) -> Action {
        match self {
            ActionType::SimpleAction(action) => action.clone(),
            ActionType::CompositeAction(builder) => builder.resolve_action(key_sequence),
        }
    }
}

pub trait ActionBuilder: ActionBuilderClone + Any + ActionBuilderEq {
    fn resolve_action(&self, key_sequence: Vec<KeyEvent>) -> Action;
}

pub trait ActionBuilderClone: Debug {
    fn clone_box(&self) -> Box<dyn ActionBuilder>;
}

impl<T> ActionBuilderClone for T
where
    T: 'static + ActionBuilder + Clone + Debug + PartialEq,
{
    fn clone_box(&self) -> Box<dyn ActionBuilder> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ActionBuilder> {
    fn clone(&self) -> Box<dyn ActionBuilder> {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn ActionBuilder> {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_ref())
    }
}

impl dyn ActionBuilder {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait ActionBuilderEq {
    fn dyn_eq(&self, other: &dyn ActionBuilder) -> bool;
}

impl<T> ActionBuilderEq for T
where
    T: 'static + ActionBuilder + PartialEq,
{
    fn dyn_eq(&self, other: &dyn ActionBuilder) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
}
