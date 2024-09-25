use color_eyre::eyre::Result;
use ratatui::layout::Constraint;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{crossterm::event::KeyEvent, layout::Rect, widgets::Clear, Frame};
use tracing::info;

use crate::action::PopupAction;
use crate::command::{Command, RenameActive};
use crate::line_entry::LineEntry;
use crate::simple_input_machine::SimpleInputMachine;
use crate::telescope_input_machine::TelescopeInputMachine;
use crate::telescope_query::TelescopeQuery;
use crate::tools::center_rect;
use crate::{
    action::Action,
    input_machine::{InputMachine, KeyProcessingResult},
    mode::Mode,
    telescope::{AppContext, PopUpComponent, Telescope},
};

pub enum PopUp {
    None,
    TelescopePopUp(TelescopeWindow),
    InputPopUp(ActionInput<RenameActive>),
}

impl PopUp {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action> {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => return popup_window.handle_key_event(key_event),
            PopUp::InputPopUp(action_input) => return action_input.handle_key_event(key_event),
        }
        None
    }

    pub fn handle_action(&mut self, action: Action) -> Option<Action> {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => return popup_window.handle_action(action),
            PopUp::InputPopUp(action_input) => return action_input.handle_action(action),
        }
        None
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.draw(frame, area)?,
            PopUp::InputPopUp(action_input) => action_input.draw(frame, area)?,
        }
        Ok(())
    }

    pub fn confirm_result(&mut self) -> Option<Action> {
        match self {
            PopUp::None => None,
            PopUp::TelescopePopUp(popup_window) => popup_window.confirm_result(),
            PopUp::InputPopUp(action_input) => action_input.confirm_result(),
        }
    }

    pub fn next_result(&mut self) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.next_result(),
            PopUp::InputPopUp(action_input) => action_input.next_result(),
        }
    }

    pub fn previous_result(&mut self) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.previous_result(),
            PopUp::InputPopUp(action_input) => action_input.previous_result(),
        }
    }

    pub fn update_search_query(&mut self, query: String) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.update_search_query(query),
            PopUp::InputPopUp(action_input) => action_input.update_search_query(query),
        }
    }

    pub fn search_query_action(&self) -> Option<Action> {
        match self {
            PopUp::None => None,
            PopUp::TelescopePopUp(popup_window) => {
                let search_query = popup_window.get_search_query();
                Some(Action::PopupAct(PopupAction::UpdateSearchQuery(
                    search_query,
                )))
            }
            PopUp::InputPopUp(action_input) => None,
        }
    }

    pub fn push_search_char(&mut self, ch: char) -> Option<Action> {
        match self {
            PopUp::None => None,
            PopUp::TelescopePopUp(popup_window) => {
                popup_window.push_search_char(ch);
                self.search_query_action()
            }
            PopUp::InputPopUp(action_input) => {
                action_input.push_search_char(ch);
                None
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
            PopUp::InputPopUp(action_input) => {
                action_input.drop_search_char();
                None
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
            PopUp::InputPopUp(action_input) => {
                action_input.erase_text();
                None
            }
        }
    }

    pub fn quit(&mut self) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.quit(),
            PopUp::InputPopUp(action_input) => action_input.quit(),
        }
    }

    pub fn destruct(&self) -> Option<Box<dyn Command>> {
        match self {
            PopUp::None => None,
            PopUp::TelescopePopUp(popup_window) => popup_window.destruct(),
            PopUp::InputPopUp(action_input) => action_input.destruct(),
        }
    }

    pub fn should_quit(&self) -> bool {
        match self {
            PopUp::None => false,
            PopUp::TelescopePopUp(popup_window) => popup_window.should_quit,
            PopUp::InputPopUp(action_input) => action_input.should_quit,
        }
    }
}

pub trait PopupEngine {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action>;

    fn handle_action(&mut self, action: Action) -> Option<Action>;

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;

    fn confirm_result(&mut self) -> Option<Action> {
        None
    }

    fn next_result(&mut self) {}

    fn previous_result(&mut self) {}

    fn update_search_query(&mut self, query: String) {}

    fn push_search_char(&mut self, ch: char);

    fn drop_search_char(&mut self);

    fn quit(&mut self);

    fn erase_text(&mut self);

    fn get_search_query(&self) -> String;

