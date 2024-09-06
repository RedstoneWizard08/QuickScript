use std::{fs, path::PathBuf, process::exit};

use clap::Parser;
use miette::IntoDiagnostic;
use qsc_compiler::Compiler;
use qsc_core::error::Result;
use target_lexicon::Triple;

use qsc_codegen::jit::JitGenerator;

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct RunCommand {
    /// The path to the file to compile.
    pub file: PathBuf,

    /// Instead of running, dump the Abstract Syntax Tree.
    #[arg(long = "dump-ast")]
    pub dump_ast: bool,

    /// Instead of running, dump the Processed Syntax Tree.
    #[arg(long = "dump-pst")]
    pub dump_pst: bool,

    /// Output VCode.
    #[arg(short = 'e', long = "vcode")]
    pub vcode: bool,

    /// Output ASM.
    #[arg(short = 'S', long = "asm")]
    pub asm: bool,

    /// Output CLIF.
    #[arg(short = 'i', long = "clif")]
    pub clif: bool,

    /// Additional libraries.
    #[arg(short = 'l', long = "lib")]
    pub libraries: Vec<String>,
}

impl Command for RunCommand {
    fn execute(&mut self) -> Result<()> {
        let name = self.file.file_name().unwrap().to_str().unwrap();
        let content = fs::read_to_string(self.file.clone()).into_diagnostic()?;

        if self.dump_ast {
            let ast = Compiler::<JitGenerator>::dump_ast(name, &content)?;
            let mut file = self.file.clone();

            file.set_extension("ast.ron");

            fs::write(file, ast).into_diagnostic()?;
        }

        if self.dump_pst {
            let ast = Compiler::<JitGenerator>::dump_pst(name, &content)?;
            let mut file = self.file.clone();

            file.set_extension("pst.ron");

            fs::write(file, ast).into_diagnostic()?;
        }

        let compiler = Compiler::<JitGenerator>::compile(
            name,
            content,
            Triple::host(),
            self.libraries.clone(),
        )?;

        if self.vcode {
            let mut file = self.file.clone();

            file.set_extension("vcode");

            fs::write(file, compiler.vcode()).into_diagnostic()?;
        }

        if self.clif {
            let mut file = self.file.clone();

            file.set_extension("clif");

            fs::write(file, compiler.clif()?).into_diagnostic()?;
        }

        // The compiler is consumed here, so we can't output both asm
        // and object files at the same time.
        if self.asm {
            let mut file = self.file.clone();

            file.set_extension("s");
            fs::write(file, compiler.asm()?).into_diagnostic()?;

            return Ok(());
        }

        exit(compiler.run()?);
    }
}
