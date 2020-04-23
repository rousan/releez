extern crate releez;
use clap::{App, Arg};
use colored::*;
use releez::constants;

fn main() {
    let matches = App::new(constants::APP_NAME)
        .version(constants::APP_VERSION)
        .version_short("v")
        .author(constants::APP_AUTHOR)
        .about(constants::APP_DESCRIPTION)
        .arg(
            Arg::with_name("config")
                .help(format!("The {} config file path", constants::CONFIG_FILE_NAME).as_str())
                .short("c")
                .long("config")
                .value_name("CONFIG_PATH")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("releaseVersion")
                .help("The next release version")
                .value_name("RELEASE_VERSION")
                .index(1)
                .required(true),
        )
        .get_matches();

    let config_file_path = matches
        .value_of("config")
        .unwrap_or(constants::DEFAULT_CONFIG_FILE_PATH);
    let release_version = matches.value_of("releaseVersion").unwrap();

    if let Err(err) = releez::runner::run_release_checklist(config_file_path, release_version) {
        eprintln!("{} {}", "error:".red(), err);
    }
}

// fn main() -> crossterm::Result<()> {
//     use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand, Result};
//     use std::io::{stdout, Write};
//     use std::thread;
//     use std::time::Duration;
//
//     let mut stdout = stdout();
//
//     // stdout.execute(terminal::Clear(terminal::ClearType::All))?;
//     let cur_pos = cursor::position()?;
//     println!("Running");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//     println!("gdfgdfgdfgfdg");
//
//     thread::sleep(Duration::from_secs(5));
//     let cur_pos2 = cursor::position()?;
//     stdout.execute(cursor::MoveTo(cur_pos.0, cur_pos.1));
//     println!("Checked");
//     stdout.execute(cursor::MoveTo(cur_pos2.0, cur_pos2.1));
//
//     Ok(())
// }
