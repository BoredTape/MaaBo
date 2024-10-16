use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager, Result,
};
use tauri_plugin_notification::NotificationExt;

pub fn setup(app: &mut App) -> Result<()> {
    let quit_menu_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let visible_item = MenuItem::with_id(app, "visible", "最小化到托盘", true, None::<&str>)?;
    let visible_item_clone = visible_item.clone();
    let tips_menu_item = MenuItem::with_id(app, "tips", "给我一个提示", true, None::<&str>)?;

    let menus = Menu::with_items(app, &[&tips_menu_item, &visible_item, &quit_menu_item])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menus)
        .menu_on_left_click(false)
        .on_tray_icon_event(move |tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(()) = window.show() {
                        visible_item_clone
                            .set_text("最小化到托盘")
                            .unwrap_or_else(|err| log::info!("error:{}", err.to_string()));
                    }
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .on_menu_event(move |app_handle, event| match event.id.as_ref() {
            "quit" => {
                app_handle.exit(0);
            }
            "visible" => {
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window.is_visible().unwrap() {
                        if let Ok(()) = window.hide() {
                            visible_item
                                .set_text("显示")
                                .unwrap_or_else(|err| log::info!("error:{}", err.to_string()));
                        }
                    } else {
                        if let Ok(()) = window.show() {
                            visible_item
                                .set_text("最小化到托盘")
                                .unwrap_or_else(|err| log::info!("error:{}", err.to_string()));
                        }
                        let _ = window.set_focus();
                    }
                }
            }
            "tips" => {
                app_handle
                    .notification()
                    .builder()
                    .title("我没想过真的有人会按这个")
                    .body("真的")
                    .show()
                    .unwrap();
            }
            _ => {}
        })
        .build(app)?;
    Ok(())
}
