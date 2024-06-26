//! CLI arguments and parsing.

use clap::Parser;

use xxif::constants;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Parser)]
#[command(name = constants::APP_NAME, version = constants::APP_VERSION, author = constants::APP_AUTHOR, about = constants::APP_DESCRIPTION)]
pub struct Args {
    /// The path containing the image(s) recursively.
    #[arg(short, long, default_value = Self::DEFAULT_PATH)]
    pub path: String,

    /// The number of CPU cores to use (0 = all).
    #[arg(short, long, default_value_t = Self::DEFAULT_CORES)]
    pub num_cores: usize,

    /// Enable verbose mode.
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    /// The default path.
    pub const DEFAULT_PATH: &'static str = constants::CURRENT_DIRECTORY_PATH;

    /// The default number of CPU cores.
    pub const DEFAULT_CORES: usize = usize::MIN;
}
