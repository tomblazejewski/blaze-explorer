use std::collections::{HashMap, VecDeque};
use std::env::set_current_dir;
use std::io::{stdout, Stdout};
use std::path::{self, PathBuf};

use color_eyre::Result;
use ratatui::crossterm::event::{Event, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use ratatui::{
    crossterm::event::{self, KeyEventKind},
    prelude::CrosstermBackend,
    Terminal,
};

use crate::action::{get_command, AppAction, CommandAction, ExplorerAction, PopupAction};
use crate::app_input_machine::AppInputMachine;
use crate::command::Command;
use crate::components::command_line::CommandLine;
use crate::components::explorer_manager::ExplorerManager;
use crate::history_stack::directory_history::DirectoryDetails;
use crate::history_stack::{command_history::CommandHistory, HistoryStack};
use crate::input_machine::{InputMachine, KeyProcessingResult};
use crate::line_entry::LineEntry;
use crate::plugin::Plugin;
use crate::popup::{PopUp, PopupEngine};
use crate::telescope::AppContext;
use crate::tools::center_rect;
use crate::{action::Action, components::Component, mode::Mode};

#[derive(Clone, PartialEq, Debug)]
pub enum ExitResult {
    Quit,
    OpenTerminal(PathBuf),
    OpenNeovim(PathBuf),
}
fn get_component_areas(frame: &mut Frame) -> HashMap<String, Rect> {
    let main_box = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.size());
    let command_box = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(30)])
        .split(frame.size());
    let command_bar = command_box[1];

    let mut areas = HashMap::new();
    areas.insert("explorer_table".to_string(), main_box[0]);
    areas.insert("command_line".to_string(), command_bar);

    let popup_area = center_rect(
        frame.size(),
        Constraint::Percentage(80),
        Constraint::Percentage(80),
    );
    areas.insert("popup".to_string(), popup_area);
    areas
}

