use crate::types::{HaltConfig, ReleaseConfig, Task, TaskType};
use crate::{constants, ResultExt};
use semver::Version;
use std::fs;
use std::path::Path;

pub fn execute_checklist(
    release_config: &ReleaseConfig,
    halt_config: Option<&HaltConfig>,
    release_version: &Version,
    root_dir: &Path,
) -> crate::Result<()> {
    let (mut start_task_id, mut start_sub_task_id) = calculate_start_task_ids(release_config, halt_config);

    if start_task_id >= release_config.checklist.len() as u64 {
        start_task_id = 0;
        start_sub_task_id = 0;
    }

    for (idx, task) in release_config.checklist.iter().skip(start_task_id as usize).enumerate() {
        if idx != 0 {
            start_sub_task_id = 0;
        }

        match task.task_type {
            TaskType::Manual => execute_manual_task(task, start_sub_task_id, release_version, root_dir)?,
            TaskType::Auto => execute_auto_task(task, start_sub_task_id, release_version, root_dir)?,
        }
    }

    fs::remove_file(root_dir.join(constants::HALT_CONFIG_FILE_NAME)).context(format!(
        "Failed to remove the halt file: {}",
        constants::HALT_CONFIG_FILE_NAME
    ))?;

    Ok(())
}

fn calculate_start_task_ids(release_config: &ReleaseConfig, halt_config: Option<&HaltConfig>) -> (u64, u64) {
    let mut last_checked_task_ids: Option<(u64, u64)> = None;

    if let Some(halt_config) = halt_config {
        if halt_config.version == release_config.version && ask_user_to_continue_with_halt(halt_config) {
            last_checked_task_ids = Some((halt_config.last_checked.task_id, halt_config.last_checked.sub_task_id));
        }
    }

    let (last_checked_task_id, last_checked_sub_task_id) = match last_checked_task_ids {
        Some(ids) => ids,
        None => return (0, 0),
    };

    if let Some(last_checked_task) = release_config.checklist.get(last_checked_task_id as usize) {
        if (last_checked_sub_task_id + 1) < (last_checked_task.sub_tasks().len() as u64) {
            (last_checked_task_id, last_checked_sub_task_id + 1)
        } else {
            (last_checked_task_id + 1, 0)
        }
    } else {
        (0, 0)
    }
}

fn ask_user_to_continue_with_halt(halt_config: &HaltConfig) -> bool {
    // @todo: ask user whether to continue or not
    true
}

fn execute_manual_task(task: &Task, sub_task_id: u64, release_version: &Version, root_dir: &Path) -> crate::Result<()> {
    // @todo: create halt file whenever a sub task is complete, if exists, overwrite it including the version field.
    Ok(())
}

fn execute_auto_task(task: &Task, sub_task_id: u64, release_version: &Version, root_dir: &Path) -> crate::Result<()> {
    // @todo: create halt file whenever a sub task is complete, if exists, overwrite it including the version field.
    Ok(())
}
