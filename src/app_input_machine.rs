// Describes the Input machine for the main app - the implementation of the trait and the keymaps
// used in the main app
use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    action::{Action, AppAction, CommandAction, ExplorerAction, PopupType, TextAction},
    input_machine::{InputMachine, KeyMapNode, KeyProcessingResult},
    mode::Mode,
};

pub struct AppInputMachine<T> {
    keymap_nodes: HashMap<Mode, KeyMapNode<T>>,
}

impl InputMachine for AppInputMachine<Action> {
    fn process_keys(
        &mut self,
        mode: &Mode,
        current_sequence: &mut Vec<KeyEvent>,
        input_key: KeyEvent,
    ) -> KeyProcessingResult<Action> {
        let keymap = self.keymap_nodes.get(mode).unwrap();
        process_app_keys(keymap, current_sequence, input_key)
    }

    fn get_default_action(&self, mode: &Mode, last_key: KeyEvent) -> Option<Action> {
        match mode {
            Mode::Normal => None,
            Mode::Search => match last_key.code {
                KeyCode::Char(ch) => Some(Action::TextAct(TextAction::InsertKey(ch))),
                _ => None,
            },
            Mode::Command => match last_key.code {
                KeyCode::Char(ch) => Some(Action::TextAct(TextAction::InsertKey(ch))),
                _ => None,
            },
        }
    }
}

impl AppInputMachine<Action> {
    pub fn new() -> Self {
        let mut keymap_nodes = HashMap::new();
        keymap_nodes.insert(Mode::Normal, default_key_map());
        keymap_nodes.insert(Mode::Search, search_key_map());
        keymap_nodes.insert(Mode::Command, command_key_map());

        AppInputMachine { keymap_nodes }
    }
}
pub fn process_app_keys(
    keymap: &KeyMapNode<Action>,
    current_sequence: &mut Vec<KeyEvent>,
    input_key: KeyEvent,
) -> KeyProcessingResult<Action> {
    current_sequence.push(input_key.clone());
    match keymap.get_node(current_sequence) {
        Some(node) => match &node.action {
            None => KeyProcessingResult::Incomplete, // More keys can follow
            Some(action) => {
                current_sequence.clear();
                KeyProcessingResult::Complete(action.clone()) // Final action reached
            }
        },
        None => {
            current_sequence.clear(); // Remove invalid key
            KeyProcessingResult::Invalid
        }
    }
}
pub fn default_key_map() -> KeyMapNode<Action> {
    let mut root = KeyMapNode::new();
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::NextSearchResult),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::SelectUp),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::SelectDown),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::ParentDirectory),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::SelectDirectory),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE)],
        Action::AppAct(AppAction::SwitchMode(Mode::Search)),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char(':'), KeyModifiers::SHIFT)],
        Action::AppAct(AppAction::SwitchMode(Mode::Command)),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::ClearSearchQuery),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
        ],
        Action::AppAct(AppAction::OpenPopup(PopupType::Telescope)),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE)],
        Action::AppAct(AppAction::OpenPopup(PopupType::Rename)),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE)],
        Action::AppAct(AppAction::OpenPopup(PopupType::Flash)),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
        ],
        Action::AppAct(AppAction::Delete),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE)],
        Action::CommandAct(CommandAction::Undo),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL)],
        Action::CommandAct(CommandAction::Redo),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE),
        ],
        Action::AppAct(AppAction::OpenNeovimHere),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE),
        ],
        Action::AppAct(AppAction::DisplayMessage(
            "message \n message \n message".to_string(),
        )),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CONTROL),
            KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE),
        ],
        Action::ExplorerAct(ExplorerAction::SplitVertically),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CONTROL),
            KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
        ],
        Action::ExplorerAct(ExplorerAction::SplitHorizontally),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('h'), KeyModifiers::CONTROL)],
        Action::ExplorerAct(ExplorerAction::FocusLeft),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL)],
        Action::ExplorerAct(ExplorerAction::FocusDown),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL)],
        Action::ExplorerAct(ExplorerAction::FocusUp),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL)],
        Action::ExplorerAct(ExplorerAction::FocusRight),
    );

    root
}

pub fn search_key_map() -> KeyMapNode<Action> {
    let mut root = KeyMapNode::new();
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
        Action::AppAct(AppAction::SwitchMode(Mode::Normal)),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL)],
        Action::TextAct(TextAction::EraseText),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)],
        Action::TextAct(TextAction::DropKey),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
        Action::AppAct(AppAction::ConfirmSearchQuery),
    );
    root
}
pub fn command_key_map() -> KeyMapNode<Action> {
    let mut root = KeyMapNode::new();
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
        Action::AppAct(AppAction::SwitchMode(Mode::Normal)),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL)],
        Action::TextAct(TextAction::EraseText),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)],
        Action::TextAct(TextAction::DropKey),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
        Action::AppAct(AppAction::ConfirmCommand),
    );
    root
}
