use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::{Command as Cmd, Stdio},
    str::FromStr,
};

use anyhow::Result;
use clap::Parser;
use cranelift_object::ObjectModule;
use qsc_cranelift_jit::JITModule;
use target_lexicon::Triple;
use tempfile::NamedTempFile;

use crate::{
    ast::AstParser,
    codegen::backend::CraneliftBackend,
    linker::{get_default_linker, get_library_dir},
    strip::strip_binary,
    tokenizer::Tokenizer,
    util::name_no_ext,
};

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
        let triple = self
            .target
            .clone()
            .map(|v| Triple::from_str(v.as_str()).unwrap())
            .unwrap_or(Triple::host());

        let content = fs::read_to_string(self.file.clone())?;
        let mut tokenizer = Tokenizer::from(content);

        tokenizer.tokenize();

        let mut parser = AstParser::new(tokenizer.tokens);

        parser.parse()?;

        if self.jit {
            let mut back = CraneliftBackend::<JITModule>::new(triple.clone(), self.vcode)?;

            back.compile(parser.exprs)?;

            if self.vcode {
                let out_path = PathBuf::from(format!("{}.vcode", name_no_ext(self.file.clone())));

                fs::write(out_path, back.vcode())?;

                return Ok(());
            }

            back.run()?;

            return Ok(());
        } else {
            let mut back = CraneliftBackend::<ObjectModule>::new(triple.clone(), self.vcode)?;

            back.compile(parser.exprs)?;

            if self.vcode {
                let out_path = PathBuf::from(format!("{}.vcode", name_no_ext(self.file.clone())));

                fs::write(out_path, back.vcode())?;

                return Ok(());
            }

            if self.asm {
                let out_path = PathBuf::from(format!("{}.asm", name_no_ext(self.file.clone())));

                fs::write(out_path, back.asm())?;

                return Ok(());
            }

            let product = back.finalize();
            let mut object = product.object;

            if self.strip {
                strip_binary(&mut object)?;
            }

            let data = object.write().unwrap();

            if self.object {
                let out_path = PathBuf::from(format!("{}.o", name_no_ext(self.file.clone())));
                let mut out_file = File::create(out_path)?;

                out_file.write_all(&*data.into_boxed_slice())?;

                return Ok(());
            }

            let out_path = PathBuf::from(name_no_ext(self.file.clone()));
            let mut tmp_file = NamedTempFile::new()?;

            tmp_file.write_all(&*data.into_boxed_slice())?;

            Cmd::new(
                self.linker
                    .clone()
                    .unwrap_or(get_default_linker().to_string()),
            )
            .arg("-o")
            .arg(out_path)
            .args(get_library_dir(
                None,
                Some(triple.architecture.to_string()),
                Some(triple.operating_system.to_string()),
                Some(triple.environment.to_string()),
            ))
            .arg("-lc")
            .arg(tmp_file.path())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;
        }

        Ok(())
    }
}
