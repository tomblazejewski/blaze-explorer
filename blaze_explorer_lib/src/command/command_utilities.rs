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
