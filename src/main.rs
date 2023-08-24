#![allow(irrefutable_let_patterns)]

use clap::{error::ErrorKind, Command, CommandFactory, Parser as ClapParser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use qsc::{compiler::compile, parser::Parser, syntax::Syntax};
use std::{fs, io::stdout, path::PathBuf, process::exit};
use tokio::{main, process as proc};

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
    let content = fs::read_to_string(path.clone()).unwrap();
    let tokens = Parser::new(content).parse();
    let keywords = Syntax::new(tokens).parse();
    let content = compile(keywords);

    fs::write("_tmp.S", content).unwrap();

    proc::Command::new("as")
        .arg("-o")
        .arg("_tmp.o")
        .arg("_tmp.S")
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();

    proc::Command::new("ld")
        .arg("-s")
        .arg("-o")
        .arg("_tmp")
        .arg("_tmp.o")
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();

    fs::remove_file("_tmp.S").unwrap();
    fs::remove_file("_tmp.o").unwrap();

    let name = path.file_name().unwrap().to_str().unwrap();
    let mut name = name.split(".").collect::<Vec<&str>>();
    
    name.pop();
    
    let name = name.join(".");

    fs::rename("_tmp", name).unwrap();
}
