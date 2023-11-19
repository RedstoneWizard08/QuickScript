use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use cranelift_jit::JITModule;
use target_lexicon::Triple;

use crate::{ast::AstParser, codegen::backend::CraneliftBackend, tokenizer::Tokenizer};

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct RunCommand {
    /// The path to the file to compile.
    pub file: PathBuf,
}

impl Command for RunCommand {
    fn execute(&mut self) -> Result<()> {
        let content = fs::read_to_string(self.file.clone())?;
        let mut tokenizer = Tokenizer::from(content);

        tokenizer.tokenize();

        let mut parser = AstParser::new(tokenizer.tokens);

        parser.parse()?;

        let mut back = CraneliftBackend::<JITModule>::new(Triple::host(), false)?;

        back.compile(parser.exprs)?;

        back.run()?;

        Ok(())
    }
}
