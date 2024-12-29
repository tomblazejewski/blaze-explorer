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
pub use match_popup_call;
