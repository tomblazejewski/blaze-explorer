use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Config {
    pub favourites: Vec<PathBuf>,
}

impl Config {
    pub fn new(favourites: Vec<PathBuf>) -> Self {
        Config { favourites }
    }
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn add_favourite(&mut self, path: PathBuf) {
        if !self.favourites.contains(&path) {
            self.favourites.push(path);
        }
    }

    pub fn remove_favourite(&mut self, path: PathBuf) {
        self.favourites.retain(|f| *f != path);
    }

    pub fn toggle_favourite(&mut self, path: PathBuf) {
        if self.favourites.contains(&path) {
            self.remove_favourite(path);
        } else {
            self.add_favourite(path);
        }
    }
}

mod tests {
    use crate::testing_utils::create_custom_testing_folder;

    use super::Config;

    #[test]
    fn test_config_load_write() {
        let config = Config::new(vec!["test1".into(), "test2".into()]);
        let test_dir = create_custom_testing_folder(vec![]).unwrap();
        let root = test_dir.root_dir.path();
        let config_path = root.join("config.json");
        config.save_to_file(&config_path).unwrap();

        let new_config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(new_config.favourites, config.favourites);
    }

    #[test]
    fn test_add_favourite() {
        let mut config = Config::new(vec!["test".into(), "test2".into()]);
        config.add_favourite("test3".into());
        assert_eq!(config.favourites.len(), 3);
        assert!(config.favourites.contains(&"test3".into()));
    }

    #[test]
    fn test_remove_favourite() {
        let mut config = Config::new(vec!["test".into(), "test2".into()]);
        config.remove_favourite("test".into());
        assert_eq!(config.favourites.len(), 1);
        assert!(!config.favourites.contains(&"test".into()));
    }
}
