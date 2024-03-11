use clap::Parser;
use miette::{IntoDiagnostic, Result};
use qsc_cli::{Cli, Command};

pub fn main() -> Result<()> {
    Cli::parse().execute().into_diagnostic()
}
