pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_AUTHOR: &'static str = "Rousan Ali <hello@rousan.io> (https://rousan.io)";

pub const CONFIG_FILE_NAME: &'static str = "releez.yml";
pub const DEFAULT_CONFIG_FILE_PATH: &'static str = "./releez.yml";
pub const HALT_CONFIG_FILE_NAME: &'static str = ".halt.releez.yml";

pub const VAR_NAME_VERSION: &'static str = "VERSION";

pub const BOOL_POSSIBLE_TRUTHY_INPUTS: [&'static str; 2] = ["yes", "y"];
pub const BOOL_POSSIBLE_FALSY_INPUTS: [&'static str; 2] = ["no", "n"];

pub const MANUAL_TASK_QUIT_COMMAND: &'static str = "q";
