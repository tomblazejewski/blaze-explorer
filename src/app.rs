use std::collections::{HashMap, VecDeque};
use std::io::{stdout, Stdout};
use std::path;

use color_eyre::Result;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use ratatui::{
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::CrosstermBackend,
    Terminal,
};
use tracing::info;

use crate::action::{AppAction, ExplorerAction, KeyAction};
use crate::components::command_line::CommandLine;
use crate::components::{self, key_tracker};
use crate::key_combination::KeyManager;
use crate::key_handler::KeyHandler;
use crate::{
    action::Action,
    components::{
        explorer_table::ExplorerTable, key_tracker::KeyTracker, mode_display::ModeDisplay,
        path_display::PathDisplay, Component,
    },
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
    areas.insert("path_display".to_string(), status_bar_parts[1]);
    areas.insert("mode_display".to_string(), status_bar_parts[0]);
    areas.insert("command_line".to_string(), command_bar);
    areas
}
pub struct App {
    pub components: HashMap<String, Box<dyn Component>>,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub action_list: VecDeque<Action>,
    pub key_manager: KeyManager,
    pub should_quit: bool,
    pub mode: Mode,
    pub command_line_manager: CommandLine,
}
impl App {
    pub fn new() -> Result<Self> {
        let components_created: HashMap<String, Box<dyn Component>> = HashMap::from([
            (
                String::from("explorer_table"),
                Box::new(ExplorerTable::new()) as Box<dyn Component>,
            ),
            (String::from("key_tracker"), Box::new(KeyTracker::new())),
            (String::from("path_display"), Box::new(PathDisplay::new())),
            (String::from("command_line"), Box::new(CommandLine::new())),
            (String::from("mode_display"), Box::new(ModeDisplay::new())),
        ]);
        Ok(Self {
            components: components_created,
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            action_list: VecDeque::new(),
            key_manager: KeyManager::new(),
            should_quit: false,
            mode: Mode::Normal,
            command_line_manager: CommandLine::new(),
        })
    }

    /// Send a key event to the appropriate component based on the current mode
    pub fn redirect_key_event(&mut self, key_event: KeyEvent) {
        match self.mode {
            Mode::Normal => {
                self.key_manager.append_key_event(key_event);
            }
            Mode::Search => {
                self.command_line_manager.append_key_event(key_event);
            }
        }
    }
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
                    self.action_list
                        .push_back(Action::KeyAct(KeyAction::Key(key)));
                    self.key_manager.append_key_event(key);
                };
                self.handle_key_event();
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

    pub fn handle_key_event(&mut self) -> Option<Action> {
        let actions = self.key_manager.handle_keymap();
        self.action_list.extend(actions);

        None
    }

    pub fn handle_self_actions(&mut self, action: Action) -> Result<()> {
        match action {
            Action::KeyAct(KeyAction::EscapeSequence) => {
                self.key_manager.clear_keys_stored();
            }
            Action::AppAct(AppAction::SwitchMode(mode)) => {
                self.key_manager.clear_keys_stored();
                self.mode = mode.clone();
                self.key_manager.switch_mode(mode);
            }
            Action::KeyAct(KeyAction::ClearAndKey(key_event)) => {
                self.key_manager.clear_and_enter(key_event)
            }
            Action::AppAct(AppAction::Quit) => self.should_quit = true,
            _ => {}
        }
        Ok(())
    }
    pub fn handle_actions(&mut self) -> Result<()> {
        while let Some(action) = self.action_list.pop_front() {
            self.handle_self_actions(action.clone());
            for (_component_name, component) in self.components.iter_mut() {
                if let Ok(Some(resulting_action)) = component.update(action.clone()) {
                    self.action_list.push_back(resulting_action);
                }
            }
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let areas = get_component_areas(frame);
            for (component_name, component) in self.components.iter_mut() {
                let area = areas.get(component_name).unwrap();
                let _ = component.draw(frame, *area);
            }
        })?;
        Ok(())
    }
}
