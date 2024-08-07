use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::action::Action;

#[derive(Clone)]
pub enum NumberCombination {
    Multiplier(u32),
    None,
}

#[derive(Clone)]
pub enum KeyCombination {
    Chain(Vec<KeyEvent>),
    None,
}

pub enum KeyEnterResult {
    Append(KeyEvent),
    ClearAndAppend(KeyEvent),
}

pub struct KeyTracker {
    number_combination: NumberCombination,
    key_combination: KeyCombination,
    last_digit: bool,
    last_key_event: Option<KeyEvent>,
    key_hash_map: HashMap<Vec<KeyEvent>, (Action, bool)>,
}

pub fn is_multiplier_digit(char_: &char) -> bool {
    if char_.is_ascii_digit() && *char_ != '0' {
        return true;
    }
    return false;
}

impl KeyTracker {
    pub fn new() -> Self {
        let keyboard_keymaps = HashMap::from([
            (
                vec![KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)],
                (Action::Quit, false),
            ),
            (
                vec![KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)],
                (Action::SelectUp, true),
            ),
            (
                vec![KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE)],
                (Action::SelectDown, true),
            ),
            (
                vec![KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)],
                (Action::EscapeSequence, false),
            ),
            (
                vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)],
                (Action::SelectDirectory, false),
            ),
            (
                vec![KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)],
                (Action::ParentDirectory, false),
            ),
        ]);
        Self {
            number_combination: NumberCombination::None,
            key_combination: KeyCombination::None,
            last_digit: false,
            last_key_event: None,
            key_hash_map: keyboard_keymaps,
        }
    }

    pub fn clear_key_combination(&mut self) {
        self.key_combination = KeyCombination::None;
    }

    pub fn accept_digit(&mut self, digit_char: char) {
        let digit_option = digit_char.to_digit(10);
        let number = digit_option.unwrap();
        match self.number_combination {
            NumberCombination::None => {
                self.number_combination = NumberCombination::Multiplier(number);
            }
            NumberCombination::Multiplier(existing_number) => {
                self.number_combination =
                    NumberCombination::Multiplier(existing_number * 10 + number);
            }
        }
    }

    pub fn accept_non_digit(&mut self, new_event: KeyEvent) {
        if let KeyCombination::Chain(ref mut event_vec) = self.key_combination {
            event_vec.push(new_event);
        } else {
            self.key_combination = KeyCombination::Chain(vec![new_event]);
        }
    }

    pub fn append_key_event(&mut self, new_event: KeyEvent) {
        // three options here: key is a digit char, key is another char or key is not a char at all
        // if key is not a char at all just append it to key combination and sequence
        // if key is a char, can always add it
        // if key is a char digit, cancel any existing number combinations first
        self.last_key_event = Some(new_event.clone());
        match new_event.code {
            KeyCode::Char(new_char) => {
                if is_multiplier_digit(&new_char) {
                    self.accept_digit(new_char);
                    self.last_digit = true;
                } else {
                    self.accept_non_digit(new_event);
                    self.last_digit = false;
                }
            }
            _ => {
                self.accept_non_digit(new_event);
                self.last_digit = false;
            }
        }
    }

    pub fn find_action(&mut self, keymap: Vec<KeyEvent>, multiplier: u32) -> Option<Vec<Action>> {
        let search_result = self.key_hash_map.get(&keymap);
        if let Some(result_found) = search_result {
            let (action, is_repeatable) = result_found;
            match is_repeatable {
                false => Some(vec![action.clone()]),
                true => Some(vec![action.clone(); multiplier.try_into().unwrap()]),
            }
        } else {
            None
        }
    }

    pub fn handle_keymap(&mut self) -> Option<Vec<Action>> {
        match (
            self.number_combination.clone(),
            self.key_combination.clone(),
            self.last_digit,
        ) {
            (NumberCombination::None, KeyCombination::Chain(keymap), _) => {
                // simple keymap with no numbers in it - search for it
                let result = self.find_action(keymap.clone(), 1);
                return result;
            }
            (NumberCombination::Multiplier(_), KeyCombination::Chain(_), true) => {
                //has some keymap but entered a digit
                //clear and add a digit
                return Some(vec![Action::ClearAndKey(self.last_key_event.unwrap())]);
            }
            (NumberCombination::Multiplier(n), KeyCombination::Chain(keymap), false) => {
                //keymap with some multiplier
                let result = self.find_action(keymap.clone(), n);
                return result;
            }
            (NumberCombination::Multiplier(_), KeyCombination::None, _) => {
                //no key combination so can only do noop
                return Some(vec![Action::Noop]);
            }
            (NumberCombination::None, KeyCombination::None, _) => {
                //no keys so far so can only noop
                return Some(vec![Action::Noop]);
            }
        }
    }
}
