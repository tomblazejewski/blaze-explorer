// Describes the Input machine for the main app - the implementation of the trait and the keymaps
// used in the main app
use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    action::{Action, AppAction, CommandAction, ExplorerAction, TextAction},
    core_features::rename::open_rename_popup,
    custom_action,
    function_helpers::{pull_current_branch, push_current_branch},
    input_machine::{InputMachine, KeyMapNode, KeyProcessingResult},
    mode::Mode,
    plugin::plugin_popup::PluginPopUp,
};

type DefaultActionMap = HashMap<Mode, Box<fn(KeyEvent) -> Option<Action>>>;
fn get_default_search_command_action(last_key: KeyEvent) -> Option<Action> {
    match last_key.code {
        KeyCode::Char(ch) => Some(Action::TextAct(TextAction::InsertKey(ch))),
        _ => None,
    }
}

pub fn get_none_action(last_key: KeyEvent) -> Option<Action> {
    None
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppInputMachine<T> {
    keymap_nodes: HashMap<Mode, KeyMapNode<T>>,
    default_actions: DefaultActionMap,
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
        self.default_actions.get(mode).unwrap()(last_key)
    }
}

impl AppInputMachine<Action> {
    pub fn new() -> Self {
        let mut keymap_nodes = HashMap::new();
        keymap_nodes.insert(Mode::Normal, default_key_map());
        keymap_nodes.insert(Mode::Search, search_key_map());
        keymap_nodes.insert(Mode::Command, command_key_map());
        keymap_nodes.insert(Mode::PopUp, KeyMapNode::new());
        keymap_nodes.insert(Mode::Visual, visual_key_map());

        let mut default_actions = HashMap::new();
        default_actions.insert(
            Mode::Normal,
            Box::new(get_none_action as fn(KeyEvent) -> Option<Action>),
        );
        default_actions.insert(Mode::PopUp, Box::new(get_none_action));
        default_actions.insert(Mode::Visual, Box::new(get_none_action));
        default_actions.insert(Mode::Search, Box::new(get_default_search_command_action));
        default_actions.insert(Mode::Command, Box::new(get_default_search_command_action));

        AppInputMachine {
            keymap_nodes,
            default_actions,
        }
    }

    pub fn attach_popup_bindings(&mut self, popup: Box<dyn PluginPopUp>) {
        //Reset popup mode bindings before appending
        self.keymap_nodes.insert(Mode::PopUp, KeyMapNode::new());
        let popup_keymap = popup.get_own_keymap();
        self.attach_from_hashmap(popup_keymap);
        self.default_actions
            .insert(Mode::PopUp, popup.get_default_action());
    }

    pub fn drop_popup_bindings(&mut self) {
        //nullify the default action in the Mode::PopUp
        self.default_actions
            .insert(Mode::PopUp, Box::new(get_none_action));
        //empty the Mode::PopUp keymap
        self.keymap_nodes.insert(Mode::PopUp, KeyMapNode::new());
    }

    pub fn attach_binding(&mut self, mode: Mode, sequence: Vec<KeyEvent>, action: Action) {
        self.keymap_nodes
            .get_mut(&mode)
            .unwrap()
            .add_sequence(sequence, action);
    }

    pub fn attach_from_hashmap(&mut self, keymap: HashMap<(Mode, Vec<KeyEvent>), Action>) {
        for ((mode, seq), action) in keymap {
            self.attach_binding(mode, seq, action);
        }
    }
}
pub fn process_app_keys(
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
        vec![KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE)],
        Action::AppAct(AppAction::SwitchMode(Mode::Visual)),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::ClearSearchQuery),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE)],
        custom_action!(open_rename_popup),
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
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('o'), KeyModifiers::CONTROL)],
        Action::AppAct(AppAction::UndoDirectory),
    );
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Char('i'), KeyModifiers::CONTROL)],
        Action::AppAct(AppAction::RedoDirectory),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE),
        ],
        Action::AppAct(AppAction::ParseKeyStrokes(
            r#":!git commit -am ""#.to_string(),
        )),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
        ],
        Action::AppAct(AppAction::ParseCommand("!git status".to_string())),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('P'), KeyModifiers::NONE),
        ],
        Action::AppAct(AppAction::ExecuteFunction(Box::new(push_current_branch))),
    );
    root.add_sequence(
        vec![
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('O'), KeyModifiers::NONE),
        ],
        Action::AppAct(AppAction::ExecuteFunction(Box::new(pull_current_branch))),
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
pub fn visual_key_map() -> KeyMapNode<Action> {
    let mut root = KeyMapNode::new();
    root.add_sequence(
        vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
        Action::AppAct(AppAction::SwitchMode(Mode::Normal)),
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
        vec![KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE)],
        Action::ExplorerAct(ExplorerAction::ToggleMark),
    );
    root
}
