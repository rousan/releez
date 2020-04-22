use crate::prelude::*;
use crate::types::ReleaseConfig;
use semver::Version;
use std::fs;
use std::path::Path;

mod executor;

pub fn run_release_checklist(config_file_path: &str, release_version: &str) -> crate::Result<()> {
    let config_file_path = Path::new(config_file_path)
        .canonicalize()
        .context(format!("Config file path does not exist: {}", config_file_path))?;

    let r_release_version = release_version.trim();
    let r_release_version = if r_release_version.starts_with("v") {
        r_release_version.trim_start_matches("v")
    } else {
        r_release_version
    };

    let release_version = Version::parse(r_release_version)
        .context(format!("Invalid release version provided: '{}'", release_version))?;

    let release_config = ReleaseConfig::parse(
        fs::read_to_string(config_file_path.as_path())
            .context("Couldn't read the config file as text")?
            .as_str(),
    )
    .context("Couldn't parse the config file as a yaml file")?;

    println!("{}", serde_yaml::to_string(&release_config).unwrap());
    println!("{:?}", release_version);

    Ok(())
}
