use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tracing::info;

use crate::action::Action;

#[derive(Clone, Debug)]
pub enum NumberCombination {
    Multiplier(u32),
    None,
}

#[derive(Clone, Debug)]
pub enum KeyCombination {
    Chain(Vec<KeyEvent>),
    None,
}

pub struct KeyManager {
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
    false
}

impl KeyManager {
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

    pub fn clear_keys_stored(&mut self) {
        self.key_combination = KeyCombination::None;
        self.number_combination = NumberCombination::None;
        self.last_digit = false;
        self.last_key_event = None;
    }

    pub fn clear_and_enter(&mut self, new_event: KeyEvent) {
        self.clear_keys_stored();
        self.append_key_event(new_event);
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
                } else if new_char == '0' {
                    match self.number_combination {
                        NumberCombination::None => {
                            self.accept_non_digit(new_event);
                            self.last_digit = false;
                        }
                        NumberCombination::Multiplier(_) => {
                            self.accept_digit(new_char);
                            self.last_digit = true;
                        }
                    }
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

    pub fn find_action(&mut self, keymap: Vec<KeyEvent>, multiplier: u32) -> Vec<Action> {
        let search_result = self.key_hash_map.get(&keymap);
        if let Some(result_found) = search_result {
            let (action, is_repeatable) = result_found;
            let mut actions_returned = match is_repeatable {
                false => {
                    vec![action.clone()]
                }
                true => vec![action.clone(); multiplier.try_into().unwrap()],
            };
            match action {
                Action::Noop => {}
                _ => {
                    actions_returned.push(Action::EscapeSequence);
                }
            }
            info!("For {:?} returning {:?}", keymap, actions_returned);
            actions_returned
        } else {
            vec![Action::EscapeSequence]
        }
    }

    pub fn handle_keymap(&mut self) -> Vec<Action> {
        info!(
            "Searching for {:?}, {:?}, {:?}",
            self.number_combination, self.key_combination, self.last_digit,
        );
        match (
            self.number_combination.clone(),
            self.key_combination.clone(),
            self.last_digit,
        ) {
            (NumberCombination::None, KeyCombination::Chain(keymap), _) => {
                // simple keymap with no numbers in it - search for it
                let result = self.find_action(keymap.clone(), 1);
                result
            }
            (NumberCombination::Multiplier(_), KeyCombination::Chain(_), true) => {
                //has some keymap but entered a digit
                //clear and add a digit
                vec![Action::ClearAndKey(self.last_key_event.unwrap())]
            }
            (NumberCombination::Multiplier(n), KeyCombination::Chain(keymap), false) => {
                //keymap with some multiplier
                let result = self.find_action(keymap.clone(), n);
                result
            }
            (NumberCombination::Multiplier(_), KeyCombination::None, _) => {
                //no key combination so can only do noop
                vec![Action::Noop]
            }
            (NumberCombination::None, KeyCombination::None, _) => {
                //no keys so far so can only noop
                vec![Action::Noop]
            }
        }
    }
}
