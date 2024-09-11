use std::fs::{self, OpenOptions};
use std::io::Write;

use serde::Serialize;

use crate::consts;
use crate::events::payload;
use crate::maa_cli;
use crate::status;

#[tauri::command]
pub fn get_cli_config() -> payload::Payload<serde_json::Value> {
    let config_str = fs::read_to_string(maa_cli::config_dir().join(maa_cli::config_name()))
        .unwrap_or_else(|err| panic!("打开MAA CLI配置文件失败:{}", err.to_string()));
    let json: serde_json::Value = toml::from_str(&config_str)
        .unwrap_or_else(|err| panic!("序列化MAA CLI配置文件失败:{}", err.to_string()));
    payload::new(consts::SUCCESS, "success".to_string(), json)
}

#[derive(Debug, Clone, Serialize)]
pub struct UserConfig {
    name: String,
    status: u8,
    core: serde_json::Value,
    tasks: serde_json::Value,
}

#[tauri::command]
pub fn get_user_configs() -> payload::Payload<Vec<UserConfig>> {
    let user_config_names = maa_cli::get_user_config_names();
    let mut user_configs = vec![];
    for user_config_name in user_config_names {
        let core_config_str =
            fs::read_to_string(maa_cli::core_config_dir().join(&user_config_name))
                .unwrap_or_else(|err| panic!("读取MAA Core配置失败:{}", err.to_string()));
        let core_json: serde_json::Value = toml::from_str(&core_config_str)
            .unwrap_or_else(|err| panic!("序列化MAA Core配置失败:{}", err.to_string()));

        let task_config_str = fs::read_to_string(maa_cli::task_dir().join(&user_config_name))
            .unwrap_or_else(|err| panic!("读取任务配置失败:{}", err.to_string()));
        let mut task_json: serde_json::Value = toml::from_str(&task_config_str)
            .unwrap_or_else(|err| panic!("序列化任务配置失败:{}", err.to_string()));

        status::set_maa_stop(&user_config_name.replace(".toml", ""));

        user_configs.push(UserConfig {
            name: user_config_name.replace(".toml", ""),
            status: consts::MAA_STATUS_STOP,
            core: core_json,
            tasks: task_json["tasks"].take(),
        })
    }
    return payload::new(consts::SUCCESS, "success".to_string(), user_configs);
}

