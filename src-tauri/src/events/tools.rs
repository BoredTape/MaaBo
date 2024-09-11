use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};

use tauri::Manager;

use crate::maa_cli;
use crate::{consts, status};

use super::payload;

#[tauri::command(async)]
pub fn tools_execute(name: String, tab: String, data: toml::Value, handle: tauri::AppHandle) {
    let task_name = format!("{}_tools", name);
    let config = maa_cli::task_dir().join(format!("{}.toml", &task_name));

    let result = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(config);
    let emit_tag = format!("{}_tools_{}_handle", name, tab);
    if let Err(err) = result {
        log::error!("{}", err.to_string());
        handle
            .emit_all(
                &emit_tag,
                payload::new_empty(consts::ERROR, err.to_string()),
            )
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        return;
    }
    let mut file = result.unwrap();
    let data_str_result = toml::to_string(&data);
    if let Err(err) = data_str_result {
        log::error!("{}", err.to_string());
        handle
            .emit_all(
                &emit_tag,
                payload::new_empty(consts::ERROR, err.to_string()),
            )
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        return;
    }
    let data_str = data_str_result.unwrap();

    let write_result = file.write_all(data_str.as_bytes());
    if let Err(err) = write_result {
        log::error!("{}", err.to_string());
        handle
            .emit_all(
                &emit_tag,
                payload::new_empty(consts::ERROR, err.to_string()),
            )
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        return;
    }

    match maa_cli::tools_process(&name, &task_name) {
        Err(err) => {
            log::error!("{}", err.to_string());
            handle
                .emit_all(
                    &emit_tag,
                    payload::new_empty(consts::ERROR, format!("错误:{}", err.to_string())),
                )
                .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        }
        Ok(reader) => {
            for line in BufReader::new(reader).lines() {
                if line.is_err() {
                    break;
                }
                let msg = line.unwrap();
                if !msg.starts_with("Schema error:") && !msg.is_empty() {
                    log::info!("[{}]{}", &name, &msg);
                    handle
                        .emit_all(&emit_tag, payload::new_empty(consts::SUCCESS, msg))
                        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
                }
            }
        }
    }

    handle
        .emit_all(
            &emit_tag,
            payload::new_empty(consts::DONE, "已停止".to_string()),
        )
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
    status::set_maa_stop(&name);
}
