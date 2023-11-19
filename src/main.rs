use anyhow::Result;
use clap::Parser;
use quickscript::cmd::{Cli, Command};

pub fn main() -> Result<()> {
    Cli::parse().execute()
}
