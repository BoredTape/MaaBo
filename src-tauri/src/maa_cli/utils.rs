use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Error, Read};
use std::process::{Child, Command};
use std::{env, path::PathBuf};

use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use tar::Archive;

use crate::errors::Error as CError;
use crate::maa_cli::consts;
use crate::utils::get_user_dir;
use crate::version;

pub fn name() -> String {
    return consts::MAA_CLI_NAME.to_string() + env::consts::EXE_SUFFIX;
}

pub fn core_lib_name() -> String {
    if std::env::consts::OS == "windows" {
        return consts::MAA_CORE_LIB_NAME_WINDOWS.to_string() + env::consts::DLL_SUFFIX;
    }
    return consts::MAA_CORE_LIB_NAME.to_string() + env::consts::DLL_SUFFIX;
}

pub fn config_name() -> String {
    consts::MAA_CLI_CONFIG_NAME.to_string()
}

pub fn dir() -> PathBuf {
    let mut p = get_user_dir().unwrap();
    p = p.join(consts::MAA_CLI_DIR);
    return p;
}

pub fn config_dir() -> PathBuf {
    dir().join(consts::MAA_CLI_CONFIG_DIR)
}

pub fn data_dir() -> PathBuf {
    dir().join(consts::MAA_CLI_DATA_DIR)
}

pub fn state_dir() -> PathBuf {
    dir().join(consts::MAA_CLI_STATE_DIR)
}

pub fn cache_dir() -> PathBuf {
    dir().join(consts::MAA_CLI_CACHE_DIR)
}

pub fn log_dir() -> PathBuf {
    dir().join(consts::MAA_CLI_LOG_DIR)
}

pub fn core_lib_dir() -> PathBuf {
    data_dir().join(consts::MAA_CLI_DATA_LIB_DIR)
}

pub fn task_dir() -> PathBuf {
    config_dir().join(consts::MAA_CLI_CONFIG_TASK_DIR)
}

pub fn core_config_dir() -> PathBuf {
    config_dir().join(consts::MAA_CORE_CONFIG_DIR)
}

pub fn core_default_config_name() -> String {
    consts::MAA_CORE_DEFAULT_CONFIG_NAME.to_string()
}

pub fn default_task_name() -> String {
    consts::MAA_CLI_TASK_DEFAULT_CONFIG_NAME.to_string()
}

pub fn extract(src: &str, dst: &str) {
    let file = std::fs::File::open(src).unwrap();
    if src.ends_with(".tar.gz") {
        Archive::new(GzDecoder::new(file)).unpack(dst).unwrap();
    } else if src.ends_with(".zip") {
        zip_extract::extract(file, &PathBuf::from(dst), true).unwrap();
    } else {
        panic!("文件 {} 无法解压", src);
    }
}

fn cli_command(clitent_name: &str, args: Vec<&str>) -> Result<(os_pipe::PipeReader, Child), Error> {
    let (reader, _, child) = cli_command_with_stdin(clitent_name, args)?;
    Ok((reader, child))
}

fn cli_command_with_stdin(
    clitent_name: &str,
    args: Vec<&str>,
) -> Result<(os_pipe::PipeReader, os_pipe::PipeWriter, Child), Error> {
    let (reader_stdout, writer_stdout) = os_pipe::pipe()?;
    let writer_stdout_clone = writer_stdout.try_clone()?;
    let mut command = Command::new(dir().join(name()).to_str().unwrap());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW constant
    }
    let mut state_path = state_dir();
    if !clitent_name.is_empty() {
        state_path = state_path.join(clitent_name);
    }
    command
        .env("MAA_CONFIG_DIR", config_dir().to_str().unwrap())
        .env("MAA_DATA_DIR", data_dir().to_str().unwrap())
        .env("MAA_STATE_DIR", state_path.to_str().unwrap())
        .env("MAA_CACHE_DIR", cache_dir().to_str().unwrap())
        .env("MAA_LOG", "info")
        .args(args);
    command.stdout(writer_stdout);
    command.stderr(writer_stdout_clone);
    let (reader_stdin, writer_stdin) = os_pipe::pipe()?;
    command.stdin(reader_stdin);
    let child = command.spawn()?;
    let mut client_name_format = String::new();
    if !clitent_name.is_empty() {
        client_name_format = format!("[{}]", clitent_name);
    }
    log::info!(
        "{}运行命令:{}",
        client_name_format,
        format!("{:?}", command).replace("\"", "")
    );
    drop(command);
    Ok((reader_stdout, writer_stdin, child))
}

fn command_result(command: Result<(os_pipe::PipeReader, Child), Error>) -> Result<String, CError> {
    if command.is_err() {
        return Err(CError::MaaCliRTError(command.err().unwrap().to_string()));
    }
    let (reader, _) = command.unwrap();
    let mut result = String::new();
    let mut is_error = false;
    for line in BufReader::new(reader).lines() {
        if line.is_err() {
            return Err(CError::MaaCliRTError("获取输出失败".to_string()));
        }
        let ch = line.unwrap();
        if ch.starts_with("Error:") {
            is_error = true;
            result.clear();
            result.push_str(&ch);
        }
    }
    if is_error {
        return Err(CError::MaaCliRTError(result));
    }
    Ok(result)
}

