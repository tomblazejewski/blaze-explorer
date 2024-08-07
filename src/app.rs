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

use crate::{
    action::Action,
    components::{
        explorer_table::ExplorerTable, key_tracker::KeyTracker, path_display::PathDisplay,
        Component,
    },
};

pub struct App {
    pub components: Vec<Box<dyn Component>>,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub action_list: VecDeque<Action>,
    pub last_command_keys: Vec<KeyEvent>,
    pub last_multiplier_keys: Vec<KeyEvent>,
    pub last_sequence_keys: Vec<KeyEvent>,
    pub multiplier: u32,
    pub keymap: HashMap<Vec<KeyEvent>, Action>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let numerical_keymaps = ['1', '2', '3', '4', '5', '6', '7', '8', '9']
            .iter()
            .map(|char_| {
                vec![KeyEvent::new(
                    KeyCode::Char(char_.to_owned()),
                    KeyModifiers::NONE,
                )]
            })
            .zip(vec![Action::Noop; 9])
            .collect::<HashMap<Vec<KeyEvent>, Action>>();
        let mut keyboard_keymaps = HashMap::from([
            (
                vec![KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)],
                Action::Quit,
            ),
            (
                vec![KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)],
                Action::SelectUp,
            ),
            (
                vec![KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE)],
                Action::SelectDown,
            ),
            (
                vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
                Action::EscapeSequence,
            ),
            (
                vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
                Action::SelectDirectory,
            ),
            (
                vec![KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)],
                Action::ParentDirectory,
            ),
        ]);
        keyboard_keymaps.extend(numerical_keymaps);

        Ok(Self {
            components: vec![
                Box::new(ExplorerTable::new()),
                Box::new(PathDisplay::new()),
                Box::new(KeyTracker::new()),
            ],
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            action_list: VecDeque::new(),
            last_command_keys: Vec::new(),
            last_sequence_keys: Vec::new(),
            last_multiplier_keys: Vec::new(),
            multiplier: 0,
            keymap: keyboard_keymaps,
            should_quit: false,
        })
    }

    pub fn reset_keys_stored(&mut self) {
        self.last_command_keys = Vec::new();
    }
    pub fn reset_command_keys(&mut self) {
        self.last_command_keys = Vec::new();
    }

    pub fn add_digit_key(&mut self, key: KeyEvent) {
        if self.last_sequence_keys.is_empty() {
            self.last_multiplier_keys.push(key);
            self.last_sequence_keys.push(key);
        } else {
            let last_key_entered = self.last_sequence_keys.pop();
            if let Some(last_key) = last_key_entered {
                match last_key{
                    KeyEvent::Ch
                }
            }
        }
    }

    pub fn append_multiplier_key(&mut self, key: KeyEvent) {
        self.last_multiplier_keys.push(key);
        self.last_sequence_keys.push(key);
    }

    pub fn escape_sequence(&mut self) {
        self.last_multiplier_keys = Vec::new();
        self.last_sequence_keys = Vec::new();
        self.last_command_keys = Vec::new();
    }

    pub fn reset_multiplier(&mut self) {
        self.multiplier = 1;
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
                self.action_list.push_back(Action::Key(key)); //need to push the key to get the
                                                              //keytracker to work?
                info!("Pushed {:?}", key);
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(char_entered) => match char_entered {
                            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                                self.reset_keys_stored(); //if a digit, automatically cancel any
                                                          //possible multi keymaps
                                self.append_multiplier_key(key);
                            }
                            _ => self.last_command_keys.push(key),
                        },
                        _ => self.last_command_keys.push(key),
                    }
                };
                if let Some(action) = self.handle_key_event() {
                    self.action_list.push_back(action);
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

    pub fn handle_key_event(&mut self) -> Option<Action> {
        let action_found = self.keymap.get(&self.last_command_keys).cloned();
        // need to store the numbers somehow as well!! given I am taking the data from last ticke
        // key events - can I just not store them and put them somewhere else as soon as I get non
        // -digit?
        self.reset_keys_stored();
        let return_value = match action_found {
            Some(action) => Some(action),
            _ => Some(Action::EscapeSequence),
        };
        info!(
            "Matching {:?} to {:?}",
            &self.last_command_keys, return_value
        );
        return_value
    }

    pub fn handle_self_actions(&mut self, action: Action) -> Result<()> {
        match action {
            Action::EscapeSequence => {
                self.reset_keys_stored();
            }
            Action::Quit => self.should_quit = true,
            _ => {}
        }
        Ok(())
    }
    pub fn handle_actions(&mut self) -> Result<()> {
        while let Some(action) = self.action_list.pop_front() {
            self.handle_self_actions(action.clone());
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
