use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[macro_export]
macro_rules! insert_binding {
    ($map:expr, $mode:expr, $binding:expr, $functionality:expr) => {{
        let events = convert_str_to_events($binding);
        $map.insert(($mode, events), $functionality.to_string());
    }};
}
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

fn parse_key_sequence(sequence: &str) -> Option<(KeyCode, KeyModifiers)> {
    let mut modifiers = KeyModifiers::NONE;
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

        let input_complex = "<C-a> <S-h>c";
        let events_complex = convert_str_to_events(input_complex);
        let expected_events_complex = vec![
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL),
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::SHIFT),
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE),
        ];
        assert_eq!(events_complex, expected_events_complex);
    }

    #[test]
    fn test_insert_binding() {
        let mut telescope_bindings = HashMap::new();
        let binding_str = "<C-a>";
        let functionality_str = "OpenSFS";
        insert_binding!(
            telescope_bindings,
            Mode::Normal,
            binding_str,
            functionality_str
        );
        let mut expected_map = HashMap::new();
        expected_map.insert(
            (
                Mode::Normal,
                vec![KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)],
            ),
            "OpenSFS".to_string(),
        );
        assert_eq!(telescope_bindings, expected_map);
    }
}
