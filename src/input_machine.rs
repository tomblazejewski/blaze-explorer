use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tracing::info;

use crate::{
    action::{Action, AppAction, ExplorerAction, TextAction},
    mode::Mode,
};

pub struct InputMachine {
    keymap_nodes: HashMap<Mode, KeyMapNode>,
}

impl InputMachine {
    pub fn new() -> Self {
        let mut keymap_nodes = HashMap::new();
        keymap_nodes.insert(Mode::Normal, default_key_map());
        keymap_nodes.insert(Mode::Search, search_key_map());

        InputMachine { keymap_nodes }
    }
    pub fn process_keys(
        &mut self,
        mode: &Mode,
        current_sequence: &mut Vec<KeyEvent>,
        input_key: KeyEvent,
    ) -> KeyProcessingResult {
        let keymap = self.keymap_nodes.get(mode).unwrap();
        process_keys(keymap, current_sequence, input_key)
    }

    pub fn get_default_action(&self, mode: &Mode, last_key: KeyEvent) -> Option<Action> {
        match mode {
            Mode::Normal => None,
            Mode::Search => match last_key.code {
                KeyCode::Char(ch) => Some(Action::TextAct(TextAction::InsertKey(ch))),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct KeyMapNode {
    action: Option<Action>,
    children: HashMap<KeyEvent, KeyMapNode>,
}

impl KeyMapNode {
    fn new() -> Self {
        KeyMapNode {
            action: None,
            children: HashMap::new(),
        }
    }

    fn add_sequence(&mut self, sequence: Vec<KeyEvent>, action: Action) {
        let mut current_node = self;
        for key in sequence {
            current_node = current_node
                .children
                .entry(key)
                .or_insert_with(KeyMapNode::new);
        }
        current_node.action = Some(action);
    }

    fn get_node(&self, sequence: &[KeyEvent]) -> Option<&KeyMapNode> {
        let mut current_node = self;
        for key in sequence {
            match current_node.children.get(key) {
                Some(node) => current_node = node,
                None => return None,
            }
        }
        Some(current_node)
    }
}
#[derive(Debug, PartialEq)]
pub enum KeyProcessingResult {
    Complete(Action), // Sequence is complete and valid
    Incomplete,       // Sequence is valid but not yet complete
    Invalid,          // Sequence is invalid
}

pub fn process_keys(
    keymap: &KeyMapNode,
    current_sequence: &mut Vec<KeyEvent>,
    input_key: KeyEvent,
) -> KeyProcessingResult {
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

pub fn default_key_map() -> KeyMapNode {
    let mut root = KeyMapNode::new();
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)],
        Action::AppAct(AppAction::Quit),
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
        vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::ClearSearchQuery),
    );
    root
}

pub fn search_key_map() -> KeyMapNode {
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
        Action::TextAct(TextAction::RemoveKey),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
        Action::TextAct(TextAction::ConfirmSearchQuery),
    );
    root
}
#[cfg(test)]
mod tests {
    use crate::action::TextAction;

    use super::*;

    #[test]
    fn test_keymaps_work() {
        let mut root = KeyMapNode::new();
        root.add_sequence(
            vec![KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)],
            Action::AppAct(AppAction::Quit),
        );
        root.add_sequence(
            vec![
                KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            ],
            Action::ExplorerAct(ExplorerAction::SelectDown),
        );
        root.add_sequence(
            vec![
                KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE),
            ],
            Action::ExplorerAct(ExplorerAction::SelectDown),
        );
        root.add_sequence(
            vec![
                KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            ],
            Action::AppAct(AppAction::CancelKeybind),
        );
        root.add_sequence(
            vec![
                KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
                KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            ],
            Action::ExplorerAct(ExplorerAction::SelectDirectory),
        );
        let mut current_sequence: Vec<KeyEvent> = Vec::new();
        let j_event = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);
        let q_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let k_event = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE);
        let b_event = KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE);

        let result = process_keys(&root, &mut current_sequence, j_event);
        assert_eq!(result, KeyProcessingResult::Incomplete);

        let result = process_keys(&root, &mut current_sequence, k_event);
        assert_eq!(result, KeyProcessingResult::Incomplete);
        let result = process_keys(&root, &mut current_sequence, j_event);
        assert_eq!(
            result,
            KeyProcessingResult::Complete(Action::ExplorerAct(ExplorerAction::SelectDirectory))
        );
        assert_eq!(current_sequence.len(), 0);
        let result = process_keys(&root, &mut current_sequence, b_event);
        let result = process_keys(&root, &mut current_sequence, j_event);
        let result = process_keys(&root, &mut current_sequence, j_event);
        // assert_eq!(
        //     result,
        //     KeyProcessingResult::Complete(Action::AppAct(AppAction::CancelKeybind))
        // );
    }
}
