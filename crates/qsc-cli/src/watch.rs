use std::{
    fs::{self, canonicalize},
    path::PathBuf,
    sync::mpsc::channel,
    thread::sleep,
    time::Duration,
};

use clap::Parser;
use miette::IntoDiagnostic;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use qsc_compiler::Compiler;
use qsc_core::error::Result;
use target_lexicon::Triple;

use qsc_codegen::jit::JitGenerator;

use super::Command;

#[derive(Debug, Clone, Parser)]
pub struct WatchCommand {
    /// The path to the directory to watch.
    pub path: PathBuf,

    /// Additional libraries.
    #[arg(short = 'l', long = "link")]
    pub libraries: Vec<String>,
}

impl WatchCommand {
    pub fn run(&self) -> Result<()> {
        let path = self.path.clone().join("main.qs");
        let name = path.file_name().unwrap().to_str().unwrap();
        let content = fs::read_to_string(path.clone()).into_diagnostic()?;

        let compiler = Compiler::<JitGenerator>::compile(
            name,
            content,
            Triple::host(),
            self.libraries.clone(),
        )?;

        let code = compiler.run()?;

        println!("=> Process exited with code {}", code);

        Ok(())
    }
}

impl Command for WatchCommand {
    fn execute(&mut self) -> Result<()> {
        self.path = canonicalize(&self.path).into_diagnostic()?;

        let (tx, rx) = channel();

        info!("Setting up watcher...");

        let mut watcher = RecommendedWatcher::new(tx, Config::default()).into_diagnostic()?;

        info!("Starting changes watcher...");

        // Don't check for errors, we'll just run it again once it changes.
        let _ = self.run()?;

        watcher
            .watch(&self.path, RecursiveMode::Recursive)
            .map_err(|v| miette!(v))?;

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
                        if let Err(err) = rx.recv().into_diagnostic()? {
                            return Err(
                                miette!("The file watcher encountered an error: {}", err).into()
                            );
                        }
                    }
                }

                Err(err) => {
                    return Err(miette!("The file watcher encountered an error: {}", err).into())
                }
            }
        }

        Ok(())
    }
}
