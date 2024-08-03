use std::{
    fs,
    io::{stdout, Stdout},
};

use color_eyre::Result;
use ratatui::{
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::CrosstermBackend,
    widgets::TableState,
    Terminal,
};

use crate::{
    action::Action,
    components::{explorer_table::ExplorerTable, Component},
};

pub struct App {
    pub current_path: String,
    pub elements_list: Vec<String>,
    pub selected_elements_list: Vec<String>,
    pub table_state: TableState,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            current_path: String::from("./"),
            elements_list: Vec::new(),
            selected_elements_list: Vec::new(),
            table_state: TableState::default().with_selected(0),
            components: vec![Box::new(ExplorerTable::new())],
            should_quit: false,
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
        })
    }

    pub fn update_path(&mut self, path: String) {
        self.current_path = path.clone();
        self.components[0].update(Action::ChangeDirectory(path));
    }

    pub fn run(&mut self) -> Result<()> {
        self.terminal.clear()?;
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

    pub fn handle_actions(&mut self, action: Action) -> Result<()> {
        if let Action::Key(key) = action {
            for component in self.components.iter_mut() {
                let _ = component.handle_key_events(key);
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
