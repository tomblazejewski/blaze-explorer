use std::collections::HashMap;
use std::fmt;

use color_eyre::eyre::Result;
use ratatui::layout::Constraint;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{crossterm::event::KeyEvent, layout::Rect, widgets::Clear, Frame};
use tracing::info;

use crate::action::PopupAction;
use crate::command::{Command, RenameActive, ResetStyling};
use crate::components::explorer_manager::ExplorerManager;
use crate::components::explorer_table::GlobalStyling;
use crate::flash_input_machine::FlashInputMachine;
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

const JUMP_KEYS: [char; 25] = [
    'q', 'w', 'e', 'r', 't', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'z',
    'x', 'c', 'v', 'b', 'n', 'm',
];

macro_rules! match_enum_call {
    ($self:ident, $func:ident $(, $arg:expr)*) => {
        match $self {
            PopUp::None => {},
            PopUp::TelescopePopUp(inner) => inner.$func($($arg),*),
            PopUp::InputPopUp(inner) => inner.$func($($arg),*),
            PopUp::FlashPopUp(inner) => inner.$func($($arg),*),
        }
    };

}
macro_rules! match_enum_return {
    ($self:ident, $func:ident $(, $arg:expr)*) => {
        match $self {
            PopUp::None => None,
            PopUp::TelescopePopUp(inner) => inner.$func($($arg),*),
            PopUp::InputPopUp(inner) => inner.$func($($arg),*),
            PopUp::FlashPopUp(inner) => inner.$func($($arg),*),
        }
    };
}

pub fn pop_char(key_list: &mut Vec<char>, ch: Option<char>) -> char {
    match ch {
        Some(ch) => {
            key_list.retain(|k| *k != ch);
            ch
        }
        None => key_list.pop().unwrap(),
    }
}

pub enum PopUp {
    None,
    TelescopePopUp(TelescopeWindow),
    InputPopUp(ActionInput<RenameActive>),
    FlashPopUp(FlashJump),
}

impl fmt::Debug for PopUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PopUp::None => write!(f, "None"),
            PopUp::TelescopePopUp(_) => write!(f, "TelescopePopUp"),
            PopUp::InputPopUp(_) => write!(f, "InputPopUp"),
            PopUp::FlashPopUp(_) => write!(f, "FlashPopUp"),
        }
    }
}
impl PopUp {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action> {
        match_enum_return!(self, handle_key_event, key_event)
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.draw(frame, area)?,
            PopUp::InputPopUp(action_input) => action_input.draw(frame, area)?,
            PopUp::FlashPopUp(flash_popup) => flash_popup.draw(frame, area)?,
        }
        Ok(())
    }

    pub fn confirm_result(&mut self) -> Option<Action> {
        match_enum_return!(self, confirm_result)
    }

    pub fn next_result(&mut self) {
        match_enum_call!(self, next_result)
    }

    pub fn previous_result(&mut self) {
        match_enum_call!(self, previous_result)
    }

    pub fn update_search_query(&mut self, query: String) {
        match self {
            PopUp::None => {}
            PopUp::TelescopePopUp(popup_window) => popup_window.update_search_query(query),
            PopUp::InputPopUp(action_input) => action_input.update_search_query(query),
            PopUp::FlashPopUp(flash_popup) => flash_popup.update_search_query(query),
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
            PopUp::InputPopUp(_action_input) => None,
            PopUp::FlashPopUp(flash_popup) => None,
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
            PopUp::FlashPopUp(flash_popup) => {
                flash_popup.push_search_char(ch);
                self.search_query_action()
            }
        }
    }

    pub fn drop_search_char(&mut self) -> Option<Action> {
        info!("Currently {:?}", self);
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
            PopUp::FlashPopUp(flash_popup) => {
                info!("dropping search char");
                flash_popup.drop_search_char();
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
            PopUp::InputPopUp(action_input) => {
                action_input.erase_text();
                None
            }
            PopUp::FlashPopUp(flash_popup) => {
                flash_popup.erase_text();
                self.search_query_action()
            }
        }
    }

    pub fn quit(&mut self) {
        match_enum_call!(self, quit)
    }

    pub fn destruct(&self) -> Option<Box<dyn Command>> {
        match_enum_return!(self, destruct)
    }

    pub fn should_quit(&self) -> bool {
        match self {
            PopUp::None => false,
            PopUp::TelescopePopUp(popup_window) => popup_window.should_quit,
            PopUp::InputPopUp(action_input) => action_input.should_quit,
            PopUp::FlashPopUp(flash_popup) => flash_popup.should_quit,
        }
    }
}

pub trait PopupEngine {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action>;

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;

