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
use tracing::field::debug;
use tracing::info;

use crate::action::{AppAction, ExplorerAction};
use crate::components::command_line::CommandLine;
use crate::focus::Focus;
use crate::input_machine::{
    default_key_map, process_keys, InputMachine, KeyMapNode, KeyProcessingResult,
};
use crate::key_handler::KeyHandler;
use crate::{
    action::Action,
    components::{explorer_table::ExplorerTable, Component},
    mode::Mode,
};

fn get_component_areas(frame: &mut Frame) -> HashMap<String, Rect> {
    let main_box = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(85),
            Constraint::Percentage(10),
            Constraint::Percentage(5),
        ])
        .split(frame.size());
    let status_bar = main_box[1];
    let status_bar_parts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(status_bar);
    let command_bar = main_box[2];

    let mut areas = HashMap::new();
    areas.insert("explorer_table".to_string(), main_box[0]);
    areas.insert("key_tracker".to_string(), status_bar_parts[2]);
    areas.insert("mode_display".to_string(), status_bar_parts[0]);
    areas.insert("command_line".to_string(), command_bar);
    areas
}
pub struct App {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub action_list: VecDeque<Action>,
    pub should_quit: bool,
    pub mode: Mode,
    pub command_line_manager: CommandLine,
    pub explorer_table: ExplorerTable,
    pub command_line: CommandLine,
    pub focus: Focus,
    pub current_sequence: Vec<KeyEvent>,
    pub input_machine: InputMachine,
}
impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            action_list: VecDeque::new(),
            should_quit: false,
            mode: Mode::Normal,
            command_line_manager: CommandLine::new(),
            explorer_table: ExplorerTable::new(),
            command_line: CommandLine::new(),
            focus: Focus::ExplorerTable,
            current_sequence: Vec::new(),
            input_machine: InputMachine::new(),
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
                info!("Pushed {:?}", key);
                if key.kind == KeyEventKind::Press {
                    self.handle_key_event(key);
                };
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
        if let KeyProcessingResult::Complete(action) = keymap_result {
            info!("Pushed {:?}", action);
            self.action_list.push_back(action);
        }
    }

    pub fn handle_self_actions(&mut self, action: AppAction) -> Option<Action> {
        match action {
            AppAction::SwitchMode(mode) => {
                self.command_line_manager.clear_key_events();
                self.mode = mode.clone();
            }
            AppAction::Quit => self.should_quit = true,
            _ => return None,
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
        })?;
        Ok(())
    }
}
