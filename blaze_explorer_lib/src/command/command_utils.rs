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

use crate::app::App;

///Obtain the backup directory name to be used for storing the data. This is based on the time of
///calling the func.
pub fn get_backup_dir(proj_dir: &ProjectDirs, create: bool) -> PathBuf {
    let mut backup_name = format!(
        "backup_{}",
        Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
    );
    backup_name += ".blzbkp";
    let path = proj_dir.cache_dir().join(backup_name);
    //create this directory
    if create {
        let _ = fs::create_dir_all(&path);
    }
    path
}

pub fn create_file_move_map(
    paths_to_copy: &Vec<PathBuf>,
    new_base: &Path,
) -> HashMap<PathBuf, PathBuf> {
    paths_to_copy
        .clone()
        .into_iter()
        .zip(join_paths(paths_to_copy.clone(), new_base))
        .collect::<HashMap<PathBuf, PathBuf>>()
}

/// Move the files from the source directory to the target directory according to the input HashMap
/// containing source and target address.
pub fn move_recursively_from_map(
    move_map: &HashMap<PathBuf, PathBuf>,
    should_delete: bool,
) -> io::Result<()> {
    for (source_path, target_path) in move_map.iter() {
        match copy_recursively(source_path, target_path) {
            Ok(_) => {
                if should_delete {
                    match remove_dir_or_file(source_path) {
                        Ok(_) => (),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(())
}

pub fn create_backup_map(proj_dir: &ProjectDirs, files: Vec<PathBuf>) -> HashMap<PathBuf, PathBuf> {
    files
        .into_iter()
        .map(|file_path| (file_path, get_backup_dir(&proj_dir, false)))
        .collect::<HashMap<PathBuf, PathBuf>>()
}
pub fn join_paths(path_list: Vec<PathBuf>, new_base: &Path) -> Vec<PathBuf> {
    path_list
        .iter()
        .map(|path| {
            let new_path = new_base.join(path.file_name().unwrap());
            new_path
        })
        .collect::<Vec<PathBuf>>()
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

pub fn remove_dir_or_file(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
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

    use crate::testing_utils::{
        TestingFolder, create_custom_testing_folder, create_testing_folder,
    };

    use super::*;
    #[test]
    fn test_get_backup_dir() {
        let app = App::new_with_name("test_app".to_string()).unwrap();
        let proj_dir = &app.project_dir;
        let backup_dir = get_backup_dir(&proj_dir, true);
        assert!(backup_dir.exists());
        let backup_dir = get_backup_dir(&proj_dir, false);
        assert!(!backup_dir.exists());
    }

    fn create_file_move_map_testing() -> (TestingFolder, TestingFolder, HashMap<PathBuf, PathBuf>) {
        let original_dir = create_custom_testing_folder(vec!["a.txt", "b/", "b/c.csv"]).unwrap();

        let new_dir = create_custom_testing_folder(vec!["d.txt"]).unwrap();
        let original_root_dir = original_dir.root_dir.path().to_path_buf();
        let to_move = vec![
            original_root_dir.join("a.txt"),
            original_root_dir.join("b/c.csv"),
        ];

        let file_move_map = create_file_move_map(&to_move, new_dir.root_dir.path());

        (original_dir, new_dir, file_move_map)
    }

    #[test]
    fn test_create_file_move_map() {
        let (original_dir, new_dir, file_move_map) = create_file_move_map_testing();
        let original_root_dir = original_dir.root_dir.path().to_path_buf();
        let expected_file_move_map = HashMap::from([
            (
                original_root_dir.join("a.txt"),
                new_dir.root_dir.path().join("a.txt"),
            ),
            (
                original_root_dir.join("b/c.csv"),
                new_dir.root_dir.path().join("c.csv"),
            ),
        ]);
        assert_eq!(file_move_map, expected_file_move_map);
    }

    #[test]
    pub fn test_move_recursively_from_map_delete() {
        let (original_dir, new_dir, file_move_map) = create_file_move_map_testing();
        move_recursively_from_map(&file_move_map, true).unwrap();
        for (original_path, new_path) in file_move_map {
            assert!(!original_path.exists());
            assert!(new_path.exists());
        }
    }
    #[test]
    pub fn test_move_recursively_from_map_delete_false() {
        let (original_dir, new_dir, file_move_map) = create_file_move_map_testing();
        move_recursively_from_map(&file_move_map, false).unwrap();
        for (original_path, new_path) in file_move_map {
            assert!(original_path.exists());
            assert!(new_path.exists());
        }
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
        let resulting_file_list = join_paths(file_list, &target_dir.path());
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
