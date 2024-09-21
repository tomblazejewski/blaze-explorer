use std::path::PathBuf;

use ratatui::crossterm::event::KeyEvent;

use crate::{
    app::App,
    command::{
        ChangeDirectory, ClearSearchQuery, Command, ConfirmCommand, ConfirmSearchQuery,
        DeleteSelection, DropKey, EraseText, InsertKey, NextSearchResult, Noop, OpenPopup,
        ParentDirectory, Quit, SelectDirectory, SelectDown, SelectUp, ShowInFolder, SwitchMode,
        TelescopeConfirmResult, TelescopeDropSearchChar, TelescopeNextResult,
        TelescopePreviousResult, TelescopePushSearchChar, TelescopeUpdateSearchQuery,
        UpdateSearchQuery,
    },
    mode::Mode,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ExplorerAction {
    ChangeDirectory(PathBuf),
    ParentDirectory,
    SelectUp,
    SelectDown,
    SelectDirectory,
    UpdateSearchQuery(String),
    ClearSearchQuery,
    NextSearchResult,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppAction {
    Quit,
    SwitchMode(Mode),
    ConfirmSearchQuery,
    ConfirmCommand,
    OpenPopup,
    ShowInFolder(PathBuf),
    Delete,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextAction {
    InsertKey(char),
    EraseText,
    DropKey,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    ExplorerAct(ExplorerAction),
    AppAct(AppAction),
    TextAct(TextAction),
    Noop,
    TelescopeAct(TelescopeAction),
    CommandAct(CommandAction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TelescopeAction {
    ConfirmResult,
    PushSearchChar(char),
    DropSearchChar,
    NextResult,
    PreviousResult,
    Quit,
    EraseText,
    UpdateSearchQuery(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandAction {
    Undo,
    Redo,
}

pub fn get_command(app: &App, action: Action) -> Box<dyn Command> {
    let ctx = app.get_app_context();
    match action {
        Action::ExplorerAct(ExplorerAction::ChangeDirectory(path)) => {
            Box::new(ChangeDirectory::new(ctx, path))
        }
        Action::ExplorerAct(ExplorerAction::ParentDirectory) => Box::new(ParentDirectory::new(ctx)),
        Action::ExplorerAct(ExplorerAction::SelectUp) => Box::new(SelectUp::new(ctx)),
        Action::ExplorerAct(ExplorerAction::SelectDown) => Box::new(SelectDown::new(ctx)),
        Action::ExplorerAct(ExplorerAction::SelectDirectory) => Box::new(SelectDirectory::new(ctx)),
        Action::ExplorerAct(ExplorerAction::UpdateSearchQuery(s)) => {
            Box::new(UpdateSearchQuery::new(ctx, s))
        }
        Action::ExplorerAct(ExplorerAction::ClearSearchQuery) => {
            Box::new(ClearSearchQuery::new(ctx))
        }
        Action::ExplorerAct(ExplorerAction::NextSearchResult) => {
            Box::new(NextSearchResult::new(ctx))
        }
        Action::AppAct(AppAction::Quit) => Box::new(Quit::new()),
        Action::AppAct(AppAction::SwitchMode(mode)) => Box::new(SwitchMode::new(ctx, mode)),
        Action::AppAct(AppAction::ConfirmSearchQuery) => Box::new(ConfirmSearchQuery::new()),
        Action::AppAct(AppAction::ConfirmCommand) => Box::new(ConfirmCommand::new()),
        Action::AppAct(AppAction::OpenPopup) => Box::new(OpenPopup::new()),
        Action::AppAct(AppAction::ShowInFolder(path)) => Box::new(ShowInFolder::new(ctx, path)),
        Action::AppAct(AppAction::Delete) => Box::new(DeleteSelection::new(ctx)),
        Action::TextAct(TextAction::InsertKey(ch)) => Box::new(InsertKey::new(ch)),
        Action::TextAct(TextAction::EraseText) => Box::new(EraseText::new()),
        Action::TextAct(TextAction::DropKey) => Box::new(DropKey::new()),
        Action::TelescopeAct(TelescopeAction::ConfirmResult) => {
            Box::new(TelescopeConfirmResult::new())
        }
        Action::TelescopeAct(TelescopeAction::PushSearchChar(ch)) => {
            Box::new(TelescopePushSearchChar::new(ch))
        }
        Action::TelescopeAct(TelescopeAction::DropSearchChar) => {
            Box::new(TelescopeDropSearchChar::new())
        }
        Action::TelescopeAct(TelescopeAction::NextResult) => Box::new(TelescopeNextResult::new()),
        Action::TelescopeAct(TelescopeAction::PreviousResult) => {
            Box::new(TelescopePreviousResult::new())
        }
        Action::TelescopeAct(TelescopeAction::Quit) => Box::new(Quit::new()),
        Action::TelescopeAct(TelescopeAction::EraseText) => Box::new(EraseText::new()),
        Action::TelescopeAct(TelescopeAction::UpdateSearchQuery(query)) => {
            Box::new(TelescopeUpdateSearchQuery::new(query))
        }

        Action::Noop => Box::new(Noop::new()),
        Action::CommandAct(_) => Box::new(Noop::new()),
    }
}
