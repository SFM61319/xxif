//! CLI arguments and parsing.

use clap::Parser;

use xxif::constants;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Parser)]
#[command(name = constants::APP_NAME, version = constants::APP_VERSION, author = constants::APP_AUTHOR, about = constants::APP_DESCRIPTION)]
pub struct Args {
    /// The directory containing the images.
    /// Recursively searches all subdirectories.
    #[arg(short, long, default_value = Self::DEFAULT_DIRECTORY_PATH)]
    pub directory: String,

    /// The number of cores to use.
    /// Set to `0` to use all cores.
    #[arg(short, long, default_value_t = Self::DEFAULT_CORES)]
    pub cores: usize,

    /// Enable verbose mode.
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    /// The default directory path.
    pub const DEFAULT_DIRECTORY_PATH: &'static str = constants::CURRENT_DIRECTORY_PATH;

    /// The default number of cores.
    pub const DEFAULT_CORES: usize = usize::MIN;
}
