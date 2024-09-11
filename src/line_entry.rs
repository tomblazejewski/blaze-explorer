use crate::action::Action;

pub trait LineEntry {
    fn pop_contents(&mut self) -> String;
    fn append_char(&mut self, c: char);
    fn clear_contents(&mut self);
    fn drop_char(&mut self);
    fn remove_char(&mut self) -> Option<Action>;
    fn get_contents(&self) -> String;
}
