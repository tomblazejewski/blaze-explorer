// Describes the Input machine for the telescope interface
// The telescope backend will respond to actions obtained from this input machine
use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    action::{Action, AppAction, ExplorerAction, TelescopeAction, TextAction},
    input_machine::{InputMachine, KeyMapNode, KeyProcessingResult},
    mode::Mode,
};

pub struct TelescopeInputMachine {
    keymap_nodes: HashMap<Mode, KeyMapNode<TelescopeAction>>,
}

impl InputMachine<TelescopeAction> for TelescopeInputMachine {
    fn process_keys(
        &mut self,
        mode: &Mode,
        current_sequence: &mut Vec<KeyEvent>,
        input_key: KeyEvent,
    ) -> KeyProcessingResult<TelescopeAction> {
        let keymap = self.keymap_nodes.get(mode).unwrap();
        process_telescope_keys(keymap, current_sequence, input_key)
    }

    fn get_default_action(&self, mode: &Mode, last_key: KeyEvent) -> Option<TelescopeAction> {
        match mode {
            Mode::Normal => match last_key.code {
                KeyCode::Char(ch) => Some(TelescopeAction::PushSearchChar(ch)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl TelescopeInputMachine {
    pub fn new() -> Self {
        let mut keymap_nodes = HashMap::new();
        keymap_nodes.insert(Mode::Normal, default_key_map());

        TelescopeInputMachine { keymap_nodes }
    }
}
pub fn process_telescope_keys(
    keymap: &KeyMapNode<TelescopeAction>,
    current_sequence: &mut Vec<KeyEvent>,
    input_key: KeyEvent,
) -> KeyProcessingResult<TelescopeAction> {
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
pub fn default_key_map() -> KeyMapNode<TelescopeAction> {
    let mut root = KeyMapNode::new();
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
        TelescopeAction::Quit,
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('n'), KeyModifiers::CONTROL)],
        TelescopeAction::NextResult,
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL)],
        TelescopeAction::PreviousResult,
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)],
        TelescopeAction::DropSearchChar,
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
        TelescopeAction::ConfirmResult,
    );
    root
}
