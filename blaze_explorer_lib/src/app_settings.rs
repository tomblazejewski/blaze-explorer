use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub proj_dir: ProjectDirs,
    pub favourites: Vec<String>,
}

impl AppSettings {
    pub fn new() -> Self {
        let proj_dir = ProjectDirs::from("", "", "blaze_explorer").unwrap();
        let favourites = load_favourites(&proj_dir);
        Self {
            proj_dir,
            favourites,
        }
    }
}

fn load_favourites(proj_dir: &ProjectDirs) -> Vec<String> {
    let data_dir = proj_dir.data_local_dir();
    let favourites = data_dir.join("favourites.txt");
    let reader = read_to_string(&favourites).unwrap();
    let mut directories = Vec::new();

    for line in reader.lines() {
        if !line.trim().is_empty() && Path::new(line.trim()).exists() {
            directories.push(line.trim().to_string());
        }
    }

    directories
}
