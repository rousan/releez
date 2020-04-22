pub use self::error::{Error, ErrorExt, ResultExt};

pub mod constants;
mod error;
pub mod prelude;
pub mod runner;
pub mod types;
pub mod utils;

pub type Result<T> = std::result::Result<T, Error>;
