use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn lookup_composite_char(expression: &str) -> Vec<KeyEvent> {
    let mut result = Vec::new();
    match expression.contains('-') {
        true => {
            let parts = expression.split('-').collect::<Vec<&str>>();
            let new_key = KeyEvent::new(
                KeyCode::Char(parts[1].chars().nth(0).unwrap()),
                KeyModifiers::CONTROL,
            );
            result.push(new_key);
        }
        false => match expression {
            "cr" => result.push(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
            _ => panic!("Unknown key {}", expression),
        },
    }
    result
}

pub fn decode_expression(expression: String) -> Vec<KeyEvent> {
    let mut key_chain = Vec::new();
    let mut temporary_chain = String::new();
    let mut in_brackets = false;

    let key_translator =
        HashMap::from([(':', KeyEvent::new(KeyCode::Char(':'), KeyModifiers::SHIFT))]);

    for c in expression.chars() {
        match c {
            '<' => {
                in_brackets = true;
            }
            '>' => {
                in_brackets = false;
                let key = lookup_composite_char(&temporary_chain);
                temporary_chain.clear();
                key_chain.extend(key);
            }
            _ => {
                if in_brackets {
                    temporary_chain.push(c);
                } else {
                    match key_translator.get(&c) {
                        Some(key) => {
                            key_chain.push(key.clone());
                        }
                        None => key_chain.push(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)),
                    }
                }
            }
        }
    }
    key_chain
}

mod tests {

    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use crate::command::key_press::lookup_composite_char;

    use super::decode_expression;

    #[test]
    fn test_decode_with_translator() {
        let expression = ":w";
        let expected = vec![
            KeyEvent::new(KeyCode::Char(':'), KeyModifiers::SHIFT),
            KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE),
        ];
        let result = decode_expression(expression.to_string());
        assert_eq!(result, expected);
    }
    #[test]
    fn test_decode_entire_expression() {
        let expression = "<C-a>w";
        let expected = vec![
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL),
            KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE),
        ];
        let result = decode_expression(expression.to_string());
        assert_eq!(result, expected);
    }
    #[test]
    fn test_decode_composite() {
        let expression = "cr";
        let expected = vec![KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)];
        let result = lookup_composite_char(expression);
        assert_eq!(result, expected);
    }
}
