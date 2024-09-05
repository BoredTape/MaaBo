use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use tauri::Manager;

use crate::{
    consts,
    events::{consts as econst, payload},
    maa_cli,
};

#[tauri::command(async)]
pub fn init_process(handle: tauri::AppHandle) {
    let (tx, rx): (Sender<(i32, String)>, Receiver<(i32, String)>) = mpsc::channel();
    thread::spawn(move || maa_cli::init_process(tx));
    let mut emit_tag = econst::EVENT_INIT_MSG.to_string();

    for msg in rx {
        if msg.0 == consts::ERROR && emit_tag == econst::EVENT_INIT_MSG {
            handle
                .emit_all(
                    &emit_tag,
                    payload::new_empty(msg.0, format!("初始化失败,5秒后退出,错误信息:{}", msg.1)),
                )
                .unwrap_or_else(|err| log::error!("{}", err.to_string()));
            thread::sleep(Duration::from_secs(6));
            std::process::exit(0);
        }
        handle
            .emit_all(&emit_tag, payload::new_empty(msg.0, msg.1))
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        if msg.0 == consts::DONE {
            emit_tag = econst::EVENT_GLOBAL_NOTIFICATION.to_string()
        }
    }
}
