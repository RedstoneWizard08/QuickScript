use clap::Parser;
use miette::Result;
use qsc_cli::{Cli, Command};

pub fn main() -> Result<()> {
    Cli::parse().execute()
}
