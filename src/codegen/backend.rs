use anyhow::Result;
use cranelift_codegen::{ir::Function, write_function, CompiledCode, Context};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_module::{DataDescription, Module};

pub struct CraneliftBackend<T>
where
    T: Module,
{
    pub builder_ctx: FunctionBuilderContext,
    pub ctx: Context,
    pub data_desc: DataDescription,
    pub module: T,
    pub fns: Vec<Function>,
    pub code: Vec<CompiledCode>,
    pub disasm: bool,
    pub is_jit: bool,
    pub bytecode: Vec<(String, *const u8, usize)>,
}

impl<T> CraneliftBackend<T>
where
    T: Module,
{
    pub fn output_clif(&self) -> Result<String> {
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

    pub fn vcode(&self) -> String {
        self.code
            .iter()
            .map(|v| v.vcode.clone())
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
