use serde::Serialize;
use std::io::Write;
use std::io::{BufRead, BufReader};
use tauri::api::notification::Notification;

use tauri::Manager;

use crate::consts;
use crate::events::consts as econsts;
use crate::events::payload;
use crate::maa_cli;
use crate::status;

#[derive(Debug, Clone, Serialize)]
struct ConfigStatus {
    name: String,
    status: u8,
}

#[tauri::command(async)]
pub fn one_key(name: String, handle: tauri::AppHandle) {
    handle
        .emit_all(
            econsts::EVENT_UPDATE_CONFIG_STATUS,
            payload::new(
                consts::SUCCESS,
                "success".to_string(),
                ConfigStatus {
                    name: name.clone(),
                    status: consts::MAA_STATUS_RUNNING,
                },
            ),
        )
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));

    let emit_tag = format!("{}_one_key_handle", name);
    match maa_cli::one_key_process(&name) {
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
            payload::new_empty(consts::SUCCESS, "已停止".to_string()),
        )
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
    status::set_maa_stop(&name);

    Notification::new(&handle.config().tauri.bundle.identifier)
        .title("MaaBo通知")
        .body(&format!("{} 已停止", &name))
        .notify(&handle)
        .unwrap();

    handle
        .emit_all(
            econsts::EVENT_UPDATE_CONFIG_STATUS,
            payload::new(
                consts::SUCCESS,
                "success".to_string(),
                ConfigStatus {
                    name,
                    status: consts::MAA_STATUS_STOP,
                },
            ),
        )
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
}

#[tauri::command(async, rename_all = "snake_case")]
pub fn copilot(name: String, uri: String, auto_formation: bool, handle: tauri::AppHandle) {
    handle
        .emit_all(
            econsts::EVENT_UPDATE_CONFIG_STATUS,
            payload::new(
                consts::SUCCESS,
                "success".to_string(),
                ConfigStatus {
                    name: name.clone(),
                    status: consts::MAA_STATUS_RUNNING,
                },
            ),
        )
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));

    let emit_tag = format!("{}_copilot_handle", name);
    match maa_cli::copilot_process(&name, &uri) {
        Err(err) => {
            log::error!("{}", err.to_string());
            handle
                .emit_all(
                    &emit_tag,
                    payload::new_empty(consts::ERROR, format!("错误:{}", err.to_string())),
                )
                .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        }
        Ok(r) => match r {
            Some((reader, mut writer)) => {
                for line in BufReader::new(reader).lines() {
                    if line.is_err() {
                        break;
                    }
                    let msg = line.unwrap();
                    if msg.starts_with("Operators:") {
                        if auto_formation {
                            writer.write_all(b"Y\n").unwrap();
                        } else {
                            writer.write_all(b"n\n").unwrap();
                        }
                    }
                    if msg.starts_with("Whether to auto formation [Y/n]: ") {
                        let msg2 = msg.replace("Whether to auto formation [Y/n]: ", "");
                        log::info!("[{}]{}", &name, &msg2);
                        handle
                            .emit_all(&emit_tag, payload::new_empty(consts::SUCCESS, msg2))
                            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
                        continue;
                    }
                    if !msg.starts_with("Schema error:") && !msg.is_empty() {
                        log::info!("[{}]{}", &name, &msg);
                        handle
                            .emit_all(&emit_tag, payload::new_empty(consts::SUCCESS, msg))
                            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
                    }
                }
            }
            None => {}
        },
    }

    handle
        .emit_all(
            &emit_tag,
            payload::new_empty(consts::SUCCESS, "已停止".to_string()),
        )
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
    status::set_maa_stop(&name);

    Notification::new(&handle.config().tauri.bundle.identifier)
        .title("MaaBo通知")
        .body(&format!("{} 已停止", &name))
        .notify(&handle)
        .unwrap();

    handle
        .emit_all(
            econsts::EVENT_UPDATE_CONFIG_STATUS,
            payload::new(
                consts::SUCCESS,
                "success".to_string(),
                ConfigStatus {
                    name,
                    status: consts::MAA_STATUS_STOP,
                },
            ),
        )
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
}

#[tauri::command(async)]
pub fn stop(name: String) -> payload::Payload<()> {
    status::maa_exit(&name);
    payload::new_empty(consts::SUCCESS, "已停止".to_string())
}
