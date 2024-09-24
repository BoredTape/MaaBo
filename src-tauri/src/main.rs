// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod consts;
mod errors;
mod events;
mod logger;
mod maa_cli;
mod migration;
mod status;
mod utils;
mod version;

use tauri::{
    api::notification::Notification, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};

use events::{
    check_update, copilot, get_cli_config, get_current_sidestory, get_fight_stages, get_item_index,
    get_path_info, ignore_maa_cli_update, init_process, maa_cli_update_process, one_key,
    save_cli_config, stop, tools_execute, version_info,
};

fn main() {
    // 对配置项迁移，临时解决方案，后面可能会改
    migration::execute();

    // 临时解决方案-start
    // n卡会白屏，WebKit新版的渲染器与nvidia驱动暂时还不兼容导致
    // 使用env WEBKIT_DISABLE_DMABUF_RENDERER=1可以退回旧版渲染器解决问题
    // 比env WEBKIT_DISABLE_COMPOSITING_MODE=1直接禁用硬件加速好
    // 感谢 @Darkatse
    // 详情：https://github.com/BoredTape/MaaBo/pull/1#issuecomment-2333454094
    if std::env::consts::OS == "linux" {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
    // 临时解决方案-end

    let maa_bo_dir = maa_cli::dir();
    if !maa_bo_dir.exists() {
        utils::make_dir_exist(&maa_bo_dir).unwrap();
    }
    logger::init_logger(&maa_bo_dir, &"maabo");
    log::info!("MaaBo 版本:{}", env!("CARGO_PKG_VERSION"));
    log::info!("tauri 版本:{}", tauri::VERSION);
    log::info!(
        "webview 版本:{}",
        tauri::webview_version().unwrap_or_else(|error| {
            log::info!("获取webview版本失败,请确认是否安装\n{}", error.to_string());
            panic!("获取webview版本失败,请确认是否安装\n{}", error.to_string());
        })
    );

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("tips".to_string(), "给我一个提示"))
        .add_item(CustomMenuItem::new("visible".to_string(), "最小化到托盘"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "退出"));

    let tray = SystemTray::new().with_menu(tray_menu);

    let app = tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init_process,
            get_cli_config,
            save_cli_config,
            one_key,
            stop,
            ignore_maa_cli_update,
            maa_cli_update_process,
            get_item_index,
            get_fight_stages,
            get_current_sidestory,
            copilot,
            version_info,
            check_update,
            tools_execute,
            get_path_info
        ])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            // SystemTrayEvent::LeftClick {
            //     position: _,
            //     size: _,
            //     ..
            // } => {
            //     println!("system tray received a left click");
            // }
            // SystemTrayEvent::RightClick {
            //     position: _,
            //     size: _,
            //     ..
            // } => {
            //     println!("system tray received a right click");
            // }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                if !window.is_visible().unwrap() {
                    window.show().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    status::kill_all();
                    std::process::exit(0);
                }
                "visible" => {
                    let item_handle = app.tray_handle().get_item(&id);

                    let window = app.get_window("main").unwrap();
                    if window.is_visible().unwrap() {
                        item_handle.set_title("显示").unwrap();
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        item_handle.set_title("最小化到托盘").unwrap();
                    }
                }
                "tips" => {
                    let handle = app.app_handle();
                    Notification::new(&handle.config().tauri.bundle.identifier)
                        .title("我没想过真的有人会按这个")
                        .body("真的")
                        .notify(&handle)
                        .unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { .. } => {
            status::kill_all();
        }
        _ => {}
    });
}
