use serde::Serialize;

use crate::consts;
use crate::events::payload;
use crate::maa_cli;
use crate::utils;

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
    payload::new(consts::SUCCESS, "success".to_string(), info)
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckUpdateData {
    require_update: bool,
    from: String,
    to: String,
}

#[tauri::command(async)]
pub fn check_update() -> payload::Payload<CheckUpdateData> {
    let online_version = semver::Version::parse(&utils::fetch_online_version())
        .unwrap_or_else(|_| semver::Version::parse(&"0.0.0").unwrap());
    let current_version = semver::Version::parse(env!("CARGO_PKG_VERSION"))
        .unwrap_or_else(|_| semver::Version::parse(&"0.0.0").unwrap());

    let resp_data = CheckUpdateData {
        require_update: online_version > current_version,
        from: current_version.to_string(),
        to: online_version.to_string(),
    };
    payload::new(consts::SUCCESS, "success".to_string(), resp_data)
}
