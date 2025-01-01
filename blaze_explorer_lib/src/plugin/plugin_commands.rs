use crate::{action::Action, app::App, command::Command, match_popup_call};

//Contains standard commands that can be reused by plugins
#[derive(Clone, PartialEq, Debug)]
pub struct PluginQuit {}

impl PluginQuit {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for PluginQuit {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, quit)
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct PluginDropSearchChar {}

impl PluginDropSearchChar {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for PluginDropSearchChar {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, drop_search_char->Option<Action>)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PluginConfirmResult {}

impl PluginConfirmResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for PluginConfirmResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, confirm_result->Option<Action>)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PluginNextResult {}

impl PluginNextResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for PluginNextResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, next_result->Option<Action>)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PluginPreviousResult {}

impl PluginPreviousResult {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for PluginPreviousResult {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, previous_result->Option<Action>)
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct PluginPushSearchChar {
    ch: char,
}

impl PluginPushSearchChar {
    pub fn new(ch: char) -> Self {
        Self { ch }
    }
}

impl Command for PluginPushSearchChar {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, push_search_char, self.ch;->Option<Action>)
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct PluginEraseText {}

impl PluginEraseText {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for PluginEraseText {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        match_popup_call!(app, erase_text->Option<Action>)
    }
}
