#![allow(irrefutable_let_patterns)]

use clap::{error::ErrorKind, Command, CommandFactory, Parser as ClapParser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use qsc::{
    ast::AstParser, codegen::backend::CraneliftBackend, tokenizer::Tokenizer, util::name_no_ext,
};
use std::{fs, io::stdout, path::PathBuf, process::exit};
use tokio::main;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the file to compile.
    file: Option<String>,

    /// Print parsed tokens only.
    #[arg(short = 't', long = "print-tokens")]
    print_tokens_only: bool,

    /// Print parsed keywords only.
    #[arg(short = 'k', long = "print-keywords")]
    print_keywords_only: bool,

    /// Output ASM.
    #[arg(short = 's', long = "asm")]
    asm: bool,

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

    let path = PathBuf::from(cli.file.unwrap());
    let content = fs::read_to_string(path.clone()).unwrap();
    let mut tokenizer = Tokenizer::from(content);

    tokenizer.tokenize();

    let mut parser = AstParser::new(tokenizer.tokens);

    parser.parse().unwrap();

    let mut back = CraneliftBackend::new().unwrap();

    back.compile(parser.exprs).unwrap();

    let product = back.finish();
    let data = product.emit().unwrap();

    let out_path = path
        .clone()
        .parent()
        .unwrap()
        .join(format!("{}.o", name_no_ext(path)));

    fs::write(out_path, data).unwrap();
}
