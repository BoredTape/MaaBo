// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod consts;
mod errors;
mod logger;
mod utils;

fn main() {
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

    let maabo_dir = utils::dir::maabo_dir();
    if !maabo_dir.exists() {
        utils::dir::make_dir_exist(&maabo_dir).unwrap();
    }
    logger::init_logger(&maabo_dir, consts::MAABO_LOGNAME);
    app_lib::run();
}
