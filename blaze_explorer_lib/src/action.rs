use crate::{
    command::{
        DeleteSplit, DisplayMessage, ExecuteFunction, FocusDown, FocusLeft, FocusRight, FocusUp,
        JumpToId, OpenNeovimHere, ParseCommand, ParseKeyStrokes, RedoDirectory, SplitHorizontally,
        SplitVertically, TerminalCommand, UndoDirectory, UpdatePlugin, UpdatePopup,
    },
    plugin::{plugin_action::PluginAction, plugin_popup::PluginPopUp},
};
use std::path::PathBuf;

use crate::app::App;
use crate::{
    command::{
        ChangeDirectory, ClearSearchQuery, Command, ConfirmCommand, ConfirmSearchQuery,
        DeleteSelection, DropKey, EraseText, InsertKey, NextSearchResult, Noop, ParentDirectory,
        Quit, SelectDirectory, SelectDown, SelectUp, ShowInFolder, SwitchMode, UpdateSearchQuery,
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
    OpenPopup(Box<dyn PluginPopUp>),
    ShowInFolder(PathBuf),
    Delete,
    OpenNeovimHere,
    DisplayMessage(String),
    TerminalCommand(String),
    UndoDirectory,
    RedoDirectory,
    ParseKeyStrokes(String),
    ParseCommand(String),
    ExecuteFunction(Box<fn(&mut App) -> Option<Action>>),
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
    PluginAct(PluginAction),
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
    UpdatePopup,
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
        Action::AppAct(AppAction::OpenPopup(popup_type)) => {
            panic!(
                "Trying to use OpenPopUp to open a {:?} popup - this will not work",
                popup_type
            );
        }
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
        Action::AppAct(AppAction::ExecuteFunction(function)) => {
            Box::new(ExecuteFunction::new(ctx, function))
        }
        Action::AppAct(AppAction::ParseKeyStrokes(command)) => {
            Box::new(ParseKeyStrokes::new(ctx, command))
        }
        Action::TextAct(TextAction::InsertKey(ch)) => Box::new(InsertKey::new(ctx, ch)),
        Action::TextAct(TextAction::EraseText) => Box::new(EraseText::new()),
        Action::TextAct(TextAction::DropKey) => Box::new(DropKey::new()),
        Action::PopupAct(PopupAction::UpdatePlugin) => Box::new(UpdatePlugin::new()),
        Action::PopupAct(PopupAction::UpdatePopup) => Box::new(UpdatePopup::new()),
        Action::Noop => Box::new(Noop::new()),
        Action::CommandAct(_) => Box::new(Noop::new()),
        Action::PluginAct(plugin_action) => plugin_action.get_command(),
        action => panic!("Action {:?} not implemented", action),
    }
}
