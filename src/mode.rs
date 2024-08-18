use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Search,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::Search => write!(f, "Search"),
        }
    }
}
