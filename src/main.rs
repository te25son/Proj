mod cli;
mod dependencies;
mod templates;
mod errors;

use clap::Parser;

use cli::Proj;
use dependencies::Deps;
use errors::ErrorType;

fn main() -> anyhow::Result<(), ErrorType> {
    Deps::check_all()?;
    Proj::parse().create()?;
    Ok(())
}
