use std::{fs, path::PathBuf};

use directories;

use crate::consts;
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

pub fn fetch_online_version() -> String {
    let client = reqwest::blocking::Client::new();
    let request = client
        .request(reqwest::Method::GET, consts::MAABO_ONLINE_VERSION_URL)
        .header("User-Agent", consts::REQUEST_QA);
    match request.send() {
        Ok(resp) => match resp.json::<serde_json::Value>() {
            Ok(json_value) => json_value["tag_name"]
                .as_str()
                .unwrap_or("0.0.0")
                .to_string(),
            Err(error) => {
                log::info!("获取maabo在线版本失败(format json)\n{}", error.to_string());
                "0.0.0".to_string()
            }
        },
        Err(error) => {
            log::info!("获取maabo在线版本失败\n{}", error.to_string());
            "0.0.0".to_string()
        }
    }
}
