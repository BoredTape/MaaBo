use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use serde_json;

mod release_0_2_1;
use crate::maa_cli;

pub fn execute() {
    let migration_file = maa_cli::dir().join("migration.json");
    let mut migration_info: serde_json::Value = serde_json::from_str("{}").unwrap();
    if migration_file.exists() {
        migration_info =
            serde_json::from_str(&fs::read_to_string(&migration_file).unwrap()).unwrap();
    }
    if !migration_info
        .get("0.2.1")
        .unwrap_or(&serde_json::Value::Bool(false))
        .as_bool()
        .unwrap_or(false)
    {
        release_0_2_1::migration_execute();
        migration_info["0.2.1"] = serde_json::Value::Bool(true)
    }

    let mut migration_write = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(&migration_file)
        .unwrap();
    migration_write
        .write_all(&migration_info.to_string().as_bytes())
        .unwrap()
}
