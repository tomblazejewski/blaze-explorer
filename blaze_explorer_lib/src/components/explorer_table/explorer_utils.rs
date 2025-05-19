use std::path::PathBuf;

/// A struct representing the information used live by the ExplorerManager e.g. for drawing
///that is passed from the app.
pub struct FileConfig {
    pub favourites: Vec<PathBuf>,
    pub string_sequence: String,
}

impl FileConfig {
    pub fn new(favourites: Vec<PathBuf>, string_sequence: String) -> Self {
        FileConfig {
            favourites,
            string_sequence,
        }
    }
}
