use std::path::PathBuf;

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
