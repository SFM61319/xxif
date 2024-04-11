//! Main entrypoint.

use clap::Parser;

use xxif;

mod cli;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    let directory = &args.directory;
    let cores = args.cores;

    let verbose = args.verbose;
    let verbosity = verbose
        .then_some(tracing::Level::TRACE)
        .unwrap_or(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(verbosity)
        .with_file(false)
        .with_line_number(false)
        .init();

    if cores == cli::Args::DEFAULT_CORES {
        tracing::info!("Processing images in `{}` using all cores", directory);
    } else {
        tracing::info!(
            "Processing images in `{}` using `{}` cores",
            directory,
            cores
        );
    }

    xxif::process_images(directory, cores);
    Ok(())
}
