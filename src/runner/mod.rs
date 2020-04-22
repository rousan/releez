use crate::prelude::*;
use std::path::Path;

pub fn run_release_checklist(config_file_path: &str) -> crate::Result<()> {
    let config_file_path = Path::new(config_file_path)
        .canonicalize()
        .context(format!("Config file path does not exist: {}", config_file_path))?;

    // println!("{:?}", config_file_path);

    Ok(())
}
