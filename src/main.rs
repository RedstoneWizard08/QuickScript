#![allow(irrefutable_let_patterns)]

use clap::{error::ErrorKind, Command, CommandFactory, Parser as ClapParser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use cranelift_object::ObjectModule;
use qsc::{
    ast::AstParser, codegen::backend::CraneliftBackend, tokenizer::Tokenizer, util::name_no_ext,
};
use std::{
    fs,
    io::{stdout, Write},
    path::PathBuf,
    process::{exit, Command as Cmd, Stdio},
};
use tempfile::NamedTempFile;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the file to compile.
    pub file: Option<String>,

    /// Print parsed tokens only.
    #[arg(short = 't', long = "print-tokens")]
    pub print_tokens_only: bool,

    /// Print parsed keywords only.
    #[arg(short = 'k', long = "print-keywords")]
    pub print_keywords_only: bool,

    /// Use JIT mode.
    #[arg(short = 'j', long = "jit")]
    pub jit: bool,

    /// The linker.
    #[arg(short = 'l', long = "linker", default_value = "ld")]
    pub linker: String,

    /// A sub-command.
    #[command(subcommand)]
    pub command: Option<Commands>,
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

pub fn main() {
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

    if cli.jit {
        todo!("JIT compilation is not implemented yet!");
    } else {
        let mut back = CraneliftBackend::<ObjectModule>::new().unwrap();

        back.compile(parser.exprs).unwrap();

        let out_path = PathBuf::from(name_no_ext(path));
        let mut tmp_file = NamedTempFile::new().unwrap();
        let product = back.finalize();
        let data = product.emit().unwrap();

        tmp_file.write_all(&*data.into_boxed_slice()).unwrap();

        Cmd::new(cli.linker)
            .arg("-o")
            .arg(out_path)
            .arg("-lc")
            .arg(tmp_file.path())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