    fn destruct(&self) -> Option<Box<dyn Command>> {
        None
    }
}
pub struct TelescopeWindow {
    input_machine: SimpleInputMachine,
    telescope_backend: Telescope,
    current_sequence: Vec<KeyEvent>,
    pub should_quit: bool,
}

impl TelescopeWindow {
    pub fn new(ctx: AppContext) -> Self {
        TelescopeWindow {
            input_machine: SimpleInputMachine::new(),
            telescope_backend: Telescope::new(ctx),
            current_sequence: Vec::new(),
            should_quit: false,
        }
    }
}
impl PopupEngine for TelescopeWindow {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action> {
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

    fn handle_action(&mut self, action: Action) -> Option<Action> {
        if action == Action::PopupAct(PopupAction::Quit) {
            self.should_quit = true;
            return None;
        }
        let new_action = self.telescope_backend.handle_action(action);
        new_action
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        frame.render_widget(Clear, area);
        self.telescope_backend.draw(frame, area)?;
        Ok(())
    }

    fn confirm_result(&mut self) -> Option<Action> {
        self.telescope_backend.confirm_result()
    }

    fn next_result(&mut self) {
        self.telescope_backend.next_result();
    }

    fn previous_result(&mut self) {
        self.telescope_backend.previous_result();
    }

    fn update_search_query(&mut self, query: String) {
        self.telescope_backend.update_search_query(query);
    }

    fn push_search_char(&mut self, ch: char) {
        self.telescope_backend.query.append_char(ch)
    }

    fn drop_search_char(&mut self) {
        self.telescope_backend.query.drop_char()
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn erase_text(&mut self) {
        self.telescope_backend.query.clear_contents();
    }

    fn get_search_query(&self) -> String {
        self.telescope_backend.query.get_contents()
    }
}

pub struct ActionInput<T> {
    resulting_action: T,
    input_machine: TelescopeInputMachine,
    current_sequence: Vec<KeyEvent>,
    pub should_quit: bool,
    query: TelescopeQuery,
}

impl ActionInput<RenameActive> {
    pub fn new(ctx: AppContext) -> Self {
        ActionInput {
            resulting_action: RenameActive::default(ctx),
            input_machine: TelescopeInputMachine::new(),
            current_sequence: Vec::new(),
            should_quit: false,
            query: TelescopeQuery::new(),
        }
    }

    pub fn update_action_details(&mut self, new_name: String) {
        self.resulting_action.update_command_context(new_name);
    }
}
impl PopupEngine for ActionInput<RenameActive> {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action> {
        let keymap_result =
            self.input_machine
                .process_keys(&Mode::Normal, &mut self.current_sequence, key_event);
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

    fn handle_action(&mut self, action: Action) -> Option<Action> {
        if let Action::PopupAct(action) = action {
            match action {
                PopupAction::Quit => self.should_quit = true,
                PopupAction::ConfirmResult => return self.confirm_result(),
                PopupAction::NextResult => {
                    self.next_result();
                }
                PopupAction::PreviousResult => self.previous_result(),
                PopupAction::UpdateSearchQuery(query) => {}
                action => return self.query.handle_text_action(action),
            }
        }
        None
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let query_area = center_rect(
            frame.size(),
            Constraint::Percentage(50),
            Constraint::Percentage(10),
        );
        let title = format!(
            "Rename {}",
            self.resulting_action.first_path.to_str().unwrap()
        );
        let query_block = Block::default().borders(Borders::ALL).title(title);
        let query_paragraph = Paragraph::new(self.query.contents.clone());
        let query_paragraph = query_paragraph.block(query_block);

        frame.render_widget(Clear, area);
        frame.render_widget(query_paragraph, query_area);

        Ok(())
    }

    fn confirm_result(&mut self) -> Option<Action> {
        self.update_action_details(self.query.get_contents());
        self.should_quit = true;
        None
    }

    fn push_search_char(&mut self, ch: char) {
        self.query.append_char(ch)
    }

    fn drop_search_char(&mut self) {
        self.query.drop_char()
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn erase_text(&mut self) {
        self.query.clear_contents();
    }

    fn get_search_query(&self) -> String {
        self.query.get_contents()
    }

    fn destruct(&self) -> Option<Box<dyn Command>> {
        if self.resulting_action.second_path.is_some() {
            return Some(Box::new(self.resulting_action.clone()));
        }
        None
    }
}
