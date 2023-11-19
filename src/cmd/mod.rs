pub mod compile;
pub mod completions;
pub mod run;
pub mod watch;

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use const_format::formatcp;
use pretty_env_logger::formatted_builder;

use self::{
    compile::CompileCommand, completions::CompletionsCommand, run::RunCommand, watch::WatchCommand,
};

pub const VERSION: &str = formatcp!(
    "version {} (commit {})",
    env!("CARGO_PKG_VERSION"),
    env!("COMMIT_HASH")
);

pub const LONG_VERSION: &str = formatcp!(
    "\nVersion: {}\nCommit: {}\nSource: {}\n",
    env!("CARGO_PKG_VERSION"),
    env!("COMMIT_HASH"),
    env!("CARGO_PKG_REPOSITORY")
);

pub trait Command {
    fn execute(&self) -> Result<()>;
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = env!("PRODUCT_NAME"),
    bin_name = "qsc",
    author,
    version = VERSION,
    long_version = LONG_VERSION,
    about,
    long_about = None,
    propagate_version = true
)]
pub struct Cli {
    /// Enables verbose mode.
    #[command(flatten)]
    pub verbose: Verbosity,

    /// A sub-command.
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Generates shell completions.
    #[command(aliases = &["completion", "complete"])]
    Completions(CompletionsCommand),

    /// Compile a source file.
    #[command(alias = "c")]
    Compile(CompileCommand),

    /// Run a file in place.
    #[command(aliases = &["r", "x"])]
    Run(RunCommand),

    /// Watch for changes and run.
    #[command(alias = "w")]
    Watch(WatchCommand),

    #[command(alias = "v")]
    Version,
}

impl Command for Cli {
    fn execute(&self) -> Result<()> {
        formatted_builder()
            .filter_level(self.verbose.log_level_filter())
            .init();

        self.command.execute()
    }
}

impl Command for Commands {
    fn execute(&self) -> Result<()> {
        match self.clone() {
            Commands::Run(cmd) => cmd.execute(),
            Commands::Compile(cmd) => cmd.execute(),
            Commands::Completions(cmd) => cmd.execute(),
            Commands::Watch(cmd) => cmd.execute(),
            Commands::Version => Ok(println!("{} {}", env!("PRODUCT_NAME"), VERSION)),
        }
    }
}
