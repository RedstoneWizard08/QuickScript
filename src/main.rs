#![allow(irrefutable_let_patterns)]

use std::{fs, io::stdout, path::PathBuf, process::exit};

use clap::{error::ErrorKind, Command, CommandFactory, Parser as ClapParser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use qsc::parser::Parser;
use serde_json::to_string_pretty;
use tokio::main;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the file to compile.
    file: Option<String>,

    /// A sub-command.
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generates shell completions.
    Completion {
        /// The shell to generate for.
        shell: Shell,
    },
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut stdout());
}

#[main]
pub async fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        if let Commands::Completion { shell } = command {
            let mut cmd = Cli::command();

            print_completions(shell, &mut cmd);

            return;
        }
    }

    if cli.file.is_none() {
        let mut cmd = Cli::command();
        let err = cmd.error(
            ErrorKind::MissingRequiredArgument,
            "Missing value for file!",
        );

        err.print().unwrap();
        exit(1);
    }

    let path = cli.file.unwrap();
    let path = PathBuf::from(path);
    let content = fs::read_to_string(path).unwrap();
    let tokens = Parser::new(content).parse();

    println!("Content:\n{}", to_string_pretty(&tokens).unwrap());
}
