use fs_extra;
use std::{fs, io, path::PathBuf};

use chrono::offset;
use directories::ProjectDirs;

///Obtain the backup directory name to be used for storing the data. This is based on the time of
///calling the func.
pub fn get_backup_dir(create: bool) -> PathBuf {
    let mut backup_name = format!(
        "backup_{}",
        offset::Local::now().format("%d_%h_%Y_%H_%M_%S_%3f")
    );
    backup_name += ".blzbkp";
    let proj_dir = ProjectDirs::from("", "", "blaze_explorer").unwrap();
    let path = proj_dir.cache_dir().join(backup_name);
    //create this directory
    if create {
        let _ = fs::create_dir_all(&path);
    }
    path
}

pub fn join_paths(path_list: Vec<PathBuf>, new_base: &PathBuf) -> Vec<PathBuf> {
    path_list
        .iter()
        .map(|path| {
            let new_path = new_base.join(path.file_name().unwrap());
            new_path
        })
        .collect::<Vec<PathBuf>>()
}

pub fn move_recursively(
    files_to_move: Vec<PathBuf>,
    destination_path: &PathBuf,
) -> io::Result<Vec<PathBuf>> {
    let options = fs_extra::dir::CopyOptions::new();
    fs_extra::move_items(&files_to_move, &destination_path, &options)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(join_paths(files_to_move, destination_path))
}

mod tests {
    use std::{
        fs::{File, create_dir_all},
        io::Write,
    };

    use tempdir::TempDir;

    use super::*;
    #[test]
    fn test_get_backup_dir() {
        let backup_dir = get_backup_dir(true);
        assert!(backup_dir.exists());
        let backup_dir = get_backup_dir(false);
        assert!(!backup_dir.exists());
    }

    #[test]
    fn test_move_recursively() -> io::Result<()> {
        //test nested folder
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
        let target_dir = TempDir::new("target_dir").unwrap();
        let expected_file_list = vec![
            target_dir.path().join("file1.txt"),
            target_dir.path().join("file2.txt"),
            target_dir.path().join("file3.txt"),
        ];
        let resulting_file_list =
            move_recursively(file_list, &target_dir.path().to_path_buf()).unwrap();
        assert_eq!(resulting_file_list, expected_file_list);
        for file in resulting_file_list.iter() {
            assert!(file.exists());
        }
        Ok(())
    }

    #[test]
    fn test_join_paths_recursively() -> io::Result<()> {
        let original_dir = TempDir::new("original_directory").unwrap();
        let target_dir = TempDir::new("target_directory").unwrap();
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
        let expected_file_list = vec![
            target_dir.path().join("file1.txt"),
            target_dir.path().join("file2.txt"),
            target_dir.path().join("file3.txt"),
        ];
        let resulting_file_list = join_paths(file_list, &target_dir.path().to_path_buf());
        assert_eq!(resulting_file_list, expected_file_list);
        Ok(())
    }
}
