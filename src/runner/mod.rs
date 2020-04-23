use crate::constants;
use crate::prelude::*;
use crate::types::{HaltConfig, ReleaseConfig};
use semver::Version;
use tokio::fs;

mod executor;

pub async fn run_release_checklist(config_file_path: &str, release_version: &str) -> crate::Result<()> {
    let config_file_path = fs::canonicalize(config_file_path)
        .await
        .context(format!("Config file path does not exist: {}", config_file_path))?;

    let project_root_dir = config_file_path
        .parent()
        .ok_or_else(|| crate::Error::new("Couldn't access project root dir"))?;

    let release_version = {
        let version = release_version.trim();
        let version = if version.starts_with("v") {
            version.trim_start_matches("v")
        } else {
            version
        };
        Version::parse(version).context(format!("Invalid release version provided: '{}'", release_version))?
    };

    let release_config = ReleaseConfig::parse(
        fs::read_to_string(config_file_path.as_path())
            .await
            .context("Couldn't read the config file as text")?
            .as_str(),
    )
    .context("Couldn't parse the config file as a yaml file")?;

    let halt_config = fs::read_to_string(project_root_dir.join(constants::HALT_CONFIG_FILE_NAME))
        .await
        .ok()
        .and_then(|config_text| HaltConfig::parse(config_text.as_str()).ok());

    executor::execute_checklist(
        &release_config,
        halt_config.as_ref(),
        &release_version,
        project_root_dir,
    )
    .await?;

    Ok(())
}
