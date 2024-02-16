use std::any::Any;

use anyhow::Result;
use cranelift_codegen::write_function;
use cranelift_module::Module;
use cranelift_object::ObjectProduct;
use target_lexicon::Triple;

use crate::ast::var::FunctionData;

use super::{aot::AotGenerator, jit::JitGenerator};

pub trait CodegenBackend {
    fn new(triple: Triple) -> Result<Self>
    where
        Self: Sized;
    fn compile(&mut self, funcs: Vec<FunctionData>) -> Result<()>;
    fn is_jit(&self) -> bool;
    fn run(&self) -> Result<i32>;
    fn finalize(self) -> ObjectProduct;
    fn as_any(&self) -> &dyn Any;
    fn clif(&self) -> Result<String>;
    fn vcode(&self) -> String;
    fn asm(self) -> Result<String>;
}

impl CodegenBackend for AotGenerator {
    fn new(triple: Triple) -> Result<Self> {
        Ok(Self::new(triple)?)
    }

    fn compile(&mut self, funcs: Vec<FunctionData>) -> Result<()> {
        for func in funcs {
            self.compile_function(func)?;
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clif(&self) -> Result<String> {
        let mut buf = String::new();
        let isa = self.module.isa();

        for func in self.fns.clone() {
            write_function(&mut buf, &func)?;
        }

        write_function(&mut buf, &self.ctx.func)?;

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
        let capstone = self.module.isa().to_capstone().map_err(|v| anyhow!(v))?;
        let product = self.finalize();
        let data = product.object.write()?;

        let disasm = capstone
            .disasm_all(&*data.into_boxed_slice(), 0x0)
            .map_err(|v| anyhow!(v))?;

        Ok(disasm
            .iter()
            .map(|v| format!("{} {}", v.mnemonic().unwrap(), v.op_str().unwrap()))
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

impl CodegenBackend for JitGenerator {
    fn new(triple: Triple) -> Result<Self> {
        Ok(Self::new(triple)?)
    }

    fn compile(&mut self, funcs: Vec<FunctionData>) -> Result<()> {
        for func in funcs {
            self.compile_function(func)?;
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clif(&self) -> Result<String> {
        let mut buf = String::new();
        let isa = self.module.isa();

        for func in self.fns.clone() {
            write_function(&mut buf, &func)?;
        }

        write_function(&mut buf, &self.ctx.func)?;

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
