// Describes the Input machine for the flash interface
// The flash backend will respond to actions obtained from this input machine
use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    action::{Action, PopupAction},
    input_machine::{InputMachine, KeyMapNode, KeyProcessingResult},
    mode::Mode,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FlashInputMachine {
    keymap_nodes: HashMap<Mode, KeyMapNode<Action>>,
    open_immediately: bool,
}

impl InputMachine for FlashInputMachine {
    fn process_keys(
        &mut self,
        mode: &Mode,
        current_sequence: &mut Vec<KeyEvent>,
        input_key: KeyEvent,
    ) -> KeyProcessingResult<Action> {
        let keymap = self.keymap_nodes.get(mode).unwrap();
        process_telescope_keys(keymap, current_sequence, input_key)
    }

    fn get_default_action(&self, mode: &Mode, last_key: KeyEvent) -> Option<Action> {
        match mode {
            Mode::Normal => match last_key.code {
                KeyCode::Char(ch) => Some(Action::PopupAct(PopupAction::PushSearchChar(ch))),
                _ => Some(Action::PopupAct(PopupAction::Quit)),
            },
            _ => None,
        }
    }
}

impl FlashInputMachine {
    pub fn new(open_immediately: bool) -> Self {
        let mut keymap_nodes = HashMap::new();
        keymap_nodes.insert(Mode::Normal, default_key_map());

        FlashInputMachine {
            keymap_nodes,
            open_immediately,
        }
    }

    pub fn merge_jump_actions(&mut self, new_map: HashMap<char, usize>) {
        let mut basic_keymap = default_key_map();
        merge_jump_actions(&mut basic_keymap, new_map, self.open_immediately);
        self.keymap_nodes.insert(Mode::Normal, basic_keymap);
    }
}
pub fn process_telescope_keys(
    keymap: &KeyMapNode<Action>,
    current_sequence: &mut Vec<KeyEvent>,
    input_key: KeyEvent,
) -> KeyProcessingResult<Action> {
    current_sequence.push(input_key);
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
        vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
        Action::PopupAct(PopupAction::Quit),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)],
        Action::PopupAct(PopupAction::DropSearchChar),
    );
    root
}

pub fn merge_jump_actions(
    root: &mut KeyMapNode<Action>,
    new_map: HashMap<char, usize>,
    should_open: bool,
) {
    for (k, v) in new_map.iter() {
        root.add_sequence(
            vec![KeyEvent::new(KeyCode::Char(*k), KeyModifiers::NONE)],
            match should_open {
                false => Action::PopupAct(PopupAction::JumpAndClose(*v)),
                true => Action::PopupAct(PopupAction::JumpAndOpen(*v)),
            },
        );
    }
}
