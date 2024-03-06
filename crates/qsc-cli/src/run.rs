use std::{cell::Cell, fs, path::PathBuf, process::exit};

use clap::Parser;
use miette::{IntoDiagnostic, Result};
use target_lexicon::Triple;

use qsc_codegen::{jit::JitGenerator, simple::SimpleCompiler};
use qsc_lexer::lexer::Lexer;

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct RunCommand {
    /// The path to the file to compile.
    pub file: PathBuf,

    /// Instead of running, dump the tokens.
    #[arg(long = "dump-tokens")]
    pub dump_tokens: bool,

    /// Instead of running, dump the AST.
    #[arg(long = "dump-ast")]
    pub dump_ast: bool,
}

impl<'a> Command<'a> for RunCommand {
    fn execute(&'a mut self) -> Result<()> {
        let name = self.file.file_name().unwrap().to_str().unwrap();
        let content = fs::read_to_string(self.file.clone()).into_diagnostic()?;

        debug!("Lexing file: {}", self.file.to_str().unwrap());

        let lexer = Lexer::new(&name, &content);
        let exprs = lexer.lex()?;

        if self.dump_ast {
            println!("{:#?}", exprs);
            return Ok(());
        }

        debug!("Compiling file: {}", self.file.to_str().unwrap());

        let mut compiler = SimpleCompiler::<JitGenerator>::new(Triple::host())?;
        let compiler_cell = Cell::from_mut(&mut compiler);

        unsafe {
            let compiler = compiler_cell.as_ptr().as_mut().unwrap();

            compiler.compile(exprs)?;
        }

        exit(compiler.run()?);
    }
}
