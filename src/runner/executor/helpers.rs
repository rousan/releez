use crate::constants;
use crate::out;
use crate::prelude::*;
use crate::types::{HaltConfig, LastCheckedConfig, ReleaseConfig};
use chrono::offset::Local;
use colored::*;
use semver::Version;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

pub async fn calculate_start_task_ids(release_config: &ReleaseConfig, halt_config: Option<&HaltConfig>) -> (u64, u64) {
    let mut last_checked_task_ids: Option<(u64, u64)> = None;

    if let Some(halt_config) = halt_config {
        if halt_config.version == release_config.version
            && ask_user_to_continue_with_halt(release_config, halt_config)
                .await
                .unwrap_or(false)
        {
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

pub async fn ask_confirmation(message: &str, default_val: &str, padding: &str) -> crate::Result<bool> {
    out::print(format!(
        "{}{} {} ({}){}",
        padding,
        "?".cyan(),
        message.trim(),
        default_val,
        " › ".bright_black(),
    ))
    .await?;

    let mut line = out::read_line().await?;

    if line.is_empty() {
        line = default_val.to_owned();
    }

    Ok(constants::BOOL_POSSIBLE_TRUTHY_INPUTS.contains(&line.to_lowercase().as_str()))
}

pub async fn ask_user_to_continue_with_halt(
    release_config: &ReleaseConfig,
    halt_config: &HaltConfig,
) -> crate::Result<bool> {
    out::print("\n").await?;

    out::print(format!("Previous release was not completed properly.\n")).await?;

    let (last_checked_task_name, is_partial_completion) = release_config
        .checklist
        .get(halt_config.last_checked.task_id as usize)
        .map(|t| {
            (
                t.name.as_str(),
                halt_config.last_checked.sub_task_id < ((t.sub_tasks().len() as u64) - 1),
            )
        })
        .unwrap_or(("na", false));

    if is_partial_completion {
        out::print(format!(
            "Last partially checked task: '{}'.\n",
            last_checked_task_name.cyan()
        ))
        .await?;
    } else {
        out::print(format!("Last checked task: '{}'.\n", last_checked_task_name.cyan())).await?;
    }

    let confirmation = ask_confirmation("Do you want to resume the release?", "yes", "").await?;

    Ok(confirmation)
}

pub fn gen_vars_data(release_version: &Version) -> HashMap<String, String> {
    let mut h = HashMap::new();
    h.insert(constants::VAR_NAME_VERSION.to_owned(), release_version.to_string());
    h
}

pub async fn save_last_checked(
    release_config: &ReleaseConfig,
    last_checked_task_id: u64,
    last_checked_sub_task_id: u64,
    root_dir: &Path,
) -> crate::Result<()> {
    let halt_config = HaltConfig {
        version: release_config.version.clone(),
        last_checked: LastCheckedConfig {
            task_id: last_checked_task_id,
            sub_task_id: last_checked_sub_task_id,
            date: Local::now().timestamp_millis(),
        },
    };

    let data = serde_yaml::to_vec(&halt_config).context("Couldn't convert halt config to yaml")?;

    fs::write(root_dir.join(constants::HALT_CONFIG_FILE_NAME), data)
        .await
        .context(format!(
            "Couldn't write task's checked state to halt config file: {}",
            constants::HALT_CONFIG_FILE_NAME
        ))?;

    Ok(())
}
