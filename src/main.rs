//! Main entrypoint.

use clap::Parser;

mod cli;
mod constants;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    println!("Directory: {}", args.directory);
    println!("Cores: {}", args.cores);

    Ok(())
}
