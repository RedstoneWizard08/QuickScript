use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;

use crate::tokenizer::{cursor::Cursor, Tokenizer};

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct RunCommand {
    /// The path to the file to compile.
    pub file: PathBuf,
}

impl Command for RunCommand {
    fn execute(&mut self) -> Result<()> {
        let content = fs::read_to_string(self.file.clone())?;

        let cursor = Cursor::new(
            self.file.clone().to_str().unwrap().to_string(),
            content.chars().collect(),
        );

        let mut tokenizer = Tokenizer::new(cursor);

        tokenizer.tokenize();

        Ok(())
    }
}
