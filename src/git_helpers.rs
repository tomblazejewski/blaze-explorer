use std::path::{Path, PathBuf};

use git2::{Repository, RepositoryOpenFlags, Status};
use ratatui::style::{Color, Style};

pub fn get_repo(path: PathBuf) -> Option<Repository> {
    Repository::open_ext(
        path,
        RepositoryOpenFlags::empty(),
        &[] as &[&std::ffi::OsStr],
    )
    .ok()
}

pub fn assign_git_styling(style: Style, status: Status) -> Style {
    match status {
        Status::WT_MODIFIED => style.fg(Color::Rgb(255, 165, 0)),
        Status::IGNORED => style.fg(Color::Rgb(128, 128, 128)),
        Status::WT_NEW => style.fg(Color::Rgb(0, 128, 0)),
        _ => style.fg(Color::Rgb(255, 192, 203)),
    }
}
