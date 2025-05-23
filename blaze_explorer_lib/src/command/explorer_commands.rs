use std::path::PathBuf;

use crate::{action::Action, app::App, app_context::AppContext};

use super::Command;

#[derive(Clone, PartialEq, Debug)]
pub struct ToggleToFavourites {
    path: PathBuf,
}

impl ToggleToFavourites {
    pub fn new(mut ctx: AppContext) -> Self {
        let path = ctx.explorer_manager.get_current_path();
        Self { path }
    }
}

impl Command for ToggleToFavourites {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.config.toggle_favourite(self.path.clone());
        None
    }
}

mod tests {
    use crate::command::Command;
    use crate::components::explorer_manager;
    use crate::testing_utils::create_custom_testing_folder;

    use super::ToggleToFavourites;
    use crate::app::App;
    use crate::app_context::AppContext;

    #[test]
    fn test_toggle_to_favourites() {
        let mut app = App::new().unwrap();
        let testing_folder = create_custom_testing_folder(vec!["test_1/"]).unwrap();
        let root_dir = testing_folder.root_dir.path().to_path_buf();

        let path = root_dir.join("test_1");
        app.explorer_manager.show_in_folder(path.clone());
        let ctx = app.get_app_context();
        let mut command = ToggleToFavourites::new(ctx);
        command.execute(&mut app);

        assert!(app.config.favourites.contains(&root_dir));

        // Toggle again
        command.execute(&mut app);

        assert!(!app.config.favourites.contains(&root_dir));
    }
}
