use crate::command::command_helpers::match_popup_call;
use crate::{action::Action, app::App, command::Command};

#[derive(Clone, PartialEq, Debug)]
pub struct TelescopeConfirmResult {}

impl TelescopeConfirmResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeConfirmResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, confirm_result)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TelescopeNextResult {}

impl TelescopeNextResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeNextResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, next_result)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TelescopePreviousResult {}

impl TelescopePreviousResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopePreviousResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, previous_result)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TelescopeUpdateSearchQuery {
    query: String,
}

impl TelescopeUpdateSearchQuery {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}
impl Command for TelescopeUpdateSearchQuery {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match &mut app.popup {
            None => None,
            Some(ref mut popup) => {
                popup.update_search_query(self.query.clone());
                None
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TelescopePushSearchChar {
    ch: char,
}

impl TelescopePushSearchChar {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }
}

impl Command for TelescopePushSearchChar {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, push_search_char, self.ch)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TelescopeDropSearchChar {}

impl TelescopeDropSearchChar {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeDropSearchChar {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, drop_search_char)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TelescopeQuit {}

impl TelescopeQuit {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeQuit {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, quit)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TelescopeEraseText {}

impl TelescopeEraseText {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for TelescopeEraseText {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, erase_text)
    }
}
