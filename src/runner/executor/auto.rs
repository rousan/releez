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
    out::print(format!("{} {}\n\n", "✔ Checked:".green().bold(), task.name)).await?;

    // @todo: check if confirm attribute is given.

    let sub_tasks = task.sub_tasks().iter().skip(start_sub_task_id as usize);
    for (sub_task, idx) in sub_tasks.zip(start_sub_task_id..) {
        let command = sub_task.trim();

        out::print(format!(
            "   {} {} {}\n     ",
            "✔".green().bold(),
            "$".bold(),
            command.cyan()
        ))
        .await?;

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

    out::print("\n").await?;

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

    let mut child = Command::new(program)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(root_dir)
        .envs(vars_data.iter())
        .kill_on_drop(true)
        .spawn()
        .context(format!(
            "Couldn't run the auto task command: '{}' for task: '{}', maybe the 'sh' or 'cmd' program not found on the system",
            command, task.name
        ))?;

    let stdout_child = child.stdout.take();
    let stderr_child = child.stderr.take();

    let stdout_task_handle = tokio::spawn(async move {
        if let Some(mut stdout_child) = stdout_child {
            helpers::print_reader_with_padding(&mut stdout_child, "     ").await
        } else {
            crate::Result::Ok(())
        }
    });

    let stderr_task_handle = tokio::spawn(async move {
        if let Some(mut stderr_child) = stderr_child {
            helpers::print_reader_with_padding(&mut stderr_child, "     ").await
        } else {
            crate::Result::Ok(())
        }
    });

    let (stdout_res, stderr_res) = tokio::try_join!(stdout_task_handle, stderr_task_handle).wrap()?;

    stdout_res.context("Couldn't add padding to child process's stdout")?;
    stderr_res.context("Couldn't add padding to child process's stderr")?;

    let status = child.await.wrap()?;

    Ok(status)
}
