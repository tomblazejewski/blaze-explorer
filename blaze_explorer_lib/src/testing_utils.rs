use std::fs::{File, create_dir_all};
use std::io::{Result, Write};
use std::path::PathBuf;

use tempdir::TempDir;

pub struct TestingFolder {
    pub root_dir: TempDir,
    pub file_list: Vec<PathBuf>,
    pub dir_list: Vec<PathBuf>,
}

impl TestingFolder {
    pub fn new(root_dir: TempDir, file_list: Vec<PathBuf>, dir_list: Vec<PathBuf>) -> Self {
        Self {
            root_dir,
            file_list,
            dir_list,
        }
    }
}

/// Create a testing folder with the following structure:
///
///original_directory/
///├─ folder_1/
///│  ├─ folder_2/
///│  │  ├─ file_3.txt
///├─ file_1.txt
///├─ file_2.txt
/// The files can be accessed via `TestingFolder.file_list`
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
    let dir_list = vec![original_dir.path().to_path_buf(), folder_1, folder_2];
    Ok(TestingFolder::new(
        original_dir,
        file_list.to_owned(),
        dir_list,
    ))
}

/// Create a temporary folder with the structure described by the argument [paths]
/// The folder is destroyed when the TestingFolder is dropped
pub fn create_custom_testing_folder(paths: Vec<&str>) -> Result<TestingFolder> {
    let original_dir = TempDir::new("original_directory").unwrap();
    let mut file_list = Vec::new();
    let mut dir_list = vec![original_dir.path().to_path_buf()];

    for path_str in paths {
        let full_path = original_dir.path().join(path_str);
        if path_str.ends_with('/') {
            // It's a directory
            create_dir_all(&full_path)?;
            dir_list.push(full_path);
        } else {
            // It's a file — ensure parent directories exist
            if let Some(parent) = full_path.parent() {
                create_dir_all(parent)?;
                if !dir_list.contains(&parent.to_path_buf()) {
                    dir_list.push(parent.to_path_buf());
                }
            }
            let mut file = File::create(&full_path)?;
            file.write_all(b"Hello, world!")?;
            file.sync_all()?;
            file_list.push(full_path);
        }
    }

    Ok(TestingFolder::new(original_dir, file_list, dir_list))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_custom_testing_folder() {
        let paths_list = vec![
            "file_1.txt",
            "folder_1/folder_2/file_3.txt",
            "folder_2/",
            "aaa.csv",
            "bbb.xlsx",
            "folder_5/ccc.jpg",
        ];
        let testing_folder = create_custom_testing_folder(paths_list.clone()).unwrap();
        let root_dir = testing_folder.root_dir.path();
        let paths_to_check = paths_list
            .iter()
            .map(|x| root_dir.join(x))
            .collect::<Vec<PathBuf>>();
        assert!(paths_to_check.iter().all(|x| x.exists()));
    }
}
