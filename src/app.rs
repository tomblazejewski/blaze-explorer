use std::collections::{HashMap, VecDeque};
use std::io::{stdout, Stdout};
use std::path;

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

use crate::action::{AppAction, ExplorerAction};
use crate::app_input_machine::AppInputMachine;
use crate::components::command_line::CommandLine;
use crate::focus::Focus;
use crate::input_machine::{InputMachine, KeyProcessingResult};
use crate::line_entry::LineEntry;
use crate::popup::{PopUp, PopUpWindow};
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
        self.handle_actions();
        loop {
            self.render();
            if let event::Event::Key(key) = event::read()? {
                //need to push the key to get the
                //keytracker to work?
                if key.kind == KeyEventKind::Press {
                    match &mut self.popup {
                        PopUp::TelescopePopUp(popup) => {
                            popup.handle_key_event(key);
                            popup.handle_actions();
                        }
                        PopUp::None => {
                            self.handle_key_event(key);
                        }
                    }
                };
                if let PopUp::TelescopePopUp(popup) = &self.popup {
                    if popup.should_quit {
                        info!("Should quit");
                        self.popup = PopUp::None;
                    }
                }
                if self.should_quit {
                    break;
                }
                self.handle_actions();
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

    fn execute_command(&mut self) -> Option<Action> {
        let command = self.command_line.pop_contents();
        match command.as_str() {
            "q" => Some(Action::AppAct(AppAction::Quit)),
            _ => None,
        }
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
    fn confirm_search_query(&mut self) -> Option<Action> {
        self.enter_normal_mode();
        Some(Action::ExplorerAct(ExplorerAction::NextSearchResult))
    }

    pub fn handle_self_actions(&mut self, action: AppAction) -> Option<Action> {
        match action {
            AppAction::SwitchMode(mode) => match mode {
                Mode::Normal => self.enter_normal_mode(),
                Mode::Search => self.enter_search_mode(),
                Mode::Command => self.enter_command_mode(),
            },
            AppAction::Quit => self.should_quit = true,
            AppAction::ConfirmSearchQuery => {
                return self.confirm_search_query();
            }
            AppAction::ConfirmCommand => {
                return self.execute_command();
            }
            AppAction::OpenPopup => {
                let ctx = self.get_app_context();
                self.popup = PopUp::TelescopePopUp(PopUpWindow::new(ctx));
            }
            AppAction::CancelKeybind => {
                self.popup = PopUp::None;
            }
        }
        None
    }
    pub fn handle_actions(&mut self) -> Result<()> {
        while let Some(action) = self.action_list.pop_front() {
            if let Some(resulting_action) = match action {
                Action::ExplorerAct(explorer_action) => {
                    self.explorer_table.explorer_action(explorer_action)
                }
                Action::AppAct(app_action) => self.handle_self_actions(app_action),
                Action::TextAct(text_action) => self.command_line.handle_text_action(text_action),
                _ => None,
            } {
                self.action_list.push_back(resulting_action);
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

    fn get_app_context(&self) -> AppContext {
        AppContext::new(self.explorer_table.get_current_path().clone())
    }
}
