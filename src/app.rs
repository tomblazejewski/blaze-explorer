use std::io::{stdout, Stdout};
use std::path;

use color_eyre::Result;
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
    pub current_path: String,
    pub components: Vec<Box<dyn Component>>,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            current_path: String::from("./"),
            components: vec![Box::new(ExplorerTable::new()), Box::new(PathDisplay::new())],
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
        })
    }

    pub fn update_path(&mut self, path: String) {
        self.current_path = path.clone();
    }

    pub fn run(&mut self) -> Result<()> {
        self.terminal.clear()?;
        let path = "./";
        let starting_path = path::absolute(path).unwrap().to_str().unwrap().to_string();
        self.handle_actions(Action::ChangeDirectory(starting_path));
        loop {
            self.render();
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                    let _ = self.handle_actions(Action::Key(key));
                }
            }
        }
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    pub fn handle_self_actions(&mut self, action: Action) -> Result<()> {
        match action {
            Action::ChangeDirectory(path) => self.update_path(path),
            _ => {}
        }
        Ok(())
    }
    pub fn handle_actions(&mut self, action: Action) -> Result<()> {
        self.handle_self_actions(action.clone());
        for component in self.components.iter_mut() {
            component.update(action.clone());
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