pub fn get_current_version() -> Result<(), CError> {
    let (mut reader, _) = cli_command(&"", vec!["version"]).unwrap();
    let mut cmd_text = String::new();
    if let Err(error) = reader.read_to_string(&mut cmd_text) {
        return Err(CError::MaaCliLocalVersionError(error.to_string()));
    }

    let text_array: Vec<&str> = cmd_text.split("\n").collect();
    if text_array.len() < 1 {
        return Err(CError::MaaCliLocalVersionError("".to_string()));
    }
    version::set_maa_cli_current_version(&text_array[0].replace("maa-cli v", ""))
}

#[derive(Debug, Serialize, Deserialize)]
struct CliConfigCli {
    api_url: String,
    channel: String,
    download_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CliConfigResourceRemote {
    branch: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CliConfigResource {
    auto_update: bool,
    remote: CliConfigResourceRemote,
}

#[derive(Debug, Serialize, Deserialize)]
struct CliConfig {
    cli: CliConfigCli,
    resource: CliConfigResource,
}

#[derive(Debug, Serialize, Deserialize)]
struct RemoteVersionDetailsAsset {
    name: String,
    size: u32,
    sha256sum: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RemoteVersionDetails {
    tag: String,
    assets: HashMap<String, RemoteVersionDetailsAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RemoteVersion {
    version: String,
    details: RemoteVersionDetails,
}

pub fn get_remote_version_info() -> Result<(), CError> {
    let cli_config = match fs::read_to_string(config_dir().join(config_name())) {
        Ok(cc) => cc,
        Err(error) => {
            return Err(CError::MaaCliRemoteVersionError(error.to_string()));
        }
    };

    let cli_config: CliConfig = match toml::from_str(&cli_config) {
        Ok(j) => j,
        Err(error) => {
            return Err(CError::MaaCliConfigReadFail(error.to_string()));
        }
    };

    let api_url = format!(
        "{}/{}.json",
        cli_config.cli.api_url.trim_end_matches("/"),
        cli_config.cli.channel.to_lowercase()
    );

    // so fucking ugly
    let version_info = match match reqwest::blocking::get(api_url) {
        Ok(resp) => resp,
        Err(error) => {
            return Err(CError::MaaCliRemoteVersionError(error.to_string()));
        }
    }
    .json::<RemoteVersion>()
    {
        Ok(v) => v,
        Err(error) => {
            return Err(CError::MaaCliRemoteVersionError(error.to_string()));
        }
    };

    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        _ => panic!("不支持的系统架构{}", std::env::consts::ARCH),
    };

    let asset_tag = match std::env::consts::OS {
        "macos" => format!("{}-apple-darwin", arch),
        "windows" => format!("{}-pc-windows-msvc", arch),
        "linux" => format!("{}-unknown-linux-gnu", arch),
        _ => panic!("不支持的系统类型{}", std::env::consts::OS.to_string()),
    };

    let asset = match version_info.details.assets.get(&asset_tag) {
        Some(res) => res,
        None => panic!("不支持的系统类型{}", asset_tag),
    };

    version::set_maa_cli_remote_info(
        &version_info.version,
        &asset.name,
        asset.size,
        &asset.sha256sum,
        &format!(
            "{}/{}/{}",
            cli_config.cli.download_url.trim_end_matches("/"),
            &version_info.details.tag,
            &asset.name
        ),
    )
}

// pub fn maa_cli_update() -> Result<String, CError> {
//     command_result(cli_command(&"", vec!["self", "update"]))
// }

pub fn install_maa_core() -> Result<String, CError> {
    command_result(cli_command(&"", vec!["install"]))
}

pub fn maa_update() -> Result<String, CError> {
    command_result(cli_command(&"", vec!["update"]))
}

pub fn maa_hot_update() -> Result<String, CError> {
    command_result(cli_command(&"", vec!["hot-update"]))
}

pub fn maa_one_key(config_name: &str) -> Result<(os_pipe::PipeReader, Child), Error> {
    cli_command(
        config_name,
        vec![
            "run",
            config_name,
            &format!("--profile={}.toml", config_name),
            // &format!("--log-file={}",log_dir().join(consts::MAA_CLI_LOG_FILE_NAME).to_str().unwrap())
        ],
    )
}

pub fn maa_copilot(
    config_name: &str,
    uri: &str,
) -> Result<(os_pipe::PipeReader, os_pipe::PipeWriter, Child), Error> {
    cli_command_with_stdin(
        config_name,
        vec![
            "copilot",
            uri,
            &format!("--profile={}.toml", config_name),
            // &format!("--log-file={}", log_dir().to_str().unwrap()),
        ],
    )
}

pub fn get_user_config_names() -> Vec<String> {
    let mut core_config_names = vec![consts::MAA_CORE_DEFAULT_CONFIG_NAME.to_string()];
    let mut user_configs = vec![consts::MAA_CLI_TASK_DEFAULT_CONFIG_NAME.to_string()];

    for entry in walkdir::WalkDir::new(core_config_dir()) {
        if let Ok(en) = entry {
            if en.path().is_file() {
                let file_name = String::from(en.file_name().to_string_lossy());
                if file_name != consts::MAA_CORE_DEFAULT_CONFIG_NAME {
                    core_config_names.push(file_name);
                }
            }
        }
    }

    for entry in walkdir::WalkDir::new(task_dir()) {
        if let Ok(en) = entry {
            if en.path().is_file() {
                let file_name = String::from(en.file_name().to_string_lossy());
                if file_name != consts::MAA_CLI_TASK_DEFAULT_CONFIG_NAME {
                    if core_config_names.contains(&file_name) {
                        user_configs.push(file_name);
                    }
                }
            }
        }
    }
    return user_configs;
}

pub fn deploy_cli_config() {
    let contents = r#"[cli]
channel = "Stable"
api_url = "https://github.com/MaaAssistantArknights/maa-cli/raw/version"
download_url = "https://github.com/MaaAssistantArknights/maa-cli/releases/download/"

[resource]
auto_update = false

[resource.remote]
branch = "main"
url = "https://github.com/MaaAssistantArknights/MaaResource.git""#;

    if fs::write(config_dir().join(config_name()).to_str().unwrap(), contents).is_err() {
        panic!("释放maa_cli配置文件失败...")
    }
}

pub fn deploy_task_default_config() {
    let contents = r#"[[tasks]]
name = "开始唤醒"
type = "StartUp"

  [tasks.params]
  enable = true
  client_type = "Official"
  start_game_enabled = true
  account_name = ""

[[tasks]]
name = "基建换班"
type = "Infrast"

  [tasks.params]
  enable = true
  mode = 0
  facility = [
  "Trade",
  "Reception",
  "Mfg",
  "Control",
  "Power",
  "Office",
  "Dorm"
]
  drones = "Money"
  threshold = 0.3
  replenish = false
  dorm_notstationed_enabled = true
  dorm_trust_enabled = true
  filename = ""
  plan_index = 0

[[tasks]]
name = "刷理智"
type = "Fight"

  [tasks.params]
  enable = true
  stage = ""
  medicine = 0
  expiring_medicine = 0
  stone = 0
  times = 1
  series = 1
  client_type = "Official"

[[tasks]]
type = "Mall"
name = "领取信用及商店购物"

  [tasks.params]
  enable = true
  shopping = true
  buy_first = [ "招聘许可" ]
  blacklist = [ "碳", "家具", "加急许可" ]
  force_shopping_if_credit_full = true
  only_buy_discount = false
  reserve_max_credit = false

[[tasks]]
name = "自动公招"
type = "Recruit"

  [tasks.params]
  enable = true
  refresh = true
  select = [ 4, 5 ]
  confirm = [ 3, 4, 5 ]
  first_tags = [ ]
  times = 0
  expedite = false
  skip_robot = false

    [tasks.params.recruitment_time]
    3 = 540
    4 = 540
    5 = 540
    6 = 540

[[tasks]]
name = "领取奖励"
type = "Award"

  [tasks.params]
  enable = true
  award = true
  mail = false
  recruit = false
  orundum = false
  mining = false
  specialaccess = false

[[tasks]]
name = "无限肉鸽"
type = "Roguelike"

  [tasks.params]
  enable = true
  theme = "Phantom"
  mode = 0
  starts_count = 2_147_483_647
  investment_enabled = true
  investments_count = 2_147_483_647
  stop_when_investment_full = false
  squad = "指挥分队"
  roles = "取长补短"
  core_char = ""
  start_with_elite_two = false
  only_start_with_elite_two = false
  use_support = false
  use_nonfriend_support = false
  refresh_trader_with_dice = false
  use_foldartal = false
  check_collapsal_paradigms = false
  double_check_collapsal_paradigms = false
  expected_collapsal_paradigms = [ "目空一些", "睁眼瞎", "图像损坏", "一抹黑" ]

[[tasks]]
name = "生息演算"
type = "ReclamationAlgorithm"

  [tasks.params]
  enable = true
  theme = 1
  mode = 0
  product = "荧光棒"
"#;
    if fs::write(
        task_dir().join(default_task_name()).to_str().unwrap(),
        contents,
    )
    .is_err()
    {
        panic!("释放任务配置文件失败...")
    }
}

pub fn deploy_core_default_config() {
    let contents = r#"[connection]
adb_path = "adb"
address = "emulator-5554"

[static_options]
cpu_ocr = true

[instance_options]
touch_mode = "MiniTouch"
deployment_with_pause = false
adb_lite_enabled = false
kill_adb_on_exit = false"#;
    if fs::write(
        core_config_dir()
            .join(core_default_config_name())
            .to_str()
            .unwrap(),
        contents,
    )
    .is_err()
    {
        panic!("释放MAA CORE配置文件失败...")
    }
}
