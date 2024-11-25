use std::process::Command;

use crate::{
    action::{Action, AppAction},
    app::App,
};

pub fn push_current_branch(_app: &mut App) -> Option<Action> {
    let branch_name = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();

    match branch_name {
        Ok(output) => {
            let mut branch_name = String::from_utf8(output.stdout).unwrap();
            if branch_name.ends_with("\n") {
                branch_name.pop();
            }
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
