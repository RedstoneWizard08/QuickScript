use std::{
    fs::{self, canonicalize},
    path::PathBuf,
    sync::mpsc::channel,
    thread::sleep,
    time::Duration,
};

use anyhow::Result;
use clap::Parser;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use target_lexicon::Triple;

use crate::{
    codegen::{jit::JitGenerator, simple::SimpleCompiler},
    lexer::Lexer,
    tokenizer::{cursor::Cursor, Tokenizer},
};

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct WatchCommand {
    /// The path to the directory to watch.
    pub path: PathBuf,
}

impl WatchCommand {
    pub fn run(&self) -> Result<()> {
        let path = self.path.clone().join("main.qs");
        let content = fs::read_to_string(path.clone())?;

        let cursor = Cursor::new(
            path.clone().to_str().unwrap().to_string(),
            content.chars().collect(),
        );

        let mut tokenizer = Tokenizer::new(cursor.clone());
        let tokens = tokenizer.tokenize();
        let mut lexer = Lexer::new(cursor, tokens);
        let exprs = lexer.lex()?;

        let mut compiler = SimpleCompiler::<JitGenerator>::new(Triple::host())?;

        compiler.compile(exprs)?;

        let code = compiler.run()?;

        println!("=> Process exited with code {}", code);

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
        let _ = self.run()?;

        watcher
            .watch(&self.path, RecursiveMode::Recursive)
            .map_err(|v| anyhow!(v))?;

        while let Ok(res) = rx.recv() {
            match res {
                Ok(ev) => {
                    if ev.kind.is_modify() || ev.kind.is_remove() || ev.kind.is_create() {
                        if !ev.paths.iter().any(|p| p.ends_with(".qs")) {
                            continue;
                        }

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
