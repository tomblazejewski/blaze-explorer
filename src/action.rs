use crate::command::{
    DeleteSplit, DisplayMessage, FocusDown, FocusLeft, FocusRight, FocusUp, JumpAndClose,
    JumpAndOpen, JumpToId, OpenNeovimHere, ParseCommand, RedoDirectory, SplitHorizontally,
    SplitVertically, TelescopeQuit, TerminalCommand, UndoDirectory, UpdatePlugin,
};
use std::path::PathBuf;

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
pub enum PopupType {
    None,
    Telescope,
    Rename,
    Flash(bool),
}

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
    SplitHorizontally,
    SplitVertically,
    FocusUp,
    FocusDown,
    FocusLeft,
    FocusRight,
    DeleteSplit,
    JumpToId(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppAction {
    Quit,
    SwitchMode(Mode),
    ConfirmSearchQuery,
    ConfirmCommand,
    OpenPopup(PopupType),
    ShowInFolder(PathBuf),
    Delete,
    OpenNeovimHere,
    DisplayMessage(String),
    TerminalCommand(String),
    UndoDirectory,
    RedoDirectory,
    ParseCommand(String),
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
    PopupAct(PopupAction),
    CommandAct(CommandAction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PopupAction {
    ConfirmResult,
    PushSearchChar(char),
    DropSearchChar,
    NextResult,
    PreviousResult,
    Quit,
    EraseText,
    UpdateSearchQuery(String),
    UpdatePlugin,
    JumpAndClose(usize),
    JumpAndOpen(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandAction {
    Undo,
    Redo,
}

pub fn get_command(app: &mut App, action: Action) -> Box<dyn Command> {
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
        Action::ExplorerAct(ExplorerAction::SplitHorizontally) => {
            Box::new(SplitHorizontally::new(ctx))
        }
        Action::ExplorerAct(ExplorerAction::SplitVertically) => Box::new(SplitVertically::new(ctx)),
        Action::ExplorerAct(ExplorerAction::FocusUp) => Box::new(FocusUp::new(ctx)),
        Action::ExplorerAct(ExplorerAction::FocusDown) => Box::new(FocusDown::new(ctx)),
        Action::ExplorerAct(ExplorerAction::FocusLeft) => Box::new(FocusLeft::new(ctx)),
        Action::ExplorerAct(ExplorerAction::FocusRight) => Box::new(FocusRight::new(ctx)),
        Action::ExplorerAct(ExplorerAction::DeleteSplit) => Box::new(DeleteSplit::new(ctx)),
        Action::ExplorerAct(ExplorerAction::JumpToId(id)) => Box::new(JumpToId::new(ctx, id)),
        Action::AppAct(AppAction::Quit) => Box::new(Quit::new()),
        Action::AppAct(AppAction::SwitchMode(mode)) => Box::new(SwitchMode::new(ctx, mode)),
        Action::AppAct(AppAction::ConfirmSearchQuery) => Box::new(ConfirmSearchQuery::new()),
        Action::AppAct(AppAction::ConfirmCommand) => Box::new(ConfirmCommand::new(ctx)),
        Action::AppAct(AppAction::OpenPopup(popup_type)) => Box::new(OpenPopup::new(popup_type)),
        Action::AppAct(AppAction::ShowInFolder(path)) => Box::new(ShowInFolder::new(ctx, path)),
        Action::AppAct(AppAction::Delete) => Box::new(DeleteSelection::new(ctx)),
        Action::AppAct(AppAction::OpenNeovimHere) => Box::new(OpenNeovimHere::new(ctx)),
        Action::AppAct(AppAction::DisplayMessage(msg)) => Box::new(DisplayMessage::new(msg)),
        Action::AppAct(AppAction::TerminalCommand(cmd)) => Box::new(TerminalCommand::new(ctx, cmd)),
        Action::AppAct(AppAction::UndoDirectory) => Box::new(UndoDirectory::new(ctx)),
        Action::AppAct(AppAction::RedoDirectory) => Box::new(RedoDirectory::new(ctx)),
        Action::AppAct(AppAction::ParseCommand(command)) => {
            Box::new(ParseCommand::new(ctx, command))
        }
        Action::TextAct(TextAction::InsertKey(ch)) => Box::new(InsertKey::new(ch)),
        Action::TextAct(TextAction::EraseText) => Box::new(EraseText::new()),
        Action::TextAct(TextAction::DropKey) => Box::new(DropKey::new()),
        Action::PopupAct(PopupAction::ConfirmResult) => Box::new(TelescopeConfirmResult::new()),
        Action::PopupAct(PopupAction::PushSearchChar(ch)) => {
            Box::new(TelescopePushSearchChar::new(ch))
        }
        Action::PopupAct(PopupAction::DropSearchChar) => Box::new(TelescopeDropSearchChar::new()),
        Action::PopupAct(PopupAction::NextResult) => Box::new(TelescopeNextResult::new()),
        Action::PopupAct(PopupAction::PreviousResult) => Box::new(TelescopePreviousResult::new()),
        Action::PopupAct(PopupAction::Quit) => Box::new(TelescopeQuit::new()),
        Action::PopupAct(PopupAction::EraseText) => Box::new(EraseText::new()),
        Action::PopupAct(PopupAction::UpdateSearchQuery(query)) => {
            Box::new(TelescopeUpdateSearchQuery::new(query))
        }
        Action::PopupAct(PopupAction::UpdatePlugin) => Box::new(UpdatePlugin::new()),
        Action::PopupAct(PopupAction::JumpAndClose(id)) => Box::new(JumpAndClose::new(id)),
        Action::PopupAct(PopupAction::JumpAndOpen(id)) => Box::new(JumpAndOpen::new(id)),
        Action::Noop => Box::new(Noop::new()),
        Action::CommandAct(_) => Box::new(Noop::new()),
    }
}
