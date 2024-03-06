use std::cell::Cell;

use super::{aot::AotGenerator, jit::JitGenerator};
use cranelift_codegen::write_function;
use cranelift_module::Module;
use cranelift_object::ObjectProduct;
use miette::{IntoDiagnostic, Result};
use qsc_ast::ast::AbstractTree;
use target_lexicon::Triple;

pub trait CodegenBackend<'a> {
    fn new(triple: Triple) -> Result<Self>
    where
        Self: Sized;

    fn compile(&'a mut self, tree: AbstractTree<'a>) -> Result<()>;
    fn is_jit(&self) -> bool;
    fn run(&self) -> Result<i32>;
    fn finalize(self) -> ObjectProduct;
    fn clif(&self) -> Result<String>;
    fn vcode(&self) -> String;
    fn asm(self) -> Result<String>;
}

impl<'a> CodegenBackend<'a> for AotGenerator<'a> {
    fn new(triple: Triple) -> Result<Self> {
        Ok(Self::new(triple)?)
    }

    fn compile(&'a mut self, tree: AbstractTree<'a>) -> Result<()> {
        let me = Cell::new(self);

        for node in tree.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(func) = decl.as_function() {
                    unsafe {
                        let me_ref = me.as_ptr().as_mut().unwrap();

                        me_ref.compile_function(func)?;
                    }
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
        self.module.finish()
    }

    fn clif(&self) -> Result<String> {
        let mut buf = String::new();
        let isa = self.module.isa();

        for func in self.fns.clone() {
            write_function(&mut buf, &func).into_diagnostic()?;
        }

        write_function(&mut buf, &self.ctx.func).into_diagnostic()?;

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
        self.vcode
            .iter()
            .map(|v| v.vcode.clone())
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn asm(self) -> Result<String> {
        let capstone = self.module.isa().to_capstone().map_err(|v| miette!(v))?;
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
}

impl<'a> CodegenBackend<'a> for JitGenerator<'a> {
    fn new(triple: Triple) -> Result<Self> {
        Ok(Self::new(triple)?)
    }

    fn compile(&'a mut self, tree: AbstractTree<'a>) -> Result<()> {
        let me = Cell::new(self);

        for node in tree.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(func) = decl.as_function() {
                    unsafe {
                        let me_ref = me.as_ptr().as_mut().unwrap();

                        me_ref.compile_function(func)?;
                    }
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
        let mut buf = String::new();
        let isa = self.module.isa();

        for func in self.fns.clone() {
            write_function(&mut buf, &func).into_diagnostic()?;
        }

        write_function(&mut buf, &self.ctx.func).into_diagnostic()?;

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
        self.vcode
            .iter()
            .map(|v| v.vcode.clone())
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn asm(self) -> Result<String> {
        let capstone = self.module.isa().to_capstone().unwrap();
        let product = self.finalize();
        let data = product.emit().unwrap();
        let disasm = capstone.disasm_all(&*data.into_boxed_slice(), 0x0).unwrap();

        Ok(disasm
            .iter()
            .map(|v| format!("{} {}", v.mnemonic().unwrap(), v.op_str().unwrap()))
            .collect::<Vec<String>>()
            .join("\n"))
    }
}
