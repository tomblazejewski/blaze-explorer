pub enum KeyType {
    SpecificKey(char),
    Digit(char),
    Letter(char),
    Char(char),
    None,
}

pub const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const LETTERS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl KeyType {
    pub fn degrade(&self) -> Self {
        match self {
            KeyType::SpecificKey(c) => match c {
                '0'..='9' => KeyType::Digit(*c),
                'a'..='z' => KeyType::Letter(*c),
                'A'..='Z' => KeyType::Letter(*c),
                _ => KeyType::Char(*c),
            },
            KeyType::Digit(c) => KeyType::Char(*c),
            KeyType::Letter(c) => KeyType::Char(*c),
            KeyType::Char(c) => KeyType::None,
            KeyType::None => KeyType::None,
        }
    }
}
