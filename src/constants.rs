//! Global general app constants.

/// The application name.
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// The application version.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The application author.
pub const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

/// The application description.
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// The current directory path.
pub const CURRENT_DIRECTORY_PATH: &str = ".";

/// List of acceptable image extensions.
pub const ACCEPTABLE_IMAGE_EXTENSIONS: [&str; 2] = ["jpg", "jpeg"];
