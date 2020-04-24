use crate::constants;
use crate::out;
use crate::prelude::*;
use crate::types::{HaltConfig, ReleaseConfig, TaskType};
use semver::Version;
use std::path::Path;
use tokio::fs;

mod auto;
mod helpers;
mod manual;

pub async fn execute_checklist(
    release_config: &ReleaseConfig,
    halt_config: Option<&HaltConfig>,
    release_version: &Version,
    root_dir: &Path,
) -> crate::Result<()> {
    let vars_data = helpers::gen_vars_data(release_version);

    let (mut start_task_id, mut start_sub_task_id) =
        helpers::calculate_start_task_ids(release_config, halt_config).await;

    if start_task_id >= release_config.checklist.len() as u64 {
        start_task_id = 0;
        start_sub_task_id = 0;
    }

    out::print("\n").await?;

    for (idx, task) in release_config.checklist.iter().skip(start_task_id as usize).enumerate() {
        if idx != 0 {
            start_sub_task_id = 0;
        }

        match task.task_type {
            TaskType::Manual => {
                manual::execute_manual_task(
                    release_config,
                    idx as u64 + start_task_id,
                    task,
                    start_sub_task_id,
                    root_dir,
                    &vars_data,
                )
                .await?
            }
            TaskType::Auto => {
                auto::execute_auto_task(
                    release_config,
                    idx as u64 + start_task_id,
                    task,
                    start_sub_task_id,
                    root_dir,
                    &vars_data,
                )
                .await?
            }
        }
    }

    fs::remove_file(root_dir.join(constants::HALT_CONFIG_FILE_NAME))
        .await
        .context(format!(
            "Failed to remove the halt file: {}",
            constants::HALT_CONFIG_FILE_NAME
        ))?;

    Ok(())
}
