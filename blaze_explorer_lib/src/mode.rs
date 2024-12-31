use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Search,
    Command,
    PopUp,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::Search => write!(f, "Search"),
            Mode::Command => write!(f, "Command"),
            Mode::PopUp => write!(f, "PopUp"),
        }
    }
}
