use std::collections::VecDeque;

use color_eyre::eyre::Result;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{
    action::Action,
    input_machine::{InputMachine, KeyProcessingResult},
    mode::Mode,
    telescope::{AppContext, PopUpComponent, Telescope},
};

pub enum PopUp {
    None,
    TelescopePopUp(PopUpWindow),
}

impl PopUp {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.handle_key_event(key_event),
        }
    }

    pub fn handle_actions(&mut self) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.handle_actions(),
        }
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.draw(frame, area)?,
        }
        Ok(())
    }
}

pub struct PopUpWindow {
    input_machine: InputMachine,
    telescope_backend: Telescope,
    current_sequence: Vec<KeyEvent>,
    action_list: VecDeque<Action>,
}

impl PopUpWindow {
    pub fn new(ctx: AppContext) -> Self {
        PopUpWindow {
            input_machine: InputMachine::new(),
            telescope_backend: Telescope::new(ctx),
            current_sequence: Vec::new(),
            action_list: VecDeque::new(),
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        let keymap_result =
            self.input_machine
                .process_keys(&Mode::Normal, &mut self.current_sequence, key_event);
        match keymap_result {
            KeyProcessingResult::Complete(action) => {
                self.action_list.push_back(action);
            }
            KeyProcessingResult::Invalid => {
                if let Some(action) = self
                    .input_machine
                    .get_default_action(&Mode::Normal, key_event)
                {
                    self.action_list.push_back(action);
                }
            }
            _ => {}
        }
    }

    pub fn handle_actions(&mut self) {
        while let Some(action) = self.action_list.pop_front() {
            let new_action = self.telescope_backend.handle_action(action);
            if let Some(new_action) = new_action {
                self.action_list.push_back(new_action);
            }
        }
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.telescope_backend.draw(frame, area)?;
        Ok(())
    }
}
