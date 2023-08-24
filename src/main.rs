#![allow(irrefutable_let_patterns)]

pub mod parser;
pub mod token;
pub mod cursor;

use std::{path::PathBuf, fs, io::stdout, process::exit};

use clap::{Parser as ClapParser, Subcommand, Command, CommandFactory, error::ErrorKind};
use clap_complete::{Shell, Generator, generate};
use parser::Parser;
use tokio::main;
use serde_json::to_string_pretty;

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
        let err = cmd.error(ErrorKind::MissingRequiredArgument, "Missing value for file!");

        err.print().unwrap();
        exit(1);
    }

    let path = cli.file.unwrap();
    let path = PathBuf::from(path);
    let content = fs::read_to_string(path).unwrap();
    let tokens = Parser::new(content).parse();

    println!("Content:\n{}", to_string_pretty(&tokens).unwrap());
}
