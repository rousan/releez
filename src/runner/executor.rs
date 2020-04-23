use crate::constants;
use crate::prelude::*;
use crate::types::{HaltConfig, LastCheckedConfig, ReleaseConfig, Task, TaskType};
use crate::utils::var_subs::substitute_variable_in_text;
use chrono::offset::Local;
use colored::*;
use crossterm::{cursor, ExecutableCommand};
use semver::Version;
use std::collections::HashMap;
use std::env::consts::OS;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn execute_checklist(
    release_config: &ReleaseConfig,
    halt_config: Option<&HaltConfig>,
    release_version: &Version,
    root_dir: &Path,
) -> crate::Result<()> {
    let vars_data = gen_vars_data(release_version);

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
            TaskType::Manual => execute_manual_task(
                release_config,
                idx as u64 + start_task_id,
                task,
                start_sub_task_id,
                root_dir,
                &vars_data,
            )?,
            TaskType::Auto => execute_auto_task(
                release_config,
                idx as u64 + start_task_id,
                task,
                start_sub_task_id,
                root_dir,
                &vars_data,
            )?,
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
        if halt_config.version == release_config.version && ask_user_to_continue_with_halt(halt_config).unwrap_or(false)
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

fn ask_user_to_continue_with_halt(halt_config: &HaltConfig) -> crate::Result<bool> {
    // let stdin = io::stdin();
    // let stdout = io::stdout();
    //
    // let mut r_handle = stdin.lock();
    // let mut w_handle = stdout.lock();
    //
    // w_handle.write_all("\n".as_bytes()).wrap()?;
    // w_handle
    //     .write_fmt(format_args!(
    //         "{} Found {} file.\n",
    //         "✔".green(),
    //         constants::HALT_CONFIG_FILE_NAME.green()
    //     ))
    //     .wrap()?;
    //
    // w_handle
    //     .write_fmt(format_args!(
    //         "{} Want to resume the release? Last checked task: [{}, {}] (yes){}",
    //         "?".cyan(),
    //         halt_config.last_checked.task_id.to_string().as_str().cyan(),
    //         halt_config.last_checked.sub_task_id.to_string().as_str().cyan(),
    //         " › ".bright_black(),
    //     ))
    //     .wrap()?;
    //
    // w_handle.flush().wrap()?;
    //
    // let mut line = String::with_capacity(10);
    // r_handle.read_line(&mut line).wrap()?;
    //
    // w_handle.write_all("\n".as_bytes()).wrap()?;
    //
    // let mut line = line.trim();
    //
    // if line.is_empty() {
    //     line = "yes";
    // }
    //
    // Ok(constants::BOOL_POSSIBLE_TRUTHY_INPUTS.contains(&line.to_lowercase().as_str()))
    Ok(false)
}

fn execute_manual_task(
    release_config: &ReleaseConfig,
    task_id: u64,
    task: &Task,
    start_sub_task_id: u64,
    root_dir: &Path,
    vars_data: &HashMap<String, String>,
) -> crate::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut r_handle = stdin.lock();
    let mut w_handle = stdout.lock();
    let mut w_err_handle = stderr.lock();

    w_handle
        .write_fmt(format_args!("{} {}\n\n", "✔ Checked:".green().bold(), task.name))
        .wrap()?;

    let sub_tasks = task.sub_tasks().iter().skip(start_sub_task_id as usize);
    for (sub_task, idx) in sub_tasks.zip(start_sub_task_id..) {
        let sub_task = substitute_variable_in_text(sub_task.as_str(), vars_data);
        let instruction = sub_task.trim();

        w_handle
            .write_fmt(format_args!(
                "   {} {} {}     ",
                "✔".green().bold(),
                instruction.cyan(),
                "[Press Enter if done or q to quit]".bright_black()
            ))
            .wrap()?;
        w_handle.flush().wrap()?;

        let mut line = String::with_capacity(10);
        r_handle.read_line(&mut line).wrap()?;

        let line = line.trim();
        if line == constants::MANUAL_TASK_QUIT_COMMAND {
            std::process::exit(0);
        }

        save_last_checked(release_config, task_id, idx, root_dir)?;

        w_handle.write_all("\n".as_bytes()).wrap()?;
    }

    w_handle.write_all("\n".as_bytes()).wrap()?;

    Ok(())
}

