use std::{fs, path::PathBuf};

use directories;

use crate::errors::Error;

pub fn get_user_dir() -> Result<PathBuf, Error> {
    let dud = directories::UserDirs::new();
    if dud.is_none() {
        return Err(Error::InvalidGetUserDir);
    }
    Ok(dud.unwrap().home_dir().to_path_buf())
}

pub fn make_dir_exist(path: &PathBuf) -> Result<(), Error> {
    let result = fs::create_dir_all(path);
    if result.is_err() {
        return Err(Error::FailCreateDir(path.display().to_string()));
    }
    Ok(())
}
