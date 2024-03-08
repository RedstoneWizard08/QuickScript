use std::{fs, path::PathBuf, process::exit};

use clap::Parser;
use miette::{IntoDiagnostic, Result};
use qsc_compiler::Compiler;
use target_lexicon::Triple;

use qsc_codegen::jit::JitGenerator;

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct RunCommand {
    /// The path to the file to compile.
    pub file: PathBuf,

    /// Instead of running, dump the AST.
    #[arg(long = "dump-ast")]
    pub dump_ast: bool,

    /// Output VCode.
    #[arg(short = 'e', long = "vcode")]
    pub vcode: bool,

    /// Output ASM.
    #[arg(short = 'S', long = "asm")]
    pub asm: bool,

    /// Output CLIF.
    #[arg(short = 'i', long = "clif")]
    pub clif: bool,
}

impl<'a> Command<'a> for RunCommand {
    fn execute(&'a mut self) -> Result<()> {
        let name = self.file.file_name().unwrap().to_str().unwrap();
        let content = fs::read_to_string(self.file.clone()).into_diagnostic()?;
        let mut compiler = Compiler::<JitGenerator>::new(name, content);

        compiler.compile(Triple::host())?;

        if self.dump_ast {
            println!("{:#?}", compiler.ast());
            return Ok(());
        }

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
