pub const SUCCESS: i32 = 0;
pub const MAA_CLI_NEED_UPDATE: i32 = 50;
pub const DONE: i32 = 1;
pub const ERROR: i32 = -999;
pub const MAA_STATUS_STOP: u8 = 0;
pub const MAA_STATUS_RUNNING: u8 = 1;
pub const MAABO_ONLINE_VERSION_URL: &str =
    "https://api.github.com/repos/BoredTape/MaaBo/releases/latest";
pub const REQUEST_QA:&str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0";

pub const MAABO_DIR: &str = ".MaaBo";
pub const MAA_CORE_LIB_NAME: &str = "libMaaCore";
pub const MAA_CORE_LIB_NAME_WINDOWS: &str = "MaaCore";
pub const MAA_CORE_CONFIG_DIR: &str = "profiles";
pub const MAA_CORE_DEFAULT_CONFIG_NAME: &str = "default.toml";

pub const MAA_CLI_NAME: &str = "maa";
pub const MAA_CLI_CONFIG_NAME: &str = "cli.toml";
pub const MAA_CLI_CONFIG_DIR: &str = "config";
pub const MAA_CLI_DATA_DIR: &str = "data";
pub const MAA_CLI_DATA_LIB_DIR: &str = "lib";
pub const MAA_CLI_CONFIG_TASK_DIR: &str = "tasks";
pub const MAA_CLI_STATE_DIR: &str = "state";
pub const MAA_CLI_CACHE_DIR: &str = "cache";
pub const MAA_CLI_LOG_DIR: &str = "log";
pub const MAA_CLI_TASK_DEFAULT_CONFIG_NAME: &str = "default.toml";
