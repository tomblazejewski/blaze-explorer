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
        Status::WT_MODIFIED => style.fg(Color::Rgb(255, 215, 0)),
        Status::INDEX_MODIFIED => style.fg(Color::Rgb(255, 140, 0)),
        Status::IGNORED => style.fg(Color::Rgb(128, 128, 128)),
        Status::WT_NEW => style.fg(Color::Rgb(152, 251, 152)),
        Status::INDEX_NEW => style.fg(Color::Rgb(0, 100, 0)),
        _ => style.fg(Color::Rgb(255, 20, 147)),
    }
}
