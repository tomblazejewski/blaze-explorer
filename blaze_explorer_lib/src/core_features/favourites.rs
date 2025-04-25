use std::path::PathBuf;

use crate::command::command_utils::get_project_dir;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    favourites: Vec<String>,
}

impl Config {
    fn new(favourites: Vec<String>) -> Self {
        Config { favourites }
    }
    fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn add_favourite(&mut self, path: String) {
        if !self.favourites.contains(&path) {
            self.favourites.push(path);
        }
    }

    fn remove_favourite(&mut self, path: &str) {
        self.favourites.retain(|f| f != path);
    }
}

impl Default for Config {
    fn default() -> Self {
        let config_dir = get_project_dir().data_dir().join("config.json");
        Config::load_from_file(config_dir).unwrap_or_else(|_| Config { favourites: vec![] })
    }
}

mod tests {
    use crate::testing_utils::create_custom_testing_folder;

    use super::Config;

    #[test]
    fn test_config_load_write() {
        let config = Config::new(vec!["test".to_string(), "test2".to_string()]);
        let test_dir = create_custom_testing_folder(vec![]).unwrap();
        let root = test_dir.root_dir.path();
        let config_path = root.join("config.json");
        config.save_to_file(&config_path).unwrap();

        let new_config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(new_config.favourites, config.favourites);
    }

    #[test]
    fn test_add_favourite() {
        let mut config = Config::new(vec!["test".to_string(), "test2".to_string()]);
        config.add_favourite("test3".to_string());
        assert_eq!(config.favourites.len(), 3);
        assert!(config.favourites.contains(&"test3".to_string()));
    }

    #[test]
    fn test_remove_favourite() {
        let mut config = Config::new(vec!["test".to_string(), "test2".to_string()]);
        config.remove_favourite("test");
        assert_eq!(config.favourites.len(), 1);
        assert!(!config.favourites.contains(&"test".to_string()));
    }
}
