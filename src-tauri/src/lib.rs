use tauri::Manager;

mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let system_info = os_info::get();
    log::info!("OS information: {}", system_info);
    log::info!("MaaBo version: {}", env!("CARGO_PKG_VERSION"));
    log::info!("tauri version: {}", tauri::VERSION);
    log::info!(
        "webview version: {}",
        tauri::webview_version().unwrap_or_else(|error| {
            log::info!(
                "get webview version fail, make sure it is installed.\n{}",
                error.to_string()
            );
            panic!(
                "get webview version fail,, make sure it is installed.\n{}",
                error.to_string()
            );
        })
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                // # I do not want to use tauri log plugin,I am using log4rs instead
                // app.handle().plugin(
                //     tauri_plugin_log::Builder::default()
                //         .level(log::LevelFilter::Info)
                //         .build(),
                // )?;
                app.get_webview_window("main").unwrap().open_devtools();
            }
            tray::setup(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
