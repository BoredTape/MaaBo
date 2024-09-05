use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
pub struct Payload<T: Clone + Serialize> {
    code: i32,
    msg: String,
    data: Option<T>,
    ts: u128,
}

pub fn new<T: Clone + Serialize>(code: i32, msg: String, data: T) -> Payload<T> {
    Payload {
        code,
        msg,
        data: Some(data),
        ts: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    }
}

pub fn new_empty(code: i32, msg: String) -> Payload<()> {
    Payload {
        code,
        msg,
        data: None,
        ts: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    }
}
