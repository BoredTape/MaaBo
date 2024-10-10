use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Result,
};

pub fn setup(app: &mut App) -> Result<()> {
    let quit_menu_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_menu_item])?;
    TrayIconBuilder::new()
        .menu(&menu)
        .menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {
                println!("menu item {:#?} not handled", event.id);
            }
        })
        .build(app)?;
    Ok(())
}
