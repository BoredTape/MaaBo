use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use serde::Serialize;

use crate::maa_cli;

#[derive(Serialize)]
struct ReclamationParams {
    enable: bool,
    theme: String,
    mode: u64,
    tool_to_craft: String,
    increment_mode: u64,
    num_craft_batches: u64,
}

#[derive(Serialize)]
struct Reclamation {
    name: String,
    #[serde(rename = "type")]
    task_type: String,
    params: ReclamationParams,
}

fn action(config_name: &str) {
    let config_path = maa_cli::task_dir().join(config_name);
    if let Ok(config_str) = fs::read_to_string(&config_path) {
        let mut new_config_json: serde_json::Value =
            serde_json::from_str(r#"{"tasks":[]}"#).unwrap();
        let mut new_tasks = Vec::new();
        if let Ok(config_json) = toml::from_str::<serde_json::Value>(&config_str) {
            for task in config_json
                .get("tasks")
                .unwrap_or(&serde_json::Value::Array(vec![]))
                .as_array()
                .unwrap_or(&vec![])
            {
                if let Some(task_type) = task.get("type") {
                    if task_type.as_str().unwrap() == "ReclamationAlgorithm" {
                        let params_obj = task.get("params").unwrap().as_object().unwrap();
                        let theme_int = params_obj.get("theme").unwrap().as_u64().unwrap_or(0);
                        let mut theme = String::from("Fire");
                        if theme_int == 1 {
                            theme = String::from("Tales");
                        }
                        let reclamation = Reclamation {
                            name: task.get("name").unwrap().as_str().unwrap().to_string(),
                            task_type: "Reclamation".to_string(),
                            params: ReclamationParams {
                                enable: params_obj.get("enable").unwrap().as_bool().unwrap(),
                                theme,
                                mode: params_obj.get("mode").unwrap().as_u64().unwrap(),
                                tool_to_craft: params_obj
                                    .get("product")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                                increment_mode: 0,
                                num_craft_batches: 16,
                            },
                        };
                        new_tasks.push(serde_json::to_value(reclamation).unwrap());
                    } else {
                        new_tasks.push(task.clone());
                    }
                }
            }
            new_config_json["tasks"] = serde_json::Value::Array(new_tasks);

            let mut result_config_file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(false)
                .truncate(true)
                .open(&config_path)
                .unwrap();
            result_config_file
                .write_all(toml::to_string(&new_config_json).unwrap().as_bytes())
                .unwrap()
        }
    }
}

pub fn migration_execute() {
    let config_names = maa_cli::get_user_config_names();
    for config_name in config_names {
        action(&config_name)
    }
}
