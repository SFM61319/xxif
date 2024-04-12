//! A program to generate image hashes and add them to the images' EXIF data.

use clap::Parser;

use xxif;

mod cli;

/// The main entrypoint function.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    let path = &args.path;
    let cores = args.num_cores;

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

    tracing::debug!(path = ?path, cores = ?cores, "Recursively processing images");
    match xxif::process_images(path, cores) {
        usize::MIN => tracing::error!("No images processed"),
        count => tracing::info!(processed = ?count, "Successfully processed images"),
    }

    Ok(())
}
