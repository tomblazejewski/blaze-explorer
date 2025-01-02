use std::path::PathBuf;

use crate::{components::explorer_manager::ExplorerManager, mode::Mode};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub current_directory: PathBuf,
    pub explorer_manager: ExplorerManager,
    pub command: String,
    pub mode: Mode,
}

impl AppContext {
    pub fn new(
        current_directory: PathBuf,
        explorer_manager: ExplorerManager,
        command: String,
        mode: Mode,
    ) -> Self {
        Self {
            current_directory,
            explorer_manager,
            command,
            mode,
        }
    }
}
