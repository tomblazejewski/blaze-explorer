use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::action::{Action, AppAction, ExplorerAction};

#[derive(Debug)]
struct KeyMapNode {
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

    fn get_action(&self, sequence: &[KeyEvent]) -> Option<&Action> {
        let mut current_node = self;
        for key in sequence {
            match current_node.children.get(key) {
                Some(node) => current_node = node,
                None => return None,
            }
        }
        current_node.action.as_ref()
    }
}

fn default_key_map() -> KeyMapNode {
    let mut root = KeyMapNode::new();
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)],
        Action::AppAct(AppAction::Quit),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::SelectUp),
    );
    root
}
