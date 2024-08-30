use crate::{
    action::{Action, ExplorerAction, TelescopeAction, TextAction},
    line_entry::LineEntry,
};

pub struct TelescopeQuery {
    pub contents: String,
}

impl LineEntry<TelescopeAction> for TelescopeQuery {
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

    fn remove_char(&mut self) -> Option<TelescopeAction> {
        self.contents.pop();
        Some(TelescopeAction::UpdateSearchQuery(self.contents.clone()))
    }
}

impl TelescopeQuery {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
        }
    }
    pub fn handle_text_action(&mut self, action: TelescopeAction) -> Option<TelescopeAction> {
        match action {
            TelescopeAction::PushSearchChar(c) => self.append_char(c),
            TelescopeAction::EraseText => self.clear_contents(),
            TelescopeAction::DropSearchChar => return self.remove_char(),
            _ => {}
        }
        Some(TelescopeAction::UpdateSearchQuery(self.contents.clone()))
    }
}
