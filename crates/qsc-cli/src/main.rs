use clap::Parser;
use miette::Result;
use qsc_cli::{Cli, Command};

pub fn main() -> Result<()> {
    Ok(Cli::parse().execute()?)
}
