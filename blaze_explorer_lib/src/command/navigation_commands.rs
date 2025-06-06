use std::path::PathBuf;

use crate::{action::Action, app::App};

use super::Command;

#[derive(Clone, PartialEq, Debug)]
pub struct ChangeDirectory {
    new_path: PathBuf,
}

impl ChangeDirectory {
    pub fn new(mut _ctx: App, path: PathBuf) -> Self {
        Self { new_path: path }
    }
}

impl Command for ChangeDirectory {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.move_directory(self.new_path.clone(), None);
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ParentDirectory {}

impl ParentDirectory {
    pub fn new(mut _ctx: App) -> Self {
        ParentDirectory {}
    }
}

impl Command for ParentDirectory {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.go_up();
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SelectUp {}

impl SelectUp {
    pub fn new(_ctx: App) -> Self {
        Self {}
    }
}
impl Command for SelectUp {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.previous();
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SelectDown {}

impl SelectDown {
    pub fn new(_ctx: App) -> Self {
        Self {}
    }
}
impl Command for SelectDown {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.next();
        None
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct JumpToId {
    id: usize,
}

impl JumpToId {
    pub fn new(mut _ctx: App, id: usize) -> Self {
        Self { id }
    }
}

impl Command for JumpToId {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.jump_to_id(self.id);
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SelectDirectory {
    path: Option<PathBuf>,
}

impl SelectDirectory {
    pub fn new(mut ctx: App) -> Self {
        Self {
            path: ctx.explorer_manager.select_directory(),
        }
    }
}

impl Command for SelectDirectory {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        if let Some(path) = &self.path {
            match path.is_dir() {
                true => app.move_directory(path.clone(), None),
                false => app.open_default(path.clone()),
            }
        }
        None
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct JumpToStart {}

impl JumpToStart {
    pub fn new(_ctx: App) -> Self {
        Self {}
    }
}

impl Command for JumpToStart {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        app.explorer_manager.jump_to_id(0);
        None
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct JumpToEnd {}

impl JumpToEnd {
    pub fn new(_ctx: App) -> Self {
        Self {}
    }
}

impl Command for JumpToEnd {
    fn execute(&mut self, app: &mut App) -> Option<Action> {
        let count = app.explorer_manager.find_elements("").len();
        let id = match count {
            0 => 0,
            _ => count - 1,
        };
        app.explorer_manager.jump_to_id(id);
        None
    }
}
mod tests {
    use std::{env, fs::create_dir_all, path};

    use crate::{
        action::{Action, ExplorerAction},
        app::App,
        command::Command,
        testing_utils::create_testing_folder,
    };

    use super::JumpToStart;

    #[test]
    fn test_change_directory() {
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        let abs_path = path::absolute("../tests/").unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                abs_path.clone(),
            )));
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_current_path(), abs_path);
        app.move_directory(starting_path, None);
    }

    #[test]
    fn test_select_up_down() {
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        let abs_path = path::absolute("../tests/").unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                abs_path.clone(),
            )));
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::SelectDown));
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_selected(), Some(1));
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::SelectUp));
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_selected(), Some(0));
        app.move_directory(starting_path, None);
    }
    #[test]
    fn test_parent_directory() {
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        let abs_path = path::absolute("../tests/folder_1").unwrap();
        let parent_path = path::absolute("../tests/").unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                abs_path.clone(),
            )));
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_current_path(), abs_path);

        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ParentDirectory));
        let _ = app.handle_new_actions();

        assert_eq!(app.explorer_manager.get_current_path(), parent_path);

        app.move_directory(starting_path, None);
    }

    #[test]
    fn test_jump_to_id() {
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::JumpToId(2)));
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_selected(), Some(2));
        app.move_directory(starting_path, None);
    }

    #[test]
    fn test_select_directory() {
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        let abs_path = path::absolute("../tests/").unwrap();
        let expected_path = path::absolute("../tests/folder_1").unwrap();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::ChangeDirectory(
                abs_path.clone(),
            )));
        let _ = app.handle_new_actions();
        app.action_list
            .push_back(Action::ExplorerAct(ExplorerAction::SelectDirectory));
        let _ = app.handle_new_actions();
        assert_eq!(app.explorer_manager.get_current_path(), expected_path);
        app.move_directory(starting_path, None);
    }

    #[test]
    fn test_jump_to_start() {
        let testing_folder = create_testing_folder().unwrap();
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        let root_path = testing_folder.root_dir.path().to_path_buf();
        app.move_directory(root_path.clone(), Some("file_2.txt".to_string()));
        let mut jump_to_start = JumpToStart::new(app.clone());

        jump_to_start.execute(&mut app);
        assert_eq!(app.explorer_manager.get_selected(), Some(0));

        let empty_folder_path = root_path.clone().join("empty_folder");
        create_dir_all(empty_folder_path.clone()).unwrap();
        app.move_directory(empty_folder_path, None);

        jump_to_start.execute(&mut app);

        assert_eq!(app.explorer_manager.get_selected(), Some(0));
        app.move_directory(starting_path, None);
    }

    #[test]
    fn test_jump_to_end() {
        let testing_folder = create_testing_folder().unwrap();
        let mut app = App::new().unwrap();
        let starting_path = env::current_dir().unwrap();
        let root_path = testing_folder.root_dir.path().to_path_buf();
        app.move_directory(root_path.clone(), Some("file_2.txt".to_string()));
        let mut jump_to_end = super::JumpToEnd::new(app.clone());

        jump_to_end.execute(&mut app);
        let expected_id = 2usize;
        assert_eq!(app.explorer_manager.get_selected(), Some(expected_id));

        let empty_folder_path = root_path.clone().join("empty_folder");
        create_dir_all(empty_folder_path.clone()).unwrap();
        app.move_directory(empty_folder_path, None);

        jump_to_end.execute(&mut app);

        assert_eq!(app.explorer_manager.get_selected(), Some(0));
        app.move_directory(starting_path, None);
    }
}
