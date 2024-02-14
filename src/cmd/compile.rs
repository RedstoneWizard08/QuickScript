use std::{fs, path::PathBuf, str::FromStr};

use anyhow::Result;
use clap::Parser;
use target_lexicon::Triple;

use crate::tokenizer::Tokenizer;

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct CompileCommand {
    /// The path to the file to compile.
    pub file: PathBuf,

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
}

impl Command for CompileCommand {
    fn execute(&mut self) -> Result<()> {
        let _triple = self
            .target
            .clone()
            .map(|v| Triple::from_str(v.as_str()).unwrap())
            .unwrap_or(Triple::host());

        let content = fs::read_to_string(self.file.clone())?;
        let mut tokenizer = Tokenizer::new(self.file.clone().to_str().unwrap(), content);

        let tokens = tokenizer.tokenize();

        println!("tokens: {:#?}", tokens);

        Ok(())
    }
}
