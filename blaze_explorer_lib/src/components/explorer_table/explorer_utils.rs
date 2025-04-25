pub struct FileConfig {
    favourites: Vec<String>,
    string_sequence: String,
}

impl FileConfig {
    pub fn new(favourites: Vec<String>, string_sequence: String) -> Self {
        FileConfig {
            favourites,
            string_sequence,
        }
    }
}
