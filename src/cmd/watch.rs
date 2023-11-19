use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use cranelift_jit::JITModule;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use target_lexicon::Triple;

use crate::{ast::AstParser, codegen::backend::CraneliftBackend, tokenizer::Tokenizer};

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct WatchCommand {
    /// The path to the directory to watch.
    pub path: PathBuf,
}

impl WatchCommand {
    pub fn run(&self) -> Result<()> {
        let content = fs::read_to_string(self.path.clone().join("main.qs"))?;
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

impl Command for WatchCommand {
    fn execute(&self) -> Result<()> {
        let this = self.clone();

        info!("Setting up watcher...");

        let mut watcher = recommended_watcher(move |res| match res {
            Ok(_) => this.run().unwrap(),
            Err(err) => panic!("The file watcher encountered an error: {}", err),
        })?;

        info!("Starting changes watcher...");

        self.run().unwrap();

        watcher
            .watch(&self.path, RecursiveMode::Recursive)
            .map_err(|v| anyhow!(v))?;

        Ok(())
    }
}
