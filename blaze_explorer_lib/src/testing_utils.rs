use std::fs::{File, create_dir_all};
use std::io::{Result, Write};
use std::path::PathBuf;

use tempdir::TempDir;

pub struct TestingFolder {
    pub root_dir: TempDir,
    pub file_list: Vec<PathBuf>,
}

impl TestingFolder {
    pub fn new(root_dir: TempDir, file_list: Vec<PathBuf>) -> Self {
        Self {
            root_dir,
            file_list,
        }
    }
}

pub fn create_testing_folder() -> Result<TestingFolder> {
    let original_dir = TempDir::new("original_directory").unwrap();
    let folder_1 = original_dir.path().join("folder_1");
    let folder_2 = folder_1.join("folder_2");
    create_dir_all(folder_2.clone())?;
    let file_list = vec![
        original_dir.path().join("file1.txt"),
        original_dir.path().join("file2.txt"),
        folder_2.join("file3.txt"),
    ];
    for file in &file_list {
        let mut f = File::create(file)?;
        f.write_all(b"Hello, world!")?;
        f.sync_all()?;
    }
    Ok(TestingFolder::new(original_dir, file_list.to_owned()))
}
