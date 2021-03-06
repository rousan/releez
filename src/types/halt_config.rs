use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HaltConfig {
    pub version: String,
    pub last_checked: LastCheckedConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LastCheckedConfig {
    // Task ID starts with 0
    pub task_id: u64,
    // Sub Task ID starts with 0
    pub sub_task_id: u64,
    // timestamp in milliseconds
    pub date: i64,
}

impl HaltConfig {
    pub fn parse(config_text: &str) -> crate::Result<HaltConfig> {
        serde_yaml::from_str::<HaltConfig>(config_text).wrap()
    }
}