#[tauri::command]
pub fn save_cli_config(data: toml::Value) -> payload::Payload<()> {
    let config = maa_cli::config_dir().join(maa_cli::config_name());

    let result = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(config);
    if let Err(err) = result {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    let mut file = result.unwrap();
    let data_str_result = toml::to_string(&data);
    if let Err(err) = data_str_result {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    let data_str = data_str_result.unwrap();

    let write_result = file.write_all(data_str.as_bytes());
    if let Err(err) = write_result {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    return payload::new_empty(consts::SUCCESS, "success".to_string());
}

#[tauri::command]
pub fn save_core_config(name: String, data: toml::Value) -> payload::Payload<()> {
    let config = maa_cli::core_config_dir().join(format!("{}.toml", name));

    let result = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(config);
    if let Err(err) = result {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    let mut file = result.unwrap();
    let data_str_result = toml::to_string(&data);
    if let Err(err) = data_str_result {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    let data_str = data_str_result.unwrap();

    let write_result = file.write_all(data_str.as_bytes());
    if let Err(err) = write_result {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    return payload::new_empty(consts::SUCCESS, "success".to_string());
}

#[tauri::command]
pub fn save_task_config(name: String, data: toml::Value) -> payload::Payload<()> {
    let config = maa_cli::task_dir().join(format!("{}.toml", name));

    let result = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(config);
    if let Err(err) = result {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    let mut file = result.unwrap();
    let data_str_result = toml::to_string(&data);
    if let Err(err) = data_str_result {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    let data_str = data_str_result.unwrap();

    let write_result = file.write_all(data_str.as_bytes());
    if let Err(err) = write_result {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    return payload::new_empty(consts::SUCCESS, "success".to_string());
}

#[tauri::command]
pub fn delete_user_config(name: String) -> payload::Payload<()> {
    let core_config = maa_cli::core_config_dir().join(format!("{}.toml", name));
    let task_config = maa_cli::task_dir().join(format!("{}.toml", name));

    if let Err(err) = fs::remove_file(core_config) {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    if let Err(err) = fs::remove_file(task_config) {
        return payload::new_empty(consts::ERROR, err.to_string());
    }
    return payload::new_empty(consts::SUCCESS, "success".to_string());
}

#[tauri::command]
pub fn get_item_index() -> payload::Payload<serde_json::Value> {
    let item_index_str = fs::read_to_string(
        maa_cli::data_dir()
            .join(&"resource")
            .join(&"item_index.json"),
    )
    .unwrap_or_else(|err| {
        panic!(
            "读取{}失败:{}",
            maa_cli::data_dir()
                .join(&"resource")
                .join(&"item_index.json")
                .to_str()
                .unwrap(),
            err.to_string()
        )
    });
    let data = serde_json::from_str(&item_index_str).unwrap_or_else(|err| {
        panic!(
            "读取{}失败:{}",
            maa_cli::data_dir()
                .join(&"resource")
                .join(&"item_index.json")
                .to_str()
                .unwrap(),
            err.to_string()
        )
    });
    payload::new(consts::SUCCESS, "success".to_string(), data)
}

#[tauri::command]
pub fn get_fight_stages() -> payload::Payload<serde_json::Value> {
    let fight_stages_str =
        fs::read_to_string(maa_cli::data_dir().join(&"resource").join(&"stages.json"))
            .unwrap_or_else(|err| {
                panic!(
                    "读取{}失败:{}",
                    maa_cli::data_dir()
                        .join(&"resource")
                        .join(&"stages.json")
                        .to_str()
                        .unwrap(),
                    err.to_string()
                )
            });
    let data = serde_json::from_str(&fight_stages_str).unwrap_or_else(|err| {
        panic!(
            "读取{}失败:{}",
            maa_cli::data_dir()
                .join(&"resource")
                .join(&"stages")
                .to_str()
                .unwrap(),
            err.to_string()
        )
    });
    payload::new(consts::SUCCESS, "success".to_string(), data)
}

#[tauri::command]
pub fn get_current_sidestory() -> payload::Payload<serde_json::Value> {
    let sidestory_stages_str = fs::read_to_string(
        maa_cli::data_dir()
            .join(&"MaaResource")
            .join(&"cache")
            .join(&"resource")
            .join(&"tasks.json"),
    )
    .unwrap_or_else(|err| {
        panic!(
            "读取{}失败:{}",
            maa_cli::data_dir()
                .join(&"MaaResource")
                .join(&"cache")
                .join(&"resource")
                .join(&"tasks.json")
                .to_str()
                .unwrap(),
            err.to_string()
        )
    });
    let data = serde_json::from_str(&sidestory_stages_str).unwrap_or_else(|err| {
        panic!(
            "读取{}失败:{}",
            maa_cli::data_dir()
                .join(&"MaaResource")
                .join(&"cache")
                .join(&"resource")
                .join(&"tasks.json")
                .to_str()
                .unwrap(),
            err.to_string()
        )
    });
    payload::new(consts::SUCCESS, "success".to_string(), data)
}

#[derive(Debug, Clone, Serialize)]
pub struct PathData {
    maabo_dir: String,
    maa_config_dir: String,
    maa_data_dir: String,
    maa_state_dir: String,
    maa_cache_dir: String,
}

#[tauri::command(async)]
pub fn get_path_info() -> payload::Payload<PathData> {
    payload::new(
        consts::SUCCESS,
        "success".to_string(),
        PathData {
            maabo_dir: String::from(maa_cli::dir().to_str().unwrap()),
            maa_config_dir: String::from(maa_cli::config_dir().to_str().unwrap()),
            maa_data_dir: String::from(maa_cli::data_dir().to_str().unwrap()),
            maa_state_dir: String::from(maa_cli::state_dir().to_str().unwrap()),
            maa_cache_dir: String::from(maa_cli::cache_dir().to_str().unwrap()),
        },
    )
}
