use tauri::Emitter;

use crate::events::{consts, payload};

#[tauri::command(async)]
pub fn init_process(handle: tauri::AppHandle) {
    let mut emit_tag = consts::EVENT_INIT_MSG.to_string();
    handle
        .emit(&emit_tag, payload::new_empty(1, "haha".to_string()))
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
}
