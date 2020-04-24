use crate::prelude::*;
use crate::utils::system_val_resolver::resolve_system_dependent_value_config;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseConfig {
    pub version: String,
    pub checklist: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub name: String,
    #[serde(rename = "type")]
    pub task_type: TaskType,
    pub instructions: Option<Value>,
    pub run: Option<Value>,
    pub confirm: Option<String>,
    #[serde(skip)]
    sub_tasks: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum TaskType {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "auto")]
    Auto,
}

impl ReleaseConfig {
    pub fn parse(config_text: &str) -> crate::Result<ReleaseConfig> {
        let mut config = serde_yaml::from_str::<ReleaseConfig>(config_text).wrap()?;

        for task in config.checklist.iter_mut() {
            let val = match task.task_type {
                TaskType::Auto => task.run.as_ref(),
                TaskType::Manual => task.instructions.as_ref(),
            };

            if let Some(val) = val {
                task.sub_tasks = resolve_system_dependent_value_config(val);
            } else {
                task.sub_tasks = Vec::new();
            }
        }

        Ok(config)
    }
}

impl Task {
    pub fn sub_tasks(&self) -> &Vec<String> {
        &self.sub_tasks
    }
}
