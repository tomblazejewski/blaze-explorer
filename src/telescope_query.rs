use crate::{
    action::{Action, ExplorerAction, PopupAction, TextAction},
    line_entry::LineEntry,
};

pub struct TelescopeQuery {
    pub contents: String,
}

impl LineEntry for TelescopeQuery {
    fn pop_contents(&mut self) -> String {
        self.contents.drain(..).collect()
    }

    fn append_char(&mut self, c: char) {
        self.contents.push(c);
    }

    fn clear_contents(&mut self) {
        self.contents.clear();
    }

    fn drop_char(&mut self) {
        self.contents.pop();
    }

    fn remove_char(&mut self) -> Option<Action> {
        self.contents.pop();
        Some(Action::PopupAct(PopupAction::UpdateSearchQuery(
            self.contents.clone(),
        )))
    }

    fn get_contents(&self) -> String {
        self.contents.clone()
    }
}

impl TelescopeQuery {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
        }
    }
    pub fn handle_text_action(&mut self, action: PopupAction) -> Option<Action> {
        match action {
            PopupAction::PushSearchChar(c) => self.append_char(c),
            PopupAction::EraseText => self.clear_contents(),
            PopupAction::DropSearchChar => return self.remove_char(),
            _ => {}
        }
        Some(Action::PopupAct(PopupAction::UpdateSearchQuery(
            self.contents.clone(),
        )))
    }
}
