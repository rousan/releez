use crate::constants;
use crate::out;
use crate::runner::executor::helpers;
use crate::types::{ReleaseConfig, Task};
use crate::utils::var_subs::substitute_variable_in_text;
use colored::*;
use std::collections::HashMap;
use std::path::Path;

pub async fn execute_manual_task(
    release_config: &ReleaseConfig,
    task_id: u64,
    task: &Task,
    start_sub_task_id: u64,
    root_dir: &Path,
    vars_data: &HashMap<String, String>,
) -> crate::Result<()> {
    out::print(format!("{} {}\n\n", "✔ Checked:".green().bold(), task.name)).await?;

    let sub_tasks = task.sub_tasks().iter().skip(start_sub_task_id as usize);
    for (sub_task, idx) in sub_tasks.zip(start_sub_task_id..) {
        let sub_task = substitute_variable_in_text(sub_task.as_str(), vars_data);
        let instruction = sub_task.trim();

        out::print(format!(
            "   {} {} {}",
            "✔".green().bold(),
            instruction.cyan(),
            "[Press Enter if done or q to quit]".bright_black()
        ))
        .await?;

        let line = out::read_line().await?;
        if line.to_lowercase() == constants::MANUAL_TASK_QUIT_COMMAND {
            std::process::exit(0);
        }

        helpers::save_last_checked(release_config, task_id, idx, root_dir).await?;

        out::print("\n").await?;
    }

    out::print("\n").await?;

    Ok(())
}
