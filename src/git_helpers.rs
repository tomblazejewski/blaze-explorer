use std::path::{Path, PathBuf};

use git2::{Repository, Status};
use ratatui::style::{Color, Style};

pub fn get_repo(path: PathBuf) -> Option<Repository> {
    Repository::open(path).ok()
}

pub fn assign_git_styling(style: Style, status: Status) -> Style {
    match status {
        Status::WT_MODIFIED => style.fg(Color::Rgb(255, 165, 0)),
        _ => style.fg(Color::Rgb(255, 192, 203)),
    }
}
