use enigo::{Direction, Key};

#[derive(Debug, PartialEq)]
pub struct EnigoKey {
    pub key: Key,
    pub direction: Direction,
}
pub fn lookup_composite_char(expression: &str) -> Vec<EnigoKey> {
    let mut result = Vec::new();
    match expression.contains('-') {
        true => {
            let parts = expression.split('-').collect::<Vec<&str>>();
            match parts[0] {
                "C" => result.push(EnigoKey {
                    key: Key::Control,
                    direction: Direction::Press,
                }),
                _ => panic!("Unknown key {}", parts[0]),
            }
            let second_part = parts[1].chars().nth(0).unwrap();
            result.push(EnigoKey {
                key: Key::Unicode(second_part),
                direction: Direction::Click,
            });
            match parts[0] {
                "C" => result.push(EnigoKey {
                    key: Key::Control,
                    direction: Direction::Release,
                }),
                _ => panic!("Unknown key {}", parts[0]),
            }
        }
        false => match expression {
            "cr" => result.push(EnigoKey {
                key: Key::Return,
                direction: Direction::Click,
            }),
            _ => panic!("Unknown key {}", expression),
        },
    }
    result
}

pub fn decode_expression(expression: String) -> Vec<EnigoKey> {
    let mut key_chain = Vec::new();
    let mut temporary_chain = String::new();
    let mut in_brackets = false;

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
                    key_chain.push(EnigoKey {
                        key: Key::Unicode(c),
                        direction: Direction::Click,
                    })
                }
            }
        }
    }
    key_chain
}

mod tests {
    use super::*;

    #[test]
    fn test_decode_entire_expression() {
        let expression = "<C-a>w";
        let expected = vec![
            EnigoKey {
                key: Key::Control,
                direction: Direction::Press,
            },
            EnigoKey {
                key: Key::Unicode('a'),
                direction: Direction::Click,
            },
            EnigoKey {
                key: Key::Control,
                direction: Direction::Release,
            },
            EnigoKey {
                key: Key::Unicode('w'),
                direction: Direction::Click,
            },
        ];
        let result = decode_expression(expression.to_string());
        assert_eq!(result, expected);
    }
    #[test]
    fn test_decode_composite() {
        let expression = "cr";
        let expected = vec![EnigoKey {
            key: Key::Return,
            direction: Direction::Click,
        }];
        let result = lookup_composite_char(expression);
        assert_eq!(result, expected);
    }
}
