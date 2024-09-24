use std::collections::{HashMap, VecDeque};
use std::io::{stdout, Stdout};
use std::path::{self, PathBuf};

use color_eyre::Result;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use ratatui::{
    crossterm::{
        event::{self, KeyEventKind},
        terminal::{disable_raw_mode, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::CrosstermBackend,
    Terminal,
};
use tracing::info;

use crate::action::{get_command, AppAction, CommandAction, ExplorerAction};
use crate::app_input_machine::AppInputMachine;
use crate::command::Command;
use crate::command_history::CommandHistory;
use crate::components::command_line::CommandLine;
use crate::focus::Focus;
use crate::input_machine::{InputMachine, KeyProcessingResult};
use crate::line_entry::LineEntry;
use crate::popup::{self, PopUp, PopupEngine};
use crate::telescope::AppContext;
use crate::tools::center_rect;
use crate::{
    action::Action,
    components::{explorer_table::ExplorerTable, Component},
    mode::Mode,
};

fn get_component_areas(frame: &mut Frame) -> HashMap<String, Rect> {
    let main_box = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.size());
    let command_bar = main_box[1];

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
pub struct App {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub action_list: VecDeque<Action>,
    pub should_quit: bool,
    pub mode: Mode,
    pub explorer_table: ExplorerTable,
    pub command_line: CommandLine,
    pub focus: Focus,
    pub current_sequence: Vec<KeyEvent>,
    pub input_machine: AppInputMachine<Action>,
    pub popup: PopUp,
    pub command_history: HashMap<PathBuf, CommandHistory>,
    pub command_input: Option<String>,
}
impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            action_list: VecDeque::new(),
            should_quit: false,
            mode: Mode::Normal,
            explorer_table: ExplorerTable::new(),
            command_line: CommandLine::new(),
            focus: Focus::ExplorerTable,
            current_sequence: Vec::new(),
            input_machine: AppInputMachine::new(),
            popup: PopUp::None,
            command_history: HashMap::new(),
            command_input: None,
        })
    }

    /// Send a key event to the appropriate component based on the current mode
    pub fn queue_key_event(&mut self, action: Action) {
        self.action_list.push_back(action);
    }
    pub fn run(&mut self) -> Result<()> {
        self.terminal.clear()?;
        let path = "./";
        let starting_path = path::absolute(path).unwrap();
        self.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                starting_path,
            )));
        self.handle_new_actions();
        loop {
            self.render();
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match &mut self.popup {
                        PopUp::TelescopePopUp(popup) => {
                            if let Some(action) = popup.handle_key_event(key) {
                                self.action_list.push_back(action);
                            }
                        }
                        PopUp::None => {
                            self.handle_key_event(key);
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
                        }
                    }
                }
                if self.should_quit {
                    break;
                }
                self.handle_new_actions();
            }
        }
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        let keymap_result =
            self.input_machine
                .process_keys(&self.mode, &mut self.current_sequence, key_event);
        info!("Keymap result: {:?}", keymap_result);
        match keymap_result {
            KeyProcessingResult::Complete(action) => {
                info!("Complete Action: {:?}", action);
                self.action_list.push_back(action);
            }
            KeyProcessingResult::Invalid => {
                if let Some(action) = self.input_machine.get_default_action(&self.mode, key_event) {
                    info!("Default Action: {:?}", action);
                    self.action_list.push_back(action);
                }
            }
            _ => {}
        }
    }

    pub fn execute_command(&mut self) -> Option<Action> {
        let command = self.command_line_contents();
        match command.as_str() {
            "q" => Some(Action::AppAct(AppAction::Quit)),
            _ => None,
        }
    }

    pub fn open_default(&self, path: PathBuf) {
        open::that(path).unwrap();
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
        self.explorer_table.switch_mode(Mode::Search);
        self.command_line.focus();
        self.explorer_table.unfocus();
    }

    pub fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.explorer_table.switch_mode(Mode::Normal);
        self.command_line.unfocus();
        self.explorer_table.focus();
    }

    pub fn enter_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.explorer_table.switch_mode(Mode::Command);
        self.command_line.focus();
        self.explorer_table.unfocus();
    }

    pub fn confirm_search_query(&mut self) -> Option<Action> {
        self.enter_normal_mode();
        Some(Action::ExplorerAct(ExplorerAction::NextSearchResult))
    }

    pub fn record_command(&mut self, command: Box<dyn Command>) {
        let current_path = self.explorer_table.get_current_path();
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
        let path = self.explorer_table.get_current_path();
        let path_history = self.command_history.get_mut(&path);
        let command = path_history.unwrap().undo();
        if let Some(mut c) = command {
            c.undo(self);
        }
    }
    fn redo(&mut self) {
        let path = self.explorer_table.get_current_path();
        let path_history = self.command_history.get_mut(&path);
        let command = path_history.unwrap().redo();
        if let Some(mut c) = command {
            c.execute(self);
        }
    }

    pub fn run_command(&mut self, mut command: Box<dyn Command>) {
        self.record_command(command.clone());
        if let Some(action) = command.execute(self) {
            self.action_list.push_back(action);
        }
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
        Ok(())
    }
    pub fn render(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let areas = get_component_areas(frame);
            self.explorer_table
                .draw(frame, *areas.get("explorer_table").unwrap());
            self.command_line
                .draw(frame, *areas.get("command_line").unwrap());
            self.popup.draw(frame, *areas.get("popup").unwrap());
        })?;
        Ok(())
    }

    pub fn get_app_context(&self) -> AppContext {
        AppContext::new(
            self.explorer_table.get_current_path().clone(),
            self.explorer_table.clone(),
        )
    }

    pub fn update_path(&mut self, path: PathBuf) {
        self.explorer_table.update_path(path.clone())
    }

    pub fn command_line_message(&mut self, msg: String) {
        self.command_line.command_line_message(msg);
    }

    pub fn command_line_contents(&self) -> String {
        self.command_line.get_contents()
    }
}
