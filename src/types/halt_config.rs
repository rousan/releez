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
    pub task_id: u64,
    pub sub_task_id: u64,
    pub date: u128,
}

impl HaltConfig {
    pub fn parse(config_text: &str) -> crate::Result<HaltConfig> {
        serde_yaml::from_str::<HaltConfig>(config_text).wrap()
    }
}
