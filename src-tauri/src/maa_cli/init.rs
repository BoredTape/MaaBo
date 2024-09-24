use futures_util::StreamExt;
use std::cmp::min;
use std::fs::{self, File};
use std::io::Write;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

use crate::maa_cli::utils as maa_utils;
use crate::utils;
use crate::{consts as pconsts, version};

pub async fn deploy_latest(tx: Sender<(i32, String)>) -> Result<(), String> {
    let temp_dir = tempdir().or(Err(format!("创建零时目录失败")))?;

    let version_info = match version::get_maa_cli_remote_info() {
        Ok(vi) => vi,
        Err(error) => {
            tx.send((pconsts::SUCCESS, error.to_string()))
                .unwrap_or_else(|err| log::error!("{}", err.to_string()));
            panic!("{}", error.to_string());
        }
    };

    let file_name = version_info.name;
    let file_path = temp_dir.path().join(file_name);
    let mut file =
        File::create(&file_path).or(Err(format!("创建临时文件失败:{}", &file_path.display())))?;
    let client = reqwest::Client::new();
    let res = client.get(&version_info.url).send().await.or(Err(format!(
        "下载MAA CLI失败,下载地址:{}",
        &version_info.url
    )))?;
    let total_size = res.content_length().unwrap();

    tx.send((
        pconsts::SUCCESS,
        format!("从{}下载MAA CLI", &version_info.url).to_string(),
    ))
    .unwrap_or_else(|err| log::error!("{}", err.to_string()));

    thread::sleep(Duration::from_secs(2));
    tx.send((
        pconsts::SUCCESS,
        format!("下载MAA CLI总大小:{}", total_size).to_string(),
    ))
    .unwrap_or_else(|err| log::error!("{}", err.to_string()));
    thread::sleep(Duration::from_secs(2));

    let mut downloaded: u64 = 0;

    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;

        if total_size > 0 {
            tx.send((
                pconsts::SUCCESS,
                format!(
                    "MAA CLI下载进度: {:.2}%",
                    ((downloaded as f64) / (total_size as f64)) * f64::from(100)
                )
                .to_string(),
            ))
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        } else {
            tx.send((
                pconsts::SUCCESS,
                format!("MAA CLI已下载: {} Bytes", downloaded).to_string(),
            ))
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        }
    }

    tx.send((pconsts::SUCCESS, format!("下载完成,解压文件").to_string()))
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));

    utils::extract(
        file_path.to_str().unwrap(),
        temp_dir.path().to_str().unwrap(),
    );

    for entry in walkdir::WalkDir::new(temp_dir.path().to_str().unwrap()) {
        let entry = entry.unwrap();
        if entry
            .path()
            .to_str()
            .unwrap()
            .ends_with(maa_utils::name().as_str())
        {
            let dest = maa_utils::dir().join(maa_utils::name());

            if dest.exists() {
                let bak_path = maa_utils::dir().join(format!("{}.bak", maa_utils::name()));
                fs::remove_file(&bak_path).unwrap_or_else(|error| {
                    log::error!("{}", error.to_string());
                });
                fs::rename(&dest, &bak_path).unwrap_or_else(|error| {
                    log::error!("{}", error.to_string());
                });
            }
            fs::rename(entry.path(), dest).or(Err("无法移动maa_cli"))?;

            tx.send((pconsts::SUCCESS, format!("最新MAA CLI部署完成").to_string()))
                .unwrap_or_else(|err| log::error!("{}", err.to_string()));
            return Ok(());
        }
    }
    tx.send((pconsts::ERROR, "最新MAA CLI部署最后阶段失败".to_string()))
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
    return Err("最新MAA CLI部署最后阶段失败".to_string());
}

