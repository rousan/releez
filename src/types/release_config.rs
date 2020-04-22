use crate::prelude::*;
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
        serde_yaml::from_str::<ReleaseConfig>(config_text).wrap()
    }
}
