use crate::{
    action::{Action, ExplorerAction, TelescopeAction, TextAction},
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
        Some(Action::TelescopeAct(TelescopeAction::UpdateSearchQuery(
            self.contents.clone(),
        )))
    }
}

impl TelescopeQuery {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
        }
    }
    pub fn handle_text_action(&mut self, action: TelescopeAction) -> Option<Action> {
        match action {
            TelescopeAction::PushSearchChar(c) => self.append_char(c),
            TelescopeAction::EraseText => self.clear_contents(),
            TelescopeAction::DropSearchChar => return self.remove_char(),
            _ => {}
        }
        Some(Action::TelescopeAct(TelescopeAction::UpdateSearchQuery(
            self.contents.clone(),
        )))
    }
}
