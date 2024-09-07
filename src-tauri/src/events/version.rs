use serde::Serialize;

use crate::consts as pconsts;
use crate::events::consts;
use crate::events::payload;
use crate::maa_cli;

#[derive(Debug, Clone, Serialize)]
pub struct VersionInfo {
    maabo: String,
    maa_cli: String,
    maa_core: String,
    tauri: String,
    webview: String,
}

#[tauri::command]
pub fn version_info() -> payload::Payload<VersionInfo> {
    let wv_version = tauri::webview_version().unwrap_or_else(|error| {
        log::info!("获取webview版本失败,请确认是否安装\n{}", error.to_string());
        "获取失败".to_string()
    });
    let (maa_cli_version, maa_core_version) =
        maa_cli::get_current_version().unwrap_or_else(|error| {
            log::info!("获取本地组件版本失败\n{}", error.to_string());
            return ("获取失败".to_string(), "获取失败".to_string());
        });
    let info = VersionInfo {
        maabo: env!("CARGO_PKG_VERSION").to_string(),
        maa_cli: maa_cli_version,
        maa_core: maa_core_version,
        tauri: tauri::VERSION.to_string(),
        webview: wv_version,
    };
    payload::new(pconsts::SUCCESS, "success".to_string(), info)
}

#[tauri::command]
pub fn maabo_online_version() -> payload::Payload<()> {
    let client = reqwest::blocking::Client::new();
    let request = client
        .request(reqwest::Method::GET, consts::MAABO_ONLINE_VERSION_URL)
        .header("User-Agent", consts::REQUEST_QA);
    let version_info = match request.send() {
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
    };
    println!("{}", &version_info);
    payload::new_empty(pconsts::SUCCESS, version_info)
}
