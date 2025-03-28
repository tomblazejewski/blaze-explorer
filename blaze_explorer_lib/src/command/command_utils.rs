use clipboard_win::Setter;
use clipboard_win::empty;
use clipboard_win::formats::FileList;
use clipboard_win::get_clipboard;
use fs_extra;
use rand::distr::{Alphanumeric, SampleString};
use std::path::Path;
use std::{collections::HashMap, fs, io, path::PathBuf};

use clipboard_win::Clipboard;

use directories::ProjectDirs;

///Obtain the backup directory name to be used for storing the data. This is based on the time of
///calling the func.
pub fn get_backup_dir(create: bool) -> PathBuf {
    let mut backup_name = format!(
        "backup_{}",
        Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
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

pub fn create_backup_map(files: Vec<PathBuf>) -> HashMap<PathBuf, PathBuf> {
    files
        .into_iter()
        .map(|file_path| (file_path, get_backup_dir(false)))
        .collect::<HashMap<PathBuf, PathBuf>>()
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

pub fn copy_to_clipboard(file_paths: Vec<&str>) -> Result<(), clipboard_win::ErrorCode> {
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
    empty()?;
    match FileList.write_clipboard(&file_paths) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn read_from_clipboard() -> Result<Vec<PathBuf>, clipboard_win::ErrorCode> {
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
    let str_files = get_clipboard(FileList)?;
    let paths = str_files
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();
    Ok(paths)
}
pub fn copy_recursively(src: &Path, dest: &Path) -> io::Result<()> {
    if src.is_file() {
        fs::copy(src, dest)?;
        return Ok(());
    }
    if dest.starts_with(src) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Destination folder is a child of the source folder",
        ));
    }
    if !dest.exists() {
        fs::create_dir(dest)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if entry_path.is_dir() {
            copy_recursively(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}
mod tests {
    use std::{
        fs::{File, create_dir_all},
        io::Write,
        thread,
        time::Duration,
    };

    use tempdir::TempDir;

    use crate::testing_utils::create_testing_folder;

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
    #[test]
    fn test_copy_and_read_clipboard() -> io::Result<()> {
        let original_dir = TempDir::new("original_directory").unwrap();
        let folder_1 = original_dir.path().join("folder_1");
        let folder_2 = folder_1.join("folder_2");
        create_dir_all(folder_2.clone())?;
        let file_list = vec![
            original_dir.path().join("file1.txt"),
            original_dir.path().join("file2.txt"),
            folder_1.join("file2.txt"),
            folder_2.join("file3.txt"),
        ];
        for file in &file_list {
            let mut f = File::create(file)?;
            f.write_all(b"Hello, world!")?;
            f.sync_all()?;
        }
        let str_path_list = file_list
            .iter()
            .map(|f| f.to_str().unwrap())
            .collect::<Vec<_>>();
        copy_to_clipboard(str_path_list).unwrap();
        let resulting_paths = read_from_clipboard().unwrap();
        assert_eq!(file_list, resulting_paths);
        Ok(())
    }

    #[test]
    fn test_copy_recursively_subfolder() -> io::Result<()> {
        let test_folder = create_testing_folder().unwrap();
        let folder_to_copy = test_folder.dir_list[1].clone();
        let location_to_paste = test_folder.dir_list[2].clone();
        let result = copy_recursively(&folder_to_copy, &location_to_paste);
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_copy_recursively() -> io::Result<()> {
        let test_folder = create_testing_folder().unwrap();
        let folder_to_copy = test_folder.dir_list[2].clone();
        let location_to_paste = test_folder.dir_list[0]
            .clone()
            .join(folder_to_copy.file_name().unwrap());
        let result = copy_recursively(&folder_to_copy, &location_to_paste);
        assert!(result.is_ok());
        assert!(location_to_paste.exists());
        Ok(())
    }
}
