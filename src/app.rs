use std::collections::{HashMap, VecDeque};
use std::io::{stdout, Stdout};
use std::path;

use color_eyre::Result;
use ratatui::crossterm::event::{KeyEvent, KeyModifiers};
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

use crate::key_combination::KeyManager;
use crate::{
    action::Action,
    components::{
        explorer_table::ExplorerTable, key_tracker::KeyTracker, path_display::PathDisplay,
        Component,
    },
    key_combination::NumberCombination,
};

pub struct App {
    pub components: Vec<Box<dyn Component>>,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub action_list: VecDeque<Action>,
    pub key_manager: KeyManager,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            components: vec![
                Box::new(ExplorerTable::new()),
                Box::new(PathDisplay::new()),
                Box::new(KeyTracker::new()),
            ],
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            action_list: VecDeque::new(),
            key_manager: KeyManager::new(),
            should_quit: false,
        })
    }

    pub fn queue_key_event(&mut self, action: Action) {
        self.action_list.push_back(action);
    }
    pub fn run(&mut self) -> Result<()> {
        self.terminal.clear()?;
        let path = "./";
        let starting_path = path::absolute(path).unwrap().to_str().unwrap().to_string();
        self.action_list
            .push_back(Action::ChangeDirectory(starting_path));
        self.handle_actions();
        loop {
            self.render();
            if let event::Event::Key(key) = event::read()? {
                //need to push the key to get the
                //keytracker to work?
                info!("Pushed {:?}", key);
                if key.kind == KeyEventKind::Press {
                    self.action_list.push_back(Action::Key(key));
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

    pub fn handle_self_actions(&mut self, action: &Action) -> Option<Action> {
        match action {
            Action::EscapeSequence => {
                self.key_manager.clear_keys_stored();
            }
            Action::ClearAndKey(key_event) => self.key_manager.clear_and_enter(*key_event),
            Action::Quit => self.should_quit = true,
            Action::Linger(n) => {
                if *n > 0 {
                    return Some(Action::Linger(n - 1));
                } else {
                    self.key_manager.clear_keys_stored();
                }
            }
            _ => {}
        }
        None
    }
    pub fn handle_actions(&mut self) -> Result<()> {
        while let Some(action) = self.action_list.pop_front() {
            if let Some(action_received) = self.handle_self_actions(&action) {
                self.action_list.push_back(action_received);
            }
            for component in self.components.iter_mut() {
                if let Ok(Some(resulting_action)) = component.update(action.clone()) {
                    self.action_list.push_back(resulting_action);
                }
            }
        }
        Ok(())
    }
    pub fn render(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            for component in self.components.iter_mut() {
                let _ = component.draw(frame, frame.size());
            }
        })?;
        Ok(())
    }
}
