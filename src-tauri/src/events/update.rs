use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use tauri::{api::notification::Notification, Manager};

use crate::consts as pconst;
use crate::events::consts;
use crate::events::payload;
use crate::maa_cli;
use crate::status;
use crate::version;

#[tauri::command]
pub fn ignore_maa_cli_update() {
    version::set_maa_cli_ignore_update();
}

#[tauri::command(async)]
pub fn maa_cli_update_process(handle: tauri::AppHandle) {
    status::kill_all();
    let (tx, rx): (Sender<(i32, String)>, Receiver<(i32, String)>) = mpsc::channel();
    thread::spawn(move || {
        tauri::async_runtime::block_on(maa_cli::deploy_latest(tx.clone())).unwrap_or_else(
            |error| {
                tx.send((
                    pconst::ERROR,
                    format!("更新MAA CLI组件失败:\n{}", error.to_string()),
                ))
                .unwrap_or_else(|err| log::error!("{}", err.to_string()));
            },
        );
        tx.send((pconst::DONE, "更新MAA CLI组件成功".to_string()))
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
    });

    for msg in rx {
        handle
            .emit_all(
                consts::EVENT_UPDATE_MAA_CLI,
                payload::new_empty(msg.0, msg.1.clone()),
            )
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        if msg.0 == pconst::DONE {
            Notification::new(&handle.config().tauri.bundle.identifier)
                .title("MAA CLI更新成功")
                .body(&msg.1)
                .notify(&handle)
                .unwrap();
            break;
        }
    }
}
