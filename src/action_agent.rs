use color_eyre::eyre::Result;

use crate::action::Action;

pub trait ActionAgent {
    fn update(&mut self, action: Action) -> Result<Option<Action>>;
}
