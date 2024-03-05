use std::{fs, path::PathBuf, str::FromStr};

use anyhow::Result;
use clap::Parser;
use target_lexicon::Triple;
use tempfile::NamedTempFile;

use qsc_codegen::{aot::AotGenerator, simple::SimpleCompiler};
use qsc_lexer::lexer::Lexer;
use qsc_linker::run_linker;

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

    /// Output CLIF.
    #[arg(short = 'i', long = "clif")]
    pub clif: bool,

    /// Instead of compiling, dump the tokens.
    #[arg(long = "dump-tokens")]
    pub dump_tokens: bool,

    /// Instead of compiling, dump the AST.
    #[arg(long = "dump-ast")]
    pub dump_ast: bool,

    /// The linker. Defaults to mold, lld, gold, ld, clang, gcc, or cc, in order of weight.
    #[arg(short = 'l', long = "linker")]
    pub linker: Option<String>,
}

impl Command for CompileCommand {
    fn execute(&mut self) -> Result<()> {
        let triple = self
            .target
            .clone()
            .map(|v| Triple::from_str(v.as_str()).unwrap())
            .unwrap_or(Triple::host());

        let content = fs::read_to_string(self.file.clone())?;

        debug!("Lexing file: {}", self.file.to_str().unwrap());

        let exprs = Lexer::new().lex(content)?;

        if self.dump_ast {
            println!("{:#?}", exprs);
            return Ok(());
        }

        debug!("Compiling file: {}", self.file.to_str().unwrap());

        let mut compiler = SimpleCompiler::<AotGenerator>::new(triple.clone())?;

        compiler.compile(exprs)?;

        debug!("Emitting object(s)...");

        if self.vcode {
            let mut file = self.file.clone();

            file.set_extension("vcode");

            fs::write(file, compiler.vcode())?;
        }

        if self.clif {
            let mut file = self.file.clone();

            file.set_extension("clif");

            fs::write(file, compiler.clif()?)?;
        }

        // The compiler is consumed here, so we can't output both asm
        // and object files at the same time.
        if self.asm {
            let mut file = self.file.clone();

            file.set_extension("s");

            fs::write(file, compiler.asm()?)?;

            return Ok(());
        }

        if self.object {
            let obj = compiler.finalize();
            let data = obj.object.write()?;
            let mut file = self.file.clone();

            {
                #[cfg(windows)]
                {
                    file.set_extension("obj");
                }

                #[cfg(not(windows))]
                {
                    file.set_extension("o");
                }
            }

            fs::write(file, data)?;

            return Ok(());
        }

        let tmp_file = NamedTempFile::new()?;
        let obj = compiler.finalize();
        let data = obj.object.write()?;

        fs::write(tmp_file.path(), data)?;

        let out_path = {
            #[cfg(windows)]
            {
                format!(
                    "{}.exe",
                    self.file
                        .clone()
                        .to_str()
                        .unwrap()
                        .split('.')
                        .next()
                        .unwrap()
                        .to_string()
                )
            }

            #[cfg(not(windows))]
            {
                self.file
                    .clone()
                    .to_str()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap()
                    .to_string()
            }
        };

        debug!(
            "Linking object: {} -> {}",
            tmp_file.path().to_str().unwrap(),
            out_path
        );

        run_linker(
            PathBuf::from(out_path),
            self.linker.clone(),
            tmp_file.path().into(),
            triple,
        )?;

        Ok(())
    }
}
