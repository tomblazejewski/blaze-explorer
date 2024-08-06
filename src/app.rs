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

use crate::{
    action::Action,
    components::{explorer_table::ExplorerTable, path_display::PathDisplay, Component},
};

pub struct App {
    pub components: Vec<Box<dyn Component>>,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub action_list: VecDeque<Action>,
    pub last_tick_key_events: Vec<KeyEvent>,
    pub multiplier: u32,
    pub keymap: HashMap<Vec<KeyEvent>, Action>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            components: vec![Box::new(ExplorerTable::new()), Box::new(PathDisplay::new())],
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            action_list: VecDeque::new(),
            last_tick_key_events: Vec::new(),
            multiplier: 0,
            keymap: HashMap::from([
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
            ]),
            should_quit: false,
        })
    }

    pub fn accept_digit(&mut self, digit_char: char) {
        if self.multiplier == 1 {
            self.multiplier = digit_char.to_digit(10).unwrap();
        } else {
            self.multiplier = self.multiplier * 10 + digit_char.to_digit(10).unwrap();
        }
    }
    pub fn reset_keys_stored(&mut self) {
        self.last_tick_key_events = Vec::new();
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
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(char_entered) => match char_entered {
                            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                                self.accept_digit(char_entered);
                            }
                            _ => self.last_tick_key_events.push(key),
                        },
                        _ => self.last_tick_key_events.push(key),
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
        let action_found = self.keymap.get(&self.last_tick_key_events).cloned();
        self.reset_keys_stored();
        match action_found {
            Some(action) => Some(action),
            _ => Some(Action::EscapeSequence),
        }
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
