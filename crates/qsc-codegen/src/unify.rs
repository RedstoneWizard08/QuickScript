use std::slice;

use super::{aot::AotGenerator, jit::JitGenerator};
use cranelift_codegen::write_function;
use cranelift_module::Module;
use miette::{IntoDiagnostic, Result};
use qsc_ast::ast::AbstractTree;
use qsc_object::ObjectProduct;
use target_lexicon::Triple;

pub trait CodegenBackend {
    fn new(
        triple: Triple,
        name: String,
        source: String,
        tree: AbstractTree,
        libs: Vec<String>,
    ) -> Result<Self>
    where
        Self: Sized;

    fn compile(&mut self) -> Result<()>;
    fn is_jit(&self) -> bool;
    fn run(&self) -> Result<i32>;
    fn finalize(self) -> ObjectProduct;
    fn clif(&self) -> Result<String>;
    fn vcode(&self) -> String;
    fn tree(&self) -> AbstractTree;
    fn asm(self) -> Result<String>;
    fn clean(self);
}

impl CodegenBackend for AotGenerator {
    fn new(
        triple: Triple,
        name: String,
        source: String,
        tree: AbstractTree,
        _libs: Vec<String>,
    ) -> Result<Self> {
        Ok(Self::new(triple, name, source, tree)?)
    }

    fn compile(&mut self) -> Result<()> {
        let data = self.ctx.read().tree.clone();

        for node in data.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(func) = decl.as_function() {
                    self.compile_function(func)?;
                }
            }
        }

        Ok(())
    }

    fn is_jit(&self) -> bool {
        false
    }

    fn run(&self) -> Result<i32> {
        panic!("AOT backend cannot be run")
    }

    fn finalize(self) -> ObjectProduct {
        self.ctx.into_inner().module.finish()
    }

    fn clif(&self) -> Result<String> {
        let ctx = self.ctx.read();
        let mut buf = String::new();
        let isa = ctx.module.isa();

        for func in ctx.fns.clone() {
            write_function(&mut buf, &func).into_diagnostic()?;
        }

        write_function(&mut buf, &ctx.ctx.func).into_diagnostic()?;

        for flag in isa.flags().iter() {
            buf.push_str(format!("set {}\n", flag).as_str());
        }

        buf.push_str(format!("target {}", isa.triple().architecture).as_str());

        for isa_flag in isa.isa_flags().iter() {
            buf.push_str(format!(" {}", isa_flag).as_str());
        }

        buf.push('\n');
        buf.push('\n');
        buf.push('\n');

        Ok(buf)
    }

    fn vcode(&self) -> String {
        self.ctx
            .read()
            .vcode
            .iter()
            .map(|v| v.vcode.clone())
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn tree(&self) -> AbstractTree {
        self.ctx.read().tree.clone()
    }

    fn asm(self) -> Result<String> {
        let capstone = self
            .ctx
            .read()
            .module
            .isa()
            .to_capstone()
            .map_err(|v| miette!(v))?;

        let product = self.finalize();
        let data = product.object.write().into_diagnostic()?;

        let disasm = capstone
            .disasm_all(&*data.into_boxed_slice(), 0x0)
            .map_err(|v| miette!(v))?;

        Ok(disasm
            .iter()
            .map(|v| format!("{} {}", v.mnemonic().unwrap(), v.op_str().unwrap()))
            .collect::<Vec<String>>()
            .join("\n"))
    }

    fn clean(self) {}
}

impl CodegenBackend for JitGenerator {
    fn new(
        triple: Triple,
        name: String,
        source: String,
        tree: AbstractTree,
        libs: Vec<String>,
    ) -> Result<Self> {
        Ok(Self::new(triple, name, source, tree, libs)?)
    }

    fn compile(&mut self) -> Result<()> {
        let data = self.ctx.read().tree.clone();

        for node in data.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(func) = decl.as_function() {
                    self.compile_function(func)?;
                }
            }
        }

        Ok(())
    }

    fn is_jit(&self) -> bool {
        true
    }

    fn run(&self) -> Result<i32> {
        self.exec()
    }

    fn finalize(self) -> ObjectProduct {
        panic!("JIT backend cannot be finalized")
    }

    fn clif(&self) -> Result<String> {
        let ctx = self.ctx.read();
        let mut buf = String::new();
        let isa = ctx.module.isa();

        for func in ctx.fns.clone() {
            write_function(&mut buf, &func).into_diagnostic()?;
        }

        write_function(&mut buf, &ctx.ctx.func).into_diagnostic()?;

        for flag in isa.flags().iter() {
            buf.push_str(format!("set {}\n", flag).as_str());
        }

        buf.push_str(format!("target {}", isa.triple().architecture).as_str());

        for isa_flag in isa.isa_flags().iter() {
            buf.push_str(format!(" {}", isa_flag).as_str());
        }

        buf.push('\n');
        buf.push('\n');
        buf.push('\n');

        Ok(buf)
    }

    fn vcode(&self) -> String {
        self.ctx
            .read()
            .vcode
            .iter()
            .map(|v| v.vcode.clone())
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn tree(&self) -> AbstractTree {
        self.ctx.read().tree.clone()
    }

    fn asm(self) -> Result<String> {
        let mut main = None;

        for (name, code, size) in self.ctx.read().code.read().values() {
            if name == "main" {
                main = Some(unsafe { slice::from_raw_parts(*code, *size) });

                debug!("Found main function!");
            }
        }

        let main = main
            .ok_or(miette!("Cannot find a main function!"))?
            .to_vec();

        let capstone = self
            .ctx
            .read()
            .module
            .isa()
            .to_capstone()
            .map_err(|v| miette!(v))?;

        let disasm = capstone
            .disasm_all(&*main.into_boxed_slice(), 0x0)
            .map_err(|v| miette!(v))?;

        Ok(disasm
            .iter()
            .map(|v| format!("{} {}", v.mnemonic().unwrap(), v.op_str().unwrap()))
            .collect::<Vec<String>>()
            .join("\n"))
    }

    fn clean(self) {
        self.dlclose_all();
    }
}
