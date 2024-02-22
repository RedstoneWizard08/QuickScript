use anyhow::Result;
use clap::Parser;
use qsc_cli::{Cli, Command};

pub fn main() -> Result<()> {
    Cli::parse().execute()
}
