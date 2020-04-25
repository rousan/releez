use crate::out;
use crate::prelude::*;
use crate::runner::executor::helpers;
use crate::types::{ReleaseConfig, Task};
use colored::*;
use std::collections::HashMap;
use std::env::consts::OS;
use std::path::Path;
use std::process::{ExitStatus, Stdio};
use tokio::process::Command;

pub async fn execute_auto_task(
    release_config: &ReleaseConfig,
    task_id: u64,
    task: &Task,
    start_sub_task_id: u64,
    root_dir: &Path,
    vars_data: &HashMap<String, String>,
) -> crate::Result<()> {
    out::print(format!("{} {}\n\n", "Running:".green().bold(), task.name.as_str())).await?;

    if let Some(ref confirm_msg) = task.confirm {
        let confirmation = helpers::ask_confirmation(confirm_msg, "yes", "").await?;

        if !confirmation {
            out::print("\nQuiting the process, run command again to resume the release.\n").await?;
            std::process::exit(0);
        }
        out::print("\n").await?;
    }

    let sub_tasks = task.sub_tasks().iter().skip(start_sub_task_id as usize);
    for (sub_task, idx) in sub_tasks.zip(start_sub_task_id..) {
        let command = sub_task.trim();

        out::print(format!("{} {}\n", "$".cyan(), command.cyan())).await?;

        let status = run_command(command, task, root_dir, vars_data).await?;

        if !status.success() {
            return Err(crate::Error::new(format!(
                "Failed to execute auto task command: '{}' with exit code: '{}' for task: '{}'",
                command,
                status.code().map(|code| code.to_string()).unwrap_or("na".to_owned()),
                task.name
            )));
        }

        helpers::save_last_checked(release_config, task_id, idx, root_dir).await?;
        out::print("\n").await?;
    }

    out::print(format!(
        "\n{}{}\n\n",
        "Checked: ".bright_black(),
        task.name.as_str().bright_black()
    ))
    .await?;

    Ok(())
}

async fn run_command(
    command: &str,
    task: &Task,
    root_dir: &Path,
    vars_data: &HashMap<String, String>,
) -> crate::Result<ExitStatus> {
    let (program, args) = if OS == "windows" {
        ("cmd", ["/C", command])
    } else {
        ("sh", ["-c", command])
    };

    let child = Command::new(program)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(root_dir)
        .envs(vars_data.iter())
        .kill_on_drop(true)
        .spawn()
        .context(format!(
            "Couldn't run the auto task command: '{}' for task: '{}', maybe the 'sh' or 'cmd' program not found on the system",
            command, task.name
        ))?;

    let status = child.await.wrap()?;

    Ok(status)
}
