use super::Action::AppAct;
#[macro_export]
macro_rules! match_popup_call {
    // Case where the function has no return value (unit type `()`) and no arguments
    ($app:ident, $func:ident) => {
        match &mut $app.popup {
            None => None,
            Some(ref mut popup) => {
                popup.$func();
                None
            }
        }
    };

    // Case where the function has no return value (unit type `()`)
    ($app:ident, $func:ident, $($args:expr),*) => {
        match &mut $app.popup {
            None => None,
            Some(ref mut popup) => {
                popup.$func($($args),*);
                None
            }
        }
    };

    // Case where the function has a return value and no arguments
    ($app:ident, $func:ident -> $ret:ty) => {
        match &mut $app.popup {
            None => None,
            Some(ref mut popup) => popup.$func(),
        }
    };

    // Case where the function has a return value
    ($app:ident, $func:ident, $($args:expr),*; -> $ret:ty) => {
        match &mut $app.popup {
            None => None,
            Some(ref mut popup) => popup.$func($($args),*),
        }
    };
}
#[macro_export]
macro_rules! custom_action {
    ($func:ident) => {
        Action::AppAct(AppAction::ExecuteFunction(Box::new($func)))
    };
}

pub use custom_action;

pub use match_popup_call;

#[cfg(test)]
mod tests {
    use crate::{
        action::{Action, AppAction},
        app::App,
    };

    use super::*;

    #[test]
    fn test_custom_action() {
        let action_func: fn(app: &mut App) -> Option<Action> = |mut app| None;
        let action = custom_action!(action_func);
        assert_eq!(
            action,
            Action::AppAct(AppAction::ExecuteFunction(Box::new(action_func)))
        );
    }
}
