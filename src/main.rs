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
