use std::collections::VecDeque;

use color_eyre::eyre::Result;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, widgets::Clear, Frame};
use tracing::info;

use crate::action::TelescopeAction;
use crate::line_entry::LineEntry;
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

    pub fn confirm_result(&mut self) -> Option<Action> {
        match self {
            PopUp::None => None,
            PopUp::TelescopePopUp(popup_window) => popup_window.confirm_result(),
        }
    }

    pub fn next_result(&mut self) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.next_result(),
        }
    }

    pub fn previous_result(&mut self) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.previous_result(),
        }
    }

    pub fn update_search_query(&mut self, query: String) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.update_search_query(query),
        }
    }

    pub fn search_query_action(&self) -> Option<Action> {
        match self {
            PopUp::None => None,
            PopUp::TelescopePopUp(popup_window) => {
                let search_query = popup_window.get_search_query();
                Some(Action::TelescopeAct(TelescopeAction::UpdateSearchQuery(
                    search_query,
                )))
            }
        }
    }

    pub fn push_search_char(&mut self, ch: char) -> Option<Action> {
        match self {
            PopUp::None => None,
            PopUp::TelescopePopUp(popup_window) => {
                popup_window.push_search_char(ch);
                self.search_query_action()
            }
        }
    }

    pub fn drop_search_char(&mut self) -> Option<Action> {
        match self {
            PopUp::None => None,
            PopUp::TelescopePopUp(popup_window) => {
                popup_window.drop_search_char();
                self.search_query_action()
            }
        }
    }

    pub fn erase_text(&mut self) -> Option<Action> {
        match self {
            PopUp::None => None,
            PopUp::TelescopePopUp(popup_window) => {
                popup_window.erase_text();
                self.search_query_action()
            }
        }
    }

    pub fn quit(&mut self) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.quit(),
        }
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

    pub fn confirm_result(&mut self) -> Option<Action> {
        self.telescope_backend.confirm_result()
    }

    pub fn next_result(&mut self) {
        self.telescope_backend.next_result();
    }

    pub fn previous_result(&mut self) {
        self.telescope_backend.previous_result();
    }

    pub fn update_search_query(&mut self, query: String) {
        self.telescope_backend.update_search_query(query);
    }

    pub fn push_search_char(&mut self, ch: char) {
        self.telescope_backend.query.append_char(ch)
    }

    fn drop_search_char(&mut self) {
        self.telescope_backend.query.drop_char()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn erase_text(&mut self) {
        self.telescope_backend.query.clear_contents();
    }

    pub fn get_search_query(&self) -> String {
        self.telescope_backend.query.get_contents()
    }
}
