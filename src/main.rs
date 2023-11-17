#![allow(irrefutable_let_patterns)]

use clap::{error::ErrorKind, Command, CommandFactory, Parser as ClapParser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use cranelift_jit::JITModule;
use cranelift_object::ObjectModule;
use qsc::{
    ast::AstParser,
    codegen::backend::CraneliftBackend,
    linker::{get_default_linker, get_library_dir},
    strip::strip_binary,
    tokenizer::Tokenizer,
    util::name_no_ext,
};
use std::{
    fs::{self, File},
    io::{stdout, Write},
    path::PathBuf,
    process::{exit, Command as Cmd, Stdio},
    str::FromStr,
};
use target_lexicon::Triple;
use tempfile::NamedTempFile;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the file to compile.
    pub file: Option<String>,

    /// The target to compile for.
    #[arg(short = 't', long = "target")]
    pub target: Option<String>,

    /// Use JIT mode.
    #[arg(short = 'j', long = "jit")]
    pub jit: bool,

    /// Strip the binary.
    #[arg(short = 's', long = "strip")]
    pub strip: bool,

    /// Output an object file.
    #[arg(short = 'c', long = "obj")]
    pub object: bool,

    /// Output VCode.
    #[arg(short = 'e', long = "vcode")]
    pub vcode: bool,

    /// Output ASM.
    #[arg(short = 'S', long = "asm")]
    pub asm: bool,

    /// The linker. Defaults to mold, lld, gold, ld, clang, gcc, or cc, in order of weight.
    #[arg(short = 'l', long = "linker")]
    pub linker: Option<String>,

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

    let triple = cli
        .target
        .map(|v| Triple::from_str(v.as_str()).unwrap())
        .unwrap_or(Triple::host());

    let path = PathBuf::from(cli.file.unwrap());
    let content = fs::read_to_string(path.clone()).unwrap();
    let mut tokenizer = Tokenizer::from(content);

    tokenizer.tokenize();

    let mut parser = AstParser::new(tokenizer.tokens);

    parser.parse().unwrap();

    if cli.jit {
        let mut back = CraneliftBackend::<JITModule>::new(triple.clone(), cli.vcode).unwrap();

        back.compile(parser.exprs).unwrap();

        if cli.vcode {
            let out_path = PathBuf::from(format!("{}.vcode", name_no_ext(path)));

            fs::write(out_path, back.vcode()).unwrap();

            return;
        }

        back.run().unwrap();

        return;
    } else {
        let mut back = CraneliftBackend::<ObjectModule>::new(triple.clone(), cli.vcode).unwrap();

        back.compile(parser.exprs).unwrap();

        if cli.vcode {
            let out_path = PathBuf::from(format!("{}.vcode", name_no_ext(path)));

            fs::write(out_path, back.vcode()).unwrap();

            return;
        }

        if cli.asm {
            let out_path = PathBuf::from(format!("{}.asm", name_no_ext(path)));

            fs::write(out_path, back.asm()).unwrap();

            return;
        }

        let product = back.finalize();
        let mut object = product.object;

        if cli.strip {
            strip_binary(&mut object).unwrap();
        }

        let data = object.write().unwrap();

        if cli.object {
            let out_path = PathBuf::from(format!("{}.o", name_no_ext(path)));
            let mut out_file = File::create(out_path).unwrap();

            out_file.write_all(&*data.into_boxed_slice()).unwrap();

            return;
        }

        let out_path = PathBuf::from(name_no_ext(path));
        let mut tmp_file = NamedTempFile::new().unwrap();

        tmp_file.write_all(&*data.into_boxed_slice()).unwrap();

        Cmd::new(cli.linker.unwrap_or(get_default_linker().to_string()))
            .arg("-o")
            .arg(out_path)
            .args(get_library_dir(
                None,
                Some(triple.architecture.to_string()),
                Some(triple.operating_system.to_string()),
                Some(triple.environment.to_string()),
            ))
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
