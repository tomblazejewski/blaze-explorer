use crate::plugin::{
    base_popup::{BasePopUp, GenericPopUp, Popupbehaviour, get_default_popup_keymap},
    plugin_action::PluginAction,
};
use std::{collections::HashMap, path::PathBuf};

use ratatui::crossterm::event::KeyEvent;

use crate::{
    action::Action,
    app::App,
    command::file_commands::RenameActive,
    create_plugin_action,
    input_machine::input_machine_helpers::convert_str_to_events,
    mode::Mode,
    plugin::plugin_commands::{PluginConfirmResult, PluginDropSearchChar, PluginQuit},
    query::Query,
};
pub fn open_rename_popup(app: &mut App) -> Option<Action> {
    let mut ctx = app.get_app_context();
    let dir = ctx.explorer_manager.select_directory().unwrap().clone();

    let initial_name = dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string();
    let extension = dir
        .extension()
        .map_or("".to_string(), |e| format!(".{}", e.to_str().unwrap()));

    let query = Query::new_with_contents("".to_string(), initial_name.clone(), extension.clone());
    let keymap = get_default_popup_keymap();

    let base = BasePopUp {
        should_quit: false,
        query,
        keymap,
    };

    let behaviour = RenameBehaviour {
        initial_path: dir,
        initial_name,
        extension,
    };

    app.attach_popup(Box::new(GenericPopUp { base, behaviour }));
    None
}

#[derive(Debug, Clone, PartialEq)]
struct RenameBehaviour {
    initial_path: PathBuf,
    initial_name: String,
    extension: String,
}

impl Popupbehaviour for RenameBehaviour {
    fn popup_title(&self) -> String {
        format!("Rename {}", self.initial_name)
    }

    fn confirm_action(&self, query: String) -> Action {
        create_plugin_action!(RenameActive, self.initial_path.clone(), query)
    }

    fn display_details(&self) -> String {
        format!("Rename {}{}", self.initial_name, self.extension)
    }
}
#[cfg(test)]
mod tests {

    use crate::{
        plugin::{base_popup::get_default_popup_keymap, plugin_popup::PluginPopUp},
        testing_utils::create_custom_testing_folder,
    };

    use super::*;

    #[test]
    fn test_open_rename_popup() {
        let test_folder = create_custom_testing_folder(vec!["file.txt"]).unwrap();
        let initial_path = test_folder.root_dir.path().join("file.txt");

        let mut app = App::new().unwrap();
        app.explorer_manager.show_in_folder(initial_path.clone());
        let action = open_rename_popup(&mut app);

        let query = Query::new_with_contents("".into(), "file".into(), ".txt".into());
        let keymap = get_default_popup_keymap();
        let behaviour = RenameBehaviour {
            initial_path: initial_path.clone(),
            initial_name: "file".into(),
            extension: ".txt".into(),
        };
        let base = BasePopUp {
            should_quit: false,
            query,
            keymap,
        };
        let expected = Box::new(GenericPopUp { base, behaviour }) as Box<dyn PluginPopUp>;
        let attached_popup = app.popup.unwrap();

        assert!(attached_popup == expected);
        assert!(action.is_none());
    }
}
