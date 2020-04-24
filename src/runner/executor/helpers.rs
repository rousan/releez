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
use tokio::io::BufReader;

pub async fn calculate_start_task_ids(release_config: &ReleaseConfig, halt_config: Option<&HaltConfig>) -> (u64, u64) {
    let mut last_checked_task_ids: Option<(u64, u64)> = None;

    if let Some(halt_config) = halt_config {
        if halt_config.version == release_config.version
            && ask_user_to_continue_with_halt(halt_config).await.unwrap_or(false)
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

pub async fn ask_user_to_continue_with_halt(halt_config: &HaltConfig) -> crate::Result<bool> {
    out::print("\n").await?;

    out::print(format!(
        "{} Found {} file.\n",
        "✔".green(),
        constants::HALT_CONFIG_FILE_NAME.green()
    ))
    .await?;

    out::print(format!(
        "{} Want to resume the release? Last checked task: [{}, {}] (yes){}",
        "?".cyan(),
        halt_config.last_checked.task_id.to_string().as_str().cyan(),
        halt_config.last_checked.sub_task_id.to_string().as_str().cyan(),
        " › ".bright_black(),
    ))
    .await?;

    let mut line = out::read_line().await?;
    out::print("\n").await?;

    if line.is_empty() {
        line = "yes".to_owned();
    }

    Ok(constants::BOOL_POSSIBLE_TRUTHY_INPUTS.contains(&line.to_lowercase().as_str()))
}

pub fn gen_vars_data(release_version: &Version) -> HashMap<String, String> {
    let mut h = HashMap::new();
    h.insert(constants::VAR_NAME_VERSION.to_owned(), release_version.to_string());
    h
}

pub async fn print_reader_with_padding<R: AsyncRead + std::marker::Unpin>(
    r: &mut R,
    padding: &str,
    is_err: bool,
) -> crate::Result<()> {
    let r = BufReader::new(r);
    let mut lines = r.lines();

    while let Some(line) = lines.next_line().await.wrap()? {
        if is_err {
            out::print_err(format!("{}\n{}", line.trim(), padding)).await.wrap()?;
        } else {
            out::print(format!("{}\n{}", line.trim(), padding)).await.wrap()?;
        }
    }

    Ok(())
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