fn execute_auto_task(
    release_config: &ReleaseConfig,
    task_id: u64,
    task: &Task,
    start_sub_task_id: u64,
    root_dir: &Path,
    vars_data: &HashMap<String, String>,
) -> crate::Result<()> {
    let stdout = io::stdout();
    let mut w_handle = stdout.lock();

    let cur_pos = cursor_pos()?;
    // w_handle
    //     .write_fmt(format_args!("{} {}\n\n", "? Running:".yellow(), task.name))
    //     .wrap()?;
    w_handle
        .write_fmt(format_args!("{} {}\n\n", "✔ Checked:".green().bold(), task.name))
        .wrap()?;
    w_handle.flush().wrap()?;

    // @todo: check if confirm attribute is given.

    let sub_tasks = task.sub_tasks().iter().skip(start_sub_task_id as usize);
    for (sub_task, idx) in sub_tasks.zip(start_sub_task_id..) {
        let command = sub_task.trim();

        let cur_pos = cursor_pos()?;
        // w_handle
        //     .write_fmt(format_args!(
        //         "   {} {} {}\n     ",
        //         "?".yellow(),
        //         "$".bold(),
        //         command.cyan()
        //     ))
        //     .wrap()?;
        w_handle
            .write_fmt(format_args!(
                "   {} {} {}\n     ",
                "✔".green().bold(),
                "$".bold(),
                command.cyan()
            ))
            .wrap()?;
        w_handle.flush().wrap()?;

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
            .spawn()
            .context(format!(
                "Couldn't run the auto task command: '{}' for task: '{}', maybe the 'sh' or 'cmd' program not found on the system",
                command, task.name
            ))?;

        let r_err_child = child.stderr.take();
        let thread_handle = std::thread::spawn(|| {
            let stderr = io::stderr();
            let mut w_err_handle = stderr.lock();

            // w_err_handle.write_fmt(format_args!("     ")).wrap().unwrap();

            if let Some(mut r_err_child) = r_err_child {
                add_padding_from_reader_to_writer(&mut r_err_child, &mut w_err_handle, "     ").unwrap();
            }
        });

        if let Some(mut r_child) = child.stdout.take() {
            add_padding_from_reader_to_writer(&mut r_child, &mut w_handle, "     ")?;
        }

        thread_handle.join();

        // if let Some(r_err_child) = child.stderr.as_mut() {
        //     add_padding_from_reader_to_writer(r_err_child, &mut w_err_handle, "     ")?;
        // }

        let status = child.wait().context(format!(
            "Couldn't run the auto task command: '{}' for task: '{}', maybe the 'sh' or 'cmd' program not found on the system",
            command, task.name
        ))?;

        if !status.success() {
            return Err(crate::Error::new(format!(
                "Failed to execute auto task command: '{}' with exit code: '{}' for task: '{}'",
                command,
                status.code().map(|code| code.to_string()).unwrap_or("na".to_owned()),
                task.name
            )));
        }

        save_last_checked(release_config, task_id, idx, root_dir)?;

        // let cur_pos2 = cursor_pos()?;
        // set_cursor_pos(&mut w_handle, cur_pos)?;
        // w_handle.write_fmt(format_args!("   {}", "✔".green())).wrap()?;
        // set_cursor_pos(&mut w_handle, cur_pos2)?;

        w_handle.write_all("\n".as_bytes()).wrap()?;
        w_handle.flush().wrap()?;
    }

    // let cur_pos2 = cursor_pos()?;
    // set_cursor_pos(&mut w_handle, cur_pos)?;
    // w_handle.write_fmt(format_args!("{}", "✔ Checked:".green())).wrap()?;
    // set_cursor_pos(&mut w_handle, cur_pos2)?;

    w_handle.write_all("\n".as_bytes()).wrap()?;
    w_handle.flush().wrap()?;

    Ok(())
}

fn save_last_checked(
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

    serde_yaml::to_vec(&halt_config)
        .context("Couldn't convert halt config to yaml")
        .and_then(|data| {
            fs::write(root_dir.join(constants::HALT_CONFIG_FILE_NAME), data).context(format!(
                "Couldn't write task's checked state to halt config file: {}",
                constants::HALT_CONFIG_FILE_NAME
            ))
        })
}

fn gen_vars_data(release_version: &Version) -> HashMap<String, String> {
    let mut h = HashMap::new();
    h.insert(constants::VAR_NAME_VERSION.to_owned(), release_version.to_string());
    h
}

fn cursor_pos() -> crate::Result<(u16, u16)> {
    cursor::position().context("Failed to extract cursor position")
}

fn set_cursor_pos<W: Write>(w: &mut W, pos: (u16, u16)) -> crate::Result<()> {
    w.execute(cursor::MoveTo(pos.0, pos.1))
        .map(|_| ())
        .context("Couldn't set cursor position")
}

// fn add_padding_from_reader_to_writer<R: Read, W: Write>(r: &mut R, w: &mut W, padding: &str) -> crate::Result<()> {
//     let mut buf = [0; 1024];
//
//     loop {
//         let len = match r.read(&mut buf) {
//             Ok(0) => break,
//             Ok(len) => len,
//             Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
//             Err(e) => return Err(e).wrap(),
//         };
//
//         // eprintln!("{:?}", std::thread::current().id());
//
//         let read_buf = &buf[..len];
//         match std::str::from_utf8(read_buf) {
//             Ok(txt) => {
//                 let txt = txt.replace("\n", format!("\n{}", padding).as_str());
//                 w.write_all(txt.as_bytes()).wrap()?;
//             }
//             Err(_) => {
//                 w.write_all(read_buf).wrap()?;
//             }
//         }
//         w.flush().wrap()?;
//     }
//
//     w.flush().wrap()?;
//
//     Ok(())
// }

fn add_padding_from_reader_to_writer<R: Read, W: Write>(r: &mut R, w: &mut W, padding: &str) -> crate::Result<()> {
    let b_reader = BufReader::new(r);

    for line in b_reader.lines() {
        let line = line.wrap()?;
        w.write_all(format!("{}\n{}", line.trim(), padding).as_bytes()).wrap()?;
        w.flush().wrap()?;
    }

    w.flush().wrap()?;

    Ok(())
}
