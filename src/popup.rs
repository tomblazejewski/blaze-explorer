use std::collections::VecDeque;

use color_eyre::eyre::Result;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{
    action::Action,
    input_machine::{InputMachine, KeyProcessingResult},
    mode::Mode,
    telescope::PopUpComponent,
};

pub enum PopUp<T>
where
    T: PopUpComponent,
{
    None,
    PopUp(PopUpWindow<T>),
}

impl<T> PopUp<T>
where
    T: PopUpComponent,
{
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self {
            PopUp::None => {}
            PopUp::PopUp(popup_window) => popup_window.handle_key_event(key_event),
        }
    }

    pub fn handle_actions(&mut self) {
        match self {
            PopUp::None => {}
            PopUp::PopUp(popup_window) => popup_window.handle_actions(),
        }
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>
    where
        T: PopUpComponent + std::fmt::Display,
    {
        match self {
            PopUp::None => {}
            PopUp::PopUp(popup_window) => popup_window.component.draw(frame, area)?,
        }
        Ok(())
    }
}

struct PopUpWindow<T>
where
    T: PopUpComponent,
{
    input_machine: InputMachine,
    component: T,
    current_sequence: Vec<KeyEvent>,
    action_list: VecDeque<Action>,
}

impl<T> PopUpWindow<T>
where
    T: PopUpComponent,
{
    fn new(component: T) -> Self {
        PopUpWindow {
            input_machine: InputMachine::new(),
            component,
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
            let new_action = self.component.handle_action(action);
            if let Some(new_action) = new_action {
                self.action_list.push_back(new_action);
            }
        }
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>
    where
        T: PopUpComponent + std::fmt::Display,
    {
        self.component.draw(frame, area)?;
        Ok(())
    }
}