#[derive(Debug)]
pub struct App {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub action_list: VecDeque<Action>,
    pub should_quit: bool,
    pub mode: Mode,
    pub explorer_manager: ExplorerManager,
    pub command_line: CommandLine,
    pub current_sequence: Vec<KeyEvent>,
    pub input_machine: AppInputMachine<Action>,
    pub popup: PopUp,
    pub command_history: HashMap<PathBuf, CommandHistory>,
    pub command_input: Option<String>,
    pub exit_status: Option<ExitResult>,
    pub current_path: PathBuf,
    pub key_queue: VecDeque<KeyEvent>,
}
impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            action_list: VecDeque::new(),
            should_quit: false,
            mode: Mode::Normal,
            explorer_manager: ExplorerManager::new(),
            command_line: CommandLine::new(),
            current_sequence: Vec::new(),
            input_machine: AppInputMachine::new(),
            popup: PopUp::None,
            command_history: HashMap::new(),
            command_input: None,
            exit_status: None,
            current_path: PathBuf::new(),
            key_queue: VecDeque::new(),
        })
    }

    /// Send a key event to the appropriate component based on the current mode
    pub fn queue_key_event(&mut self, action: Action) {
        self.action_list.push_back(action);
    }

    pub fn draw_key_event(&mut self) -> std::io::Result<Event> {
        match self.key_queue.pop_front() {
            Some(key) => Ok(event::Event::Key(key)),
            None => event::read(),
        }
    }
    pub fn run(&mut self, cold_start: bool) -> Result<ExitResult> {
        self.terminal.clear()?;
        if cold_start {
            let path = "./";
            let starting_path = path::absolute(path).unwrap();
            self.own_push_action(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                starting_path,
            )));
        }
        let _ = self.handle_new_actions();
        loop {
            let _ = self.render();
            if let event::Event::Key(key) = self.draw_key_event()? {
                if key.kind == KeyEventKind::Press {
                    match &mut self.popup {
                        PopUp::TelescopePopUp(popup) => {
                            if let Some(action) = popup.handle_key_event(key) {
                                self.own_push_action(action);
                            }
                        }
                        PopUp::None => {
                            self.handle_key_event(key);
                        }
                        PopUp::InputPopUp(input) => {
                            if let Some(action) = input.handle_key_event(key) {
                                self.own_push_action(action);
                            }
                        }
                        PopUp::FlashPopUp(flashpopup) => {
                            if let Some(action) = flashpopup.handle_key_event(key) {
                                self.own_push_action(action);
                            }
                            self.own_push_action(Action::PopupAct(PopupAction::UpdatePlugin));
                        }
                    }
                };
                match &self.popup {
                    PopUp::None => {}
                    active_popup => {
                        if active_popup.should_quit() {
                            if let Some(command) = active_popup.destruct() {
                                self.run_command(command);
                            }
                            self.popup = PopUp::None;
                            self.explorer_manager.set_plugin_display(None);
                        }
                    }
                }
                if self.should_quit {
                    break;
                }
                let _ = self.handle_new_actions();
            }
        }

        self.exit_status
            .clone()
            .map(Ok)
            .unwrap_or(Ok(ExitResult::Quit))
    }

    pub fn own_push_action(&mut self, action: Action) {
        self.action_list.push_back(action);
    }

    /// Register the key event, obtain possible action and push it back if applicable.
    /// Command line message takes precedence over register actions by the input machine
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        let keymap_result =
            self.input_machine
                .process_keys(&self.mode, &mut self.current_sequence, key_event);
        if self.command_line.current_message.is_some() {
            self.command_line.get_message_batch();
            if self.command_line.current_message.is_none() {
                self.own_push_action(Action::AppAct(AppAction::SwitchMode(Mode::Normal)));
            }
        } else {
            match keymap_result {
                KeyProcessingResult::Complete(action) => {
                    self.own_push_action(action);
                }
                KeyProcessingResult::Invalid => {
                    if let Some(action) =
                        self.input_machine.get_default_action(&self.mode, key_event)
                    {
                        self.own_push_action(action);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn execute_command(&mut self, command: String) -> Option<Action> {
        match command.as_str() {
            "q" => Some(Action::ExplorerAct(ExplorerAction::DeleteSplit)),
            "t" => Some(Action::AppAct(AppAction::ParseKeyStrokes(":".into()))),
            other_command => Some(Action::AppAct(AppAction::DisplayMessage(format!(
                "Not a supported command: {}",
                other_command
            )))),
        }
    }

    pub fn execute_keys(&mut self, enigo_keys: Vec<KeyEvent>) {
        for key in enigo_keys {
            self.key_queue.push_back(key);
        }
    }

    pub fn parse_command(&mut self, command: String) {
        self.enter_command_mode();
        self.command_line.set_contents(command);
        self.own_push_action(Action::AppAct(AppAction::ConfirmCommand));
    }

    pub fn open_default(&self, path: PathBuf) {
        open::that(path).unwrap();
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
        self.explorer_manager.switch_mode(Mode::Search);
        self.command_line.focus();
        self.explorer_manager.unfocus();
    }

    pub fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.explorer_manager.switch_mode(Mode::Normal);
        self.command_line.unfocus();
        self.explorer_manager.focus();
    }

    pub fn enter_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.explorer_manager.switch_mode(Mode::Command);
        self.command_line.focus();
        self.explorer_manager.unfocus();
    }

    pub fn confirm_search_query(&mut self) -> Option<Action> {
        self.enter_normal_mode();
        Some(Action::ExplorerAct(ExplorerAction::NextSearchResult))
    }

    pub fn record_command(&mut self, command: Box<dyn Command>) {
        let current_path = self.explorer_manager.get_current_path();
        let c_history = self.command_history.get_mut(&current_path);
        if let Some(history) = c_history {
            if command.is_reversible() {
                history.perform(command);
            }
        } else {
            let mut history = CommandHistory::new();
            if command.is_reversible() {
                history.perform(command);
            }
            self.command_history.insert(current_path, history);
        }
    }
    fn undo(&mut self) {
        let path = self.explorer_manager.get_current_path();
        let path_history = self.command_history.get_mut(&path);
        let command = path_history.unwrap().undo();
        if let Some(mut c) = command {
            c.undo(self);
        }
    }
    fn redo(&mut self) {
        let path = self.explorer_manager.get_current_path();
        let path_history = self.command_history.get_mut(&path);
        let command = path_history.unwrap().redo();
        if let Some(mut c) = command {
            c.execute(self);
        }
    }

    pub fn run_command(&mut self, mut command: Box<dyn Command>) {
        if let Some(action) = command.execute(self) {
            self.action_list.push_back(action);
        }
        //Record the command after execution (execution can mutate the command)
        self.record_command(command.clone());
    }
    pub fn handle_new_actions(&mut self) -> Result<()> {
        while let Some(action) = self.action_list.pop_front() {
            match action {
                Action::CommandAct(CommandAction::Undo) => self.undo(),
                Action::CommandAct(CommandAction::Redo) => self.redo(),
                _ => {
                    let command = get_command(self, action.clone());
                    self.run_command(command);
                }
            }
        }
        self.handle_post_actions()?;
        Ok(())
    }

    pub fn handle_post_actions(&mut self) -> Result<()> {
        if let PopUp::FlashPopUp(flashpopup) = &self.popup {
            self.explorer_manager
                .set_plugin_display(Some(flashpopup.display_details()));
        } else {
            self.explorer_manager.set_plugin_display(None);
        }
        Ok(())
    }

    pub fn undo_directory(&mut self) {
        let directory_history = self.explorer_manager.get_directory_history();
        let new_details = directory_history.undo();
        if let Some(nd) = new_details {
            self.update_path(nd.directory, nd.selected);
        }
    }
    pub fn redo_directory(&mut self) {
        let directory_history = self.explorer_manager.get_directory_history();
        let new_details = directory_history.redo();
        if let Some(nd) = new_details {
            self.update_path(nd.directory, nd.selected);
        }
    }

    pub fn move_directory(&mut self, path: PathBuf, selected: Option<String>) {
        let directory_history = self.explorer_manager.get_directory_history();
        // Save the directory to be entered
        directory_history.perform(DirectoryDetails {
            directory: path.clone(),
            selected: selected.clone(),
        });
        self.update_path(path, selected);
    }

    pub fn render(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let areas = get_component_areas(frame);
            self.explorer_manager.draw(
                frame,
                *areas.get("explorer_table").unwrap(),
                self.current_sequence.clone(),
            );
            let _ = self
                .command_line
                .draw(frame, *areas.get("command_line").unwrap());
            let _ = self.popup.draw(frame, *areas.get("popup").unwrap());
        })?;
        Ok(())
    }

    pub fn get_app_context(&mut self) -> AppContext {
        AppContext::new(
            self.explorer_manager.get_current_path().clone(),
            self.explorer_manager.clone(),
            self.command_line_contents().clone(),
            self.mode.clone(),
        )
    }

    pub fn update_path(&mut self, path: PathBuf, selected: Option<String>) {
        let abs_path = path::absolute(path.clone());
        match abs_path {
            Ok(abs_path) => {
                self.current_path = abs_path.clone();
                self.explorer_manager
                    .update_path(abs_path.clone(), selected);
                let _ = set_current_dir(abs_path);
            }
            Err(err) => {
                self.command_line_message(err.to_string());
            }
        }
    }

    pub fn command_line_message(&mut self, msg: String) {
        self.command_line.command_line_message(msg);
    }

    pub fn command_line_contents(&self) -> String {
        self.command_line.get_contents()
    }

    pub fn go_up(&mut self) {
        let prev_folder = self.current_path.file_name().map(|name| name.to_owned());
        if let Some(prev_folder_name) = prev_folder {
            let prev_folder_string = prev_folder_name.to_str().unwrap();
            let new_absolute_path = self.current_path.parent().unwrap().to_owned();
            self.move_directory(new_absolute_path, Some(prev_folder_string.to_string()));
        }
    }
}

