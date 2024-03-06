use std::io::stdout;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use miette::Result;

use super::{Cli, Command};

#[derive(Debug, Clone, Parser)]
pub struct CompletionsCommand {
    /// The shell to generate for.
    pub shell: Shell,
}

impl<'a> Command<'a> for CompletionsCommand {
    fn execute(&'a mut self) -> Result<()> {
        let mut cmd = Cli::command();
        let name = cmd.get_name().to_string();

        generate(self.shell, &mut cmd, name, &mut stdout());

        Ok(())
    }
}