pub fn init_process(tx: Sender<(i32, String)>) {
    let cli_dir = maa_utils::dir();
    if !cli_dir.exists() {
        utils::make_dir_exist(&cli_dir).unwrap();
    }

    let cli_config_dir = maa_utils::config_dir();
    if !cli_config_dir.exists() {
        utils::make_dir_exist(&cli_config_dir).unwrap();
    }

    let task_dir = maa_utils::task_dir();
    if !task_dir.exists() {
        utils::make_dir_exist(&task_dir).unwrap();
    }

    let core_config_dir = maa_utils::core_config_dir();
    if !core_config_dir.exists() {
        utils::make_dir_exist(&core_config_dir).unwrap();
    }

    let cli_data_dir = maa_utils::data_dir();
    if !cli_data_dir.exists() {
        utils::make_dir_exist(&cli_data_dir).unwrap();
    }

    let cli_state_dir = maa_utils::state_dir();
    if !cli_state_dir.exists() {
        utils::make_dir_exist(&cli_state_dir).unwrap();
    }

    let cli_cache_dir = maa_utils::cache_dir();
    if !cli_cache_dir.exists() {
        utils::make_dir_exist(&cli_cache_dir).unwrap();
    }

    let cli_log_dir = maa_utils::log_dir();
    if !cli_log_dir.exists() {
        utils::make_dir_exist(&cli_log_dir).unwrap();
    }

    tx.send((pconsts::SUCCESS, "检查toml配置".to_string()))
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
    let cli_config = cli_config_dir.join(maa_utils::config_name());
    if !cli_config.exists() {
        maa_utils::deploy_cli_config();
    }

    let core_default_config = core_config_dir.join(maa_utils::core_default_config_name());
    if !core_default_config.exists() {
        maa_utils::deploy_core_default_config();
    }

    let task_default_config = task_dir.join(maa_utils::default_task_name());
    if !task_default_config.exists() {
        maa_utils::deploy_task_default_config();
    }

    if !cli_dir.join(maa_utils::name()).exists() {
        tx.send((
            pconsts::SUCCESS,
            "未初始化MAA组件,进入初始化流程".to_string(),
        ))
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));

        maa_utils::get_remote_version_info().unwrap_or_else(|error| {
            tx.send((
                pconsts::SUCCESS,
                format!("获取MAA CLI远程更新信息失败:\n{}", error.to_string()),
            ))
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
            panic!("获取MAA CLI远程更新信息失败:\n{}", error.to_string());
        });

        tauri::async_runtime::block_on(deploy_latest(tx.clone())).unwrap_or_else(|error| {
            tx.send((
                pconsts::SUCCESS,
                format!("部署MAA组件失败:\n{}", error.to_string()),
            ))
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
            panic!("部署MAA组件失败");
        });
    }

    let (maa_cli_version, _) = maa_utils::get_current_version().unwrap_or_else(|error| {
        tx.send((
            pconsts::SUCCESS,
            format!("获取MAA CLI本地版本失败:\n{}", error.to_string()),
        ))
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        panic!("获取MAA CLI本地版本失败");
    });

    version::set_maa_cli_current_version(&maa_cli_version).unwrap_or_else(|error| {
        tx.send((
            pconsts::SUCCESS,
            format!("设置MAA CLI本地版本失败:\n{}", error.to_string()),
        ))
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        panic!("设置MAA CLI本地版本失败");
    });
    let core_lib_dir = maa_utils::core_lib_dir();
    if !core_lib_dir.exists() || !core_lib_dir.join(maa_utils::core_lib_name()).exists() {
        tx.send((pconsts::SUCCESS, "安装MAA CORE".to_string()))
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        if let Err(error_msg) = maa_utils::install_maa_core() {
            log::error!("安装MAA CORE失败,错误:{}", error_msg.to_string());
            tx.send((pconsts::ERROR, error_msg.to_string()))
                .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        }
    }

    tx.send((pconsts::DONE, "自检完成".to_string()))
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));

    let mut maa_update_msg = (pconsts::SUCCESS, "更新资源成功".to_string());
    if let Err(error_msg) = maa_utils::maa_update() {
        log::error!("更新资源失败{}", error_msg.to_string());
        maa_update_msg = (pconsts::ERROR, error_msg.to_string());
    }
    tx.send(maa_update_msg)
        .unwrap_or_else(|err| log::error!("{}", err.to_string()));

    let mut maa_hot_update_msg = (pconsts::SUCCESS, "资源热更新成功".to_string());
    loop {
        if !version::get_maa_cli_ignore_update() {
            maa_utils::get_remote_version_info().unwrap_or_else(|error| {
                log::error!("获取MAA CLI远程更新信息失败:\n{}", error.to_string());
                tx.send((
                    pconsts::ERROR,
                    format!("获取MAA CLI远程更新信息失败:\n{}", error.to_string()),
                ))
                .unwrap_or_else(|err| log::error!("{}", err.to_string()));
            });
            let (need_update, update_msg) = version::maa_cli_need_update();
            if need_update {
                tx.send((
                    pconsts::MAA_CLI_NEED_UPDATE,
                    format!("MAA CLI更新: {}", update_msg),
                ))
                .unwrap_or_else(|err| log::error!("{}", err.to_string()));
            }
        }
        thread::sleep(Duration::from_secs(3600));
        if let Err(error_msg) = maa_utils::maa_hot_update() {
            log::error!("资源热更新失败,错误:{}", error_msg.to_string());
            maa_hot_update_msg = (
                pconsts::ERROR,
                format!("资源热更新失败,错误:{}", error_msg.to_string()),
            );
        }
        tx.send(maa_hot_update_msg)
            .unwrap_or_else(|err| log::error!("{}", err.to_string()));
        maa_hot_update_msg = (pconsts::SUCCESS, "资源热更新成功".to_string());
    }
}
