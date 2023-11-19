use std::{
    fs::{self, canonicalize},
    path::PathBuf,
    sync::mpsc::channel,
    thread::sleep,
    time::Duration,
};

use anyhow::Result;
use clap::Parser;
use cranelift_jit::JITModule;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
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
        let path = self.path.clone().join("main.qs");
        let content = fs::read_to_string(path)?;
        let mut tokenizer = Tokenizer::from(content);

        tokenizer.tokenize();

        let mut parser = AstParser::new(tokenizer.tokens);

        parser.parse()?;

        let mut back = CraneliftBackend::<JITModule>::new(Triple::host(), false)?;

        back.watch_mode = true;
        back.compile(parser.exprs)?;
        back.run()?;

        Ok(())
    }
}

impl Command for WatchCommand {
    fn execute(&mut self) -> Result<()> {
        self.path = canonicalize(&self.path)?;

        let (tx, rx) = channel();

        info!("Setting up watcher...");

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        info!("Starting changes watcher...");

        // Don't check for errors, we'll just run it again once it changes.
        let _ = self.run();

        watcher
            .watch(&self.path, RecursiveMode::Recursive)
            .map_err(|v| anyhow!(v))?;

        while let Ok(res) = rx.recv() {
            match res {
                Ok(ev) => {
                    if ev.kind.is_modify() || ev.kind.is_remove() || ev.kind.is_create() {
                        info!("File changes detected, rerunning.");
                        
                        // Let the editor close the handle so we can read.
                        sleep(Duration::from_millis(5));

                        let _ = self.run();

                        // Events fire twice, so consume an extra.
                        if let Err(err) = rx.recv()? {
                            return Err(anyhow!("The file watcher encountered an error: {}", err));
                        }
                    }
                }
                Err(err) => return Err(anyhow!("The file watcher encountered an error: {}", err)),
            }
        }

        Ok(())
    }
}
