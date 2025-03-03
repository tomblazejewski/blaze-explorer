use clipboard_win::Getter;
use clipboard_win::Setter;
use clipboard_win::empty;
use clipboard_win::formats::FileList;
use clipboard_win::get_clipboard;
use tempdir::TempDir;

use clipboard_win::Clipboard;
use std::fs::create_dir_all;
use std::thread;
use std::time::Duration;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use chrono::offset;
use directories::ProjectDirs;

/// Move a directory recursively.
/// # Arguments
/// * `from` - The directory to me moved
/// * `to` - The directory where `from` is to be moved
pub fn move_recursively(from: &PathBuf, to: &Path) -> io::Result<()> {
    //if file, rename
    if !from.is_dir() {
        let dst_path = to;
        fs::rename(from, &dst_path)?;
        return Ok(());
    }
    // Create the destination directory, if it's a folder
    fs::create_dir_all(to)?;
    // Iterate over the directory entries
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        let src_path = entry.path();
        let dst_path = to.join(entry.file_name());

        // If the entry is a directory, call the function recursively
        if file_type.is_dir() {
            move_recursively(&src_path, &dst_path)?;
        } else {
            // If it's a file, copy it
            fs::rename(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

pub fn move_path(from: &PathBuf, to: &PathBuf) -> Result<(), std::io::Error> {
    //create the to directory
    fs::rename(from, to)?;
    Ok(())
}
/// Remove a directory recursively, while saving it to a specified backup_path
pub fn remove_with_backup(path: &PathBuf, backup_path: &Path) {
    //write contents to a file so this can be recovered later on
    move_recursively(path, backup_path).unwrap();
    //if path is a folder, it will remain unless deleted manually
    if path.is_dir() {
        remove_no_backup(path.to_path_buf()).unwrap();
    }
}
///Remove the file or folder specified at the path
pub fn remove_no_backup(path: PathBuf) -> io::Result<()> {
    match path.is_dir() {
        true => fs::remove_dir_all(path),
        false => fs::remove_file(path),
    }
}
///Obtain the backup directory name to be used for storing the data. This is based on the time of
///calling the func.
pub fn get_backup_dir() -> PathBuf {
    let mut backup_name = format!(
        "backup_{}",
        offset::Local::now().format("%d_%h_%Y_%H_%M_%S_%3f")
    );
    backup_name += ".blzbkp";
    let proj_dir = ProjectDirs::from("", "", "blaze_explorer").unwrap();
    proj_dir.cache_dir().join(backup_name)
}

/// Move a directory recursively.
/// # Arguments
/// * `from` - The directory to be renamed
/// * `to` - The new path the `from` should take
pub fn rename_recursively(first_path: PathBuf, second_path: PathBuf) -> io::Result<()> {
    // Create the destination directory, if it's a folder
    let is_folder_path = !second_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains(".");
    if is_folder_path {
        fs::create_dir_all(second_path.clone())?;
        //loop over the directories in the folder
        for entry in fs::read_dir(first_path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            //src_path - the new first_path
            //dst_path - the new second_path
            let src_path = entry.path();
            let dst_path = second_path.join(entry.file_name());

            // If the entry is a directory, call the function recursively
            if file_type.is_dir() {
                move_recursively(&src_path, &dst_path)?;
            } else {
                // If it's a file, copy it
                fs::rename(&src_path, &dst_path)?;
            }
        }
    } else {
        //rename immediately
        fs::rename(first_path.clone(), second_path.clone())?;
        return Ok(());
    }
    Ok(())
}
/// Remove the directory if it happens to be a folder. Do nothing otherwise.
pub fn remove_if_folder(path: PathBuf) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?
    }
    Ok(())
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

pub fn move_from_clipboard(file_paths: Vec<PathBuf>, destination_path: PathBuf) -> io::Result<()> {
    for path in file_paths {
        let target_path = destination_path.join(path.file_name().unwrap());
        fs::copy(path, target_path)?;
    }
    Ok(())
}
fn copy_dir_all(
    src: PathBuf,
    dst: impl AsRef<Path>,
    paths_list: &mut Vec<PathBuf>,
) -> io::Result<()> {
    create_dir_all(&dst)?;
    let file_name = src.file_name().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Could not get the file name",
    ))?;
    let resulting_path = dst.as_ref().join(file_name);
    if src.is_file() {
        fs::copy(src, resulting_path.clone())?;
        paths_list.push(resulting_path);
        return Ok(());
    };
    //if src is a folder
    let new_path = dst.as_ref().join(file_name);
    fs::create_dir_all(&new_path)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        copy_dir_all(entry.path(), &new_path, paths_list)?;
    }
    Ok(())
}

/// Copy all the directories to a specified backup_path
/// Returns a result with list of filepaths created if successful
pub fn copy_to_backup(
    files: Vec<PathBuf>,
    backup_path: PathBuf,
) -> Result<Vec<PathBuf>, io::Error> {
    let mut path_list = Vec::new();
    files
        .iter()
        .map(|path| copy_dir_all(path.to_owned(), backup_path.clone(), &mut path_list))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(path_list)
}

mod tests {
    use color_eyre::owo_colors::OwoColorize;
    use fs::create_dir_all;

    use super::*;
    use std::{
        fs::File,
        io::Write,
        thread::{self, Thread},
        time::Duration,
    };
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
    fn test_copy_dir_all() -> io::Result<()> {
        let backup_dir = TempDir::new("backup_dir").unwrap();
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
        let folder_in_backup = backup_dir
            .path()
            .join(original_dir.as_ref().file_name().unwrap());
        let expected_file_list = vec![
            folder_in_backup.join("file1.txt"),
            folder_in_backup.join("file2.txt"),
            folder_in_backup.join("folder_1/file2.txt"),
            folder_in_backup.join("folder_1/folder_2/file3.txt"),
        ];
        let mut resulting_file_list = Vec::new();
        copy_dir_all(
            original_dir.path().to_path_buf(),
            &backup_dir,
            &mut resulting_file_list,
        )
        .unwrap();
        assert_eq!(resulting_file_list, expected_file_list);
        //assert files exist
        for file in resulting_file_list.iter() {
            assert!(file.exists());
        }
        Ok(())
    }
    #[test]
    fn test_copy_to_backup() -> io::Result<()> {
        let backup_dir = TempDir::new("backup_dir").unwrap();
        let original_dir = TempDir::new("original_directory").unwrap();
        let file_list = vec![
            original_dir.path().join("file1.txt"),
            original_dir.path().join("file2.txt"),
        ];
        for file in &file_list {
            let mut f = File::create(file)?;
            f.write_all(b"Hello, world!")?;
            f.sync_all()?;
        }
        let resulting_file_list =
            copy_to_backup(file_list.clone(), backup_dir.path().to_path_buf()).unwrap();
        let expected_file_list = file_list
            .iter()
            .map(|file| backup_dir.path().join(file.file_name().unwrap()))
            .collect::<Vec<PathBuf>>();
        for file in resulting_file_list.iter() {
            assert!(file.exists());
        }
        assert_eq!(resulting_file_list, expected_file_list);
        Ok(())
    }
}
