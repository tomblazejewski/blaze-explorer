// Defines the skeleton of an input machine that can be used by an appropriate app - be it the main
// app or the telescope backend
use std::collections::HashMap;

use ratatui::crossterm::event::KeyEvent;

use crate::{action::Action, mode::Mode};

pub trait InputMachine {
    fn process_keys(
        &mut self,
        mode: &Mode,
        current_sequence: &mut Vec<KeyEvent>,
        input_key: KeyEvent,
    ) -> KeyProcessingResult<Action>;

    fn get_default_action(&self, mode: &Mode, last_key: KeyEvent) -> Option<Action>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyMapNode<T> {
    pub action: Option<T>,
    children: HashMap<KeyEvent, KeyMapNode<T>>,
}

impl<T> KeyMapNode<T> {
    pub fn new() -> Self {
        KeyMapNode {
            action: None,
            children: HashMap::new(),
        }
    }

    pub fn add_sequence(&mut self, sequence: Vec<KeyEvent>, action: T) {
        let mut current_node = self;
        for key in sequence {
            current_node = current_node
                .children
                .entry(key)
                .or_insert_with(KeyMapNode::new);
        }
        current_node.action = Some(action);
    }

    pub fn get_node(&self, sequence: &[KeyEvent]) -> Option<&KeyMapNode<T>> {
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
pub enum KeyProcessingResult<T> {
    Complete(T), // Sequence is complete and valid
    Incomplete,  // Sequence is valid but not yet complete
    Invalid,     // Sequence is invalid
}

pub fn process_keys<T: Clone>(
    keymap: &KeyMapNode<T>,
    current_sequence: &mut Vec<KeyEvent>,
    input_key: KeyEvent,
) -> KeyProcessingResult<T> {
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

#[cfg(test)]
mod tests {

    use ratatui::crossterm::event::{KeyCode, KeyModifiers};

    use crate::action::{Action, AppAction, ExplorerAction};

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
            Action::AppAct(AppAction::Delete),
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
        let k_event = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE);

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
    }
}
