use std::collections::VecDeque;

use color_eyre::eyre::Result;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, widgets::Clear, Frame};
use tracing::info;

use crate::action::TelescopeAction;
use crate::telescope_input_machine::TelescopeInputMachine;
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
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action> {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => return popup_window.handle_key_event(key_event),
        }
        None
    }

    pub fn handle_action(&mut self, action: Action) -> Option<Action> {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => return popup_window.handle_action(action),
        }
        None
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
    input_machine: TelescopeInputMachine,
    telescope_backend: Telescope,
    current_sequence: Vec<KeyEvent>,
    pub should_quit: bool,
}

impl PopUpWindow {
    pub fn new(ctx: AppContext) -> Self {
        PopUpWindow {
            input_machine: TelescopeInputMachine::new(),
            telescope_backend: Telescope::new(ctx),
            current_sequence: Vec::new(),
            should_quit: false,
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action> {
        let keymap_result =
            self.input_machine
                .process_keys(&Mode::Normal, &mut self.current_sequence, key_event);
        info!("Telescope Keymap result: {:?}", keymap_result);
        match keymap_result {
            KeyProcessingResult::Complete(action) => {
                return Some(action);
            }
            KeyProcessingResult::Invalid => {
                return self
                    .input_machine
                    .get_default_action(&Mode::Normal, key_event)
            }
            _ => {}
        }
        None
    }

    pub fn handle_action(&mut self, action: Action) -> Option<Action> {
        if action == Action::TelescopeAct(TelescopeAction::Quit) {
            self.should_quit = true;
            return None;
        }
        let new_action = self.telescope_backend.handle_action(action);
        new_action
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        frame.render_widget(Clear, area);
        self.telescope_backend.draw(frame, area)?;
        Ok(())
    }
}
