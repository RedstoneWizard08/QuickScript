#![allow(irrefutable_let_patterns)]

use clap::{error::ErrorKind, Command, CommandFactory, Parser as ClapParser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use qsc::{
    arch::{detect_arch, Architecture},
    compiler::{assemble_and_link, compile},
    parser::Parser,
    syntax::Syntax,
};
use std::{fs, io::stdout, path::PathBuf, process::exit};
use tokio::main;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the file to compile.
    file: Option<String>,

    /// The arch to compile for.
    #[arg(short, long)]
    arch: Option<Architecture>,

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

fn name_no_ext(path: PathBuf) -> String {
    let name = path.file_name().unwrap().to_str().unwrap();
    let mut name = name.split(".").collect::<Vec<&str>>();

    name.pop();

    name.join(".")
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

    let arch = cli.arch.unwrap_or(detect_arch());
    let path = cli.file.unwrap();
    let path = PathBuf::from(path);
    let content = fs::read_to_string(path.clone()).unwrap();
    let tokens = Parser::new(content).parse();
    let keywords = Syntax::new(tokens).parse();
    let content = compile(keywords, arch);
    let name = name_no_ext(path);

    assemble_and_link(name, content, arch).await;
}
