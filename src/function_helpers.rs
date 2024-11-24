use std::process::Command;

use crate::{
    action::{Action, AppAction},
    app::App,
};

pub fn push_current_branch(app: &mut App) -> Option<Action> {
    let branch_name = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();

    match branch_name {
        Ok(output) => {
            let branch_name = String::from_utf8(output.stdout).unwrap();
            Some(Action::AppAct(AppAction::ParseCommand(format!(
                "!git push origin {}",
                branch_name
            ))))
        }
        Err(err) => Some(Action::AppAct(AppAction::DisplayMessage(format!(
            "Failed to get current branch: {}",
            err
        )))),
    }
}
