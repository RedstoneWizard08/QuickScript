#![allow(irrefutable_let_patterns)]

use clap::{error::ErrorKind, Command, CommandFactory, Parser as ClapParser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use qsc::{
    arch::{detect_arch, Architecture},
    build::build,
    compiler::compile,
    parser::Parser,
    syntax::Syntax,
    util::name_no_ext,
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

    /// Print parsed tokens only.
    #[arg(short = 't', long = "print-tokens")]
    print_tokens_only: bool,

    /// Print parsed keywords only.
    #[arg(short = 'k', long = "print-keywords")]
    print_keywords_only: bool,

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

    /// Does new parsing.
    NewParsingDemo {
        /// The file to parse.
        file: String,
    },
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut stdout());
}

#[main]
#[cfg(not(target_arch = "wasm32"))]
pub async fn main() {
    start().await;
}

#[main(flavor = "current_thread")]
#[cfg(target_arch = "wasm32")]
pub async fn main() {
    start().await;
}

pub async fn start() {
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
    let keywords = Syntax::new(tokens.clone()).parse();

    if cli.print_tokens_only {
        return println!("Tokens:\n{:#?}", tokens);
    }

    if cli.print_keywords_only {
        return println!("Keywords:\n{:#?}", keywords);
    }

    let content = compile(keywords, arch);
    let name = name_no_ext(path);

    build(name, content, arch);
}