    fn confirm_result(&mut self) -> Option<Action> {
        None
    }

    fn next_result(&mut self) {}

    fn previous_result(&mut self) {}

    fn update_search_query(&mut self, _query: String) {}

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
    pub fn new(mut ctx: AppContext) -> Self {
        let suffix = ctx
            .explorer_manager
            .select_directory()
            .unwrap()
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let dot_suffix = format!(".{}", suffix);
        ActionInput {
            resulting_action: RenameActive::default(ctx),
            input_machine: TelescopeInputMachine::new(),
            current_sequence: Vec::new(),
            should_quit: false,
            query: TelescopeQuery::new(String::new(), dot_suffix),
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

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let query_area = center_rect(
            frame.size(),
            Constraint::Percentage(50),
            Constraint::Length(3),
        );
        let title = format!(
            "Rename {}",
            self.resulting_action.first_path.to_str().unwrap()
        );
        let query_block = Block::default().borders(Borders::ALL).title(title);
        let new_name = self.query.contents.clone();
        let extension = self.resulting_action.first_path.extension().unwrap();
        let rename_field_output = format!("{}.{}", new_name, extension.to_str().unwrap());
        let query_paragraph = Paragraph::new(rename_field_output);
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

//Struct representing the plugin used to jump to a chosen filename
//Aims to send and request data from the explorer table in order to send an action requesting to
//jump to a specific file
pub struct FlashJump {
    query: String,
    input_machine: FlashInputMachine,
    pub should_quit: bool,
    current_sequence: Vec<KeyEvent>,
    jump_map: HashMap<char, usize>,
    open_immediately: bool,
}
impl FlashJump {
    pub fn new(mut _ctx: AppContext, open: bool) -> Self {
        FlashJump {
            query: String::new(),
            input_machine: FlashInputMachine::new(open),
            should_quit: false,
            current_sequence: Vec::new(),
            jump_map: HashMap::new(),
            open_immediately: open,
        }
    }

    pub fn update_search_query(&mut self, query: String) {
        self.query = query;
    }

    pub fn update_interface(&mut self, explorer_manager: &mut ExplorerManager) {
        if !&self.query.is_empty() {
            let resulting_file_data = explorer_manager.find_elements(&self.query);
            let mut new_map = HashMap::new();
            let mut key_list = JUMP_KEYS.to_vec();
            let current_map_reverted = self
                .jump_map
                .iter()
                .map(|(k, v)| (*v, *k))
                .collect::<HashMap<usize, char>>();
            if resulting_file_data.len() > JUMP_KEYS.len() {
                self.jump_map = HashMap::new();
            } else {
                //if an id already exists in the map, it should have the same char
                for file_data in resulting_file_data {
                    let id = file_data.id;
                    if let Some(ch) = current_map_reverted.get(&id) {
                        let ch = pop_char(&mut key_list, Some(*ch));
                        new_map.insert(ch, id);
                    } else {
                        let ch = pop_char(&mut key_list, None);
                        new_map.insert(ch, id);
                    }
                }
                self.jump_map = new_map;
            }
        } else {
            if !self.jump_map.is_empty() {
                self.quit();
                return;
            }
            self.jump_map = HashMap::new();
        };
        self.input_machine = FlashInputMachine::new(self.open_immediately);
        self.input_machine.merge_jump_actions(self.jump_map.clone());
        explorer_manager.set_styling(GlobalStyling::HighlightJump(
            self.query.clone(),
            self.jump_map.clone(),
        ));
    }
}
impl PopupEngine for FlashJump {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Action> {
        let keymap_result =
            self.input_machine
                .process_keys(&Mode::Normal, &mut self.current_sequence, key_event);
        match keymap_result {
            KeyProcessingResult::Complete(action) => {
                info!("Action: {:?}", action);
                return Some(action);
            }
            KeyProcessingResult::Invalid => {
                info!(
                    "Invalid: {:?}",
                    self.input_machine
                        .get_default_action(&Mode::Normal, key_event)
                );
                return self
                    .input_machine
                    .get_default_action(&Mode::Normal, key_event);
            }
            _ => {}
        }
        None
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        Ok(())
    }

    fn push_search_char(&mut self, ch: char) {
        self.query.push(ch)
    }

    fn drop_search_char(&mut self) {
        info!("Query: {}", self.query);
        self.query.pop();
        info!("Query: {}", self.query);
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn get_search_query(&self) -> String {
        self.query.clone()
    }

    fn destruct(&self) -> Option<Box<dyn Command>> {
        Some(Box::new(ResetStyling::new()))
    }

    fn erase_text(&mut self) {}
}