impl Clone for App {
    fn clone(&self) -> Self {
        App {
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            action_list: VecDeque::new(),
            should_quit: self.should_quit,
            mode: self.mode.clone(),
            explorer_manager: self.explorer_manager.clone(),
            command_line: self.command_line.clone(),
            current_sequence: self.current_sequence.clone(),
            input_machine: self.input_machine.clone(),
            popup: self.popup.clone(),
            command_history: self.command_history.clone(),
            command_input: self.command_input.clone(),
            exit_status: self.exit_status.clone(),
            current_path: self.current_path.clone(),
            key_queue: self.key_queue.clone(),
        }
    }
}

impl PartialEq for App {
    fn eq(&self, other: &Self) -> bool {
        self.action_list == other.action_list
            && self.should_quit == other.should_quit
            && self.mode == other.mode
            && self.explorer_manager == other.explorer_manager
            && self.command_line == other.command_line
            && self.current_sequence == other.current_sequence
            && self.input_machine == other.input_machine
            && self.popup == other.popup
            && self.command_history == other.command_history
            && self.command_input == other.command_input
            && self.exit_status == other.exit_status
            && self.current_path == other.current_path
            && self.key_queue == other.key_queue
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_move_directory() {
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        let abs_path = path::absolute("tests/").unwrap();
        app.move_directory(abs_path.clone(), None);
        assert_eq!(app.explorer_manager.get_current_path(), abs_path);
        app.move_directory(starting_path, None);
    }

    #[test]
    fn test_undo_directory() {
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        app.move_directory(starting_path.clone(), None);
        let abs_path = path::absolute("tests/").unwrap();
        app.move_directory(abs_path.clone(), None);
        app.undo_directory();
        assert_eq!(app.explorer_manager.get_current_path(), starting_path);
        app.move_directory(starting_path, None);
    }
    #[test]
    fn test_redo_directory() {
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        app.move_directory(starting_path.clone(), None);
        let abs_path = path::absolute("tests/").unwrap();
        app.move_directory(abs_path.clone(), None);
        app.undo_directory();
        assert_eq!(app.explorer_manager.get_current_path(), starting_path);
        app.redo_directory();
        assert_eq!(app.explorer_manager.get_current_path(), abs_path);
        app.move_directory(starting_path, None);
    }
}
