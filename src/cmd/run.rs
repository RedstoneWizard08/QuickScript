use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;

use crate::tokenizer::Tokenizer;

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct RunCommand {
    /// The path to the file to compile.
    pub file: PathBuf,
}

impl Command for RunCommand {
    fn execute(&mut self) -> Result<()> {
        let content = fs::read_to_string(self.file.clone())?;
        let mut tokenizer = Tokenizer::new(self.file.clone().to_str().unwrap(), content);

        tokenizer.tokenize();

        Ok(())
    }
}
