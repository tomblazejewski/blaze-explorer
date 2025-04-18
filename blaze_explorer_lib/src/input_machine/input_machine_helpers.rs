use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn convert_str_to_events(input: &str) -> Vec<KeyEvent> {
    let mut events = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            // Handle special key sequences, e.g., <C-a>, <S-h>
            let mut key_sequence = String::new();
            while let Some(&next_char) = chars.peek() {
                if next_char == '>' {
                    chars.next(); // Consume '>'
                    break;
                }
                key_sequence.push(next_char);
                chars.next();
            }

            if let Some((key, modifiers)) = parse_key_sequence(&key_sequence) {
                events.push(KeyEvent::new(key, modifiers));
            }
        } else {
            // Handle regular characters
            events.push(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }
    }

    events
}

pub fn parse_key_sequence(sequence: &str) -> Option<(KeyCode, KeyModifiers)> {
    let mut modifiers = KeyModifiers::NONE;

    match sequence {
        "Esc" => Some((KeyCode::Esc, modifiers)),
        "CR" => Some((KeyCode::Enter, modifiers)),
        "BS" => Some((KeyCode::Backspace, modifiers)),
        _ => {
            let mut key_char = None;
            for part in sequence.split('-') {
                match part {
                    "C" => modifiers |= KeyModifiers::CONTROL,
                    "S" => modifiers |= KeyModifiers::SHIFT,
                    "A" => modifiers |= KeyModifiers::ALT,
                    _ if part.len() == 1 => key_char = Some(part.chars().next().unwrap()),
                    _ => return None, // Invalid sequence
                }
            }

            if let Some(c) = key_char {
                Some((KeyCode::Char(c), modifiers))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ratatui::crossterm::event::{KeyCode, KeyModifiers};

    use crate::mode::Mode;

    use super::*;

    #[test]
    fn test_convert_str_to_events() {
        let input = "abc";
        let events = convert_str_to_events(input);
        let expected_events = vec![
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE),
        ];

        assert_eq!(events, expected_events);

        let input_complex = "<C-a> <S-h>c<Esc><CR><BS>";
        let events_complex = convert_str_to_events(input_complex);
        let expected_events_complex = vec![
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL),
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::SHIFT),
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
        ];
        assert_eq!(events_complex, expected_events_complex);
    }
}
