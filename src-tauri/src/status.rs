use crate::consts;
use std::{
    collections::HashMap,
    process::Child,
    sync::{LazyLock, Mutex},
};

pub struct Status {
    pub status: u8,
    pub child: Option<Child>,
}

pub static MAA_STATUS: LazyLock<Mutex<HashMap<String, Status>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn get_maa_status(name: &str) -> u8 {
    let mut maa_status = MAA_STATUS.lock().unwrap();
    if maa_status.contains_key(name) {
        let status = maa_status.get_mut(name).unwrap();
        return status.status.clone();
    }
    consts::MAA_STATUS_STOP
}

pub fn set_maa_running(name: String, child: Option<Child>) {
    let mut maa_status = MAA_STATUS.lock().unwrap();
    maa_status.insert(
        name,
        Status {
            status: consts::MAA_STATUS_RUNNING,
            child,
        },
    );
}

pub fn maa_exit(name: &str) {
    let mut maa_status = MAA_STATUS.lock().unwrap();
    if maa_status.contains_key(name) {
        let status = maa_status.get_mut(name).unwrap();
        if let Some(mut c) = status.child.take() {
            c.kill().unwrap_or_else(|err| {
                log::error!("[{}] kill process error:{}", name, err.to_string());
            });
            log::info!("[{}] pid:{}, killed", &name, c.id());
        }
        status.child = None
    }
}

pub fn set_maa_stop(name: &str) {
    let mut maa_status = MAA_STATUS.lock().unwrap();
    if maa_status.contains_key(name) {
        let status = maa_status.get_mut(name).unwrap();
        status.status = consts::MAA_STATUS_STOP;
        status.child = None;
    } else {
        maa_status.insert(
            String::from(name),
            Status {
                status: consts::MAA_STATUS_STOP,
                child: None,
            },
        );
    }
}

pub fn kill_all() {
    let mut maa_status = MAA_STATUS.lock().unwrap();
    for (key, value) in maa_status.iter_mut() {
        if let Some(mut c) = value.child.take() {
            c.kill().unwrap_or_else(|err| {
                log::error!("[{}] kill process error:{}", key, err.to_string());
            });
            log::info!("[{}] pid:{}, killed", key, c.id());
        }
        log::info!("[{}] exit!", key);
    }
}
