use std::collections::VecDeque;
use std::io::{stdout, Stdout};
use std::path;

use color_eyre::Result;
use ratatui::crossterm::event::KeyEvent;
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
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            components: vec![Box::new(ExplorerTable::new()), Box::new(PathDisplay::new())],
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            action_list: VecDeque::new(),
            last_tick_key_events: Vec::new(),
            multiplier: 0,
        })
    }

    pub fn accept_digit(&mut self, digit_char: char) {
        if self.multiplier == 1 {
            self.multiplier = digit_char.to_digit(10).unwrap();
        } else {
            self.multiplier = self.multiplier * 10 + digit_char.to_digit(10).unwrap();
        }
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
        loop {
            self.handle_actions();
            self.render();
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char(char_entered) = key.code {
                        if char_entered == 'q' {
                            break;
                        } else if char_entered.is_ascii_digit() && char_entered != '0' {
                            self.accept_digit(char_entered);
                        }
                    }
                    if key.code == KeyCode::Esc {
                        self.reset_multiplier();
                    }
                    self.queue_key_event(Action::Key(key));
                }
            }
        }
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    pub fn accept_keys(&self) {}

    pub fn handle_self_actions(&mut self, action: Action) -> Result<()> {
        match action {
            _ => {}
        }
        Ok(())
    }
    pub fn handle_actions(&mut self) -> Result<()> {
        while let Some(action) = self.action_list.pop_front() {
            self.handle_self_actions(action.clone());
            for component in self.components.iter_mut() {
                if let Ok(Some(new_action)) = component.update(action.clone()) {
                    self.action_list.push_back(new_action);
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
