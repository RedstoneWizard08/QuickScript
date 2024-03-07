#[macro_use]
extern crate miette;

#[macro_use]
extern crate log;

pub mod compile;
pub mod completions;
pub mod run;
pub mod style;
pub mod watch;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use const_format::formatcp;
use log::LevelFilter;
use miette::Result;
use pretty_env_logger::formatted_builder;

use self::{
    compile::CompileCommand, completions::CompletionsCommand, run::RunCommand, style::get_styles,
    watch::WatchCommand,
};

pub const VERSION: &str = formatcp!(
    "version {} (commit {})",
    env!("CARGO_PKG_VERSION"),
    env!("COMMIT_HASH")
);

pub const LONG_VERSION: &str = formatcp!(
    "\nVersion: {}\nCommit: {}\nSource: {}",
    env!("CARGO_PKG_VERSION"),
    env!("COMMIT_HASH"),
    env!("CARGO_PKG_REPOSITORY")
);

pub trait Command<'a> {
    fn execute(&'a mut self) -> Result<()>;
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
    propagate_version = true,
    styles = get_styles()
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

    /// Print version information.
    #[command(alias = "v")]
    Version,
}

impl<'a> Command<'a> for Cli {
    fn execute(&'a mut self) -> Result<()> {
        formatted_builder()
            .parse_default_env()
            .filter(Some("cranelift_jit::backend"), LevelFilter::Warn)
            .filter(Some("cranelift_object::backend"), LevelFilter::Warn)
            .filter(Some("cranelift_codegen::timing::enabled"), LevelFilter::Warn)
            .filter_level(self.verbose.log_level_filter())
            .init();

        self.command.execute()
    }
}

impl<'a> Command<'a> for Commands {
    fn execute(&'a mut self) -> Result<()> {
        match self.clone() {
            Commands::Run(mut cmd) => cmd.execute(),
            Commands::Compile(mut cmd) => cmd.execute(),
            Commands::Completions(mut cmd) => cmd.execute(),
            Commands::Watch(mut cmd) => cmd.execute(),
            Commands::Version => Ok(println!("{} {}", env!("PRODUCT_NAME"), VERSION)),
        }
    }
}
