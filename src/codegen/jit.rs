use std::{any::Any, mem::transmute};

use anyhow::Result;
use cranelift_codegen::{
    isa::lookup,
    settings::{builder as flag_builder, Configurable, Flags},
};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{default_libcall_names, DataDescription, DataId, FuncId, Module};
use target_lexicon::Triple;

use super::backend::CraneliftBackend;

impl CraneliftBackend<JITModule> {
    pub fn new(target: Triple, disasm: bool) -> Result<Self> {
        let mut flags = flag_builder();

        flags.set("use_colocated_libcalls", "false")?;
        flags.set("is_pic", "false")?;

        let isa = lookup(target)?.finish(Flags::new(flags))?;
        let builder = JITBuilder::with_isa(isa, default_libcall_names());
        let module = JITModule::new(builder);

        Ok(Self {
            builder_ctx: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_desc: DataDescription::new(),
            module,
            fns: Vec::new(),
            code: Vec::new(),
            disasm,
            is_jit: true,
            bytecode: Vec::new(),
        })
    }

    pub fn run(self) -> Result<i32> {
        let mut main = None;

        for (name, code, _) in self.bytecode {
            if name == "_start" {
                main = Some(code);
            }
        }

        let main = main.ok_or(anyhow!("No main function found!"))?;
        let main = unsafe { transmute::<_, fn() -> i32>(main) };

        Ok(main())
    }
}

pub trait JITFinish {
    fn jit_finish_fn(&mut self, name: String, id: FuncId) -> Result<()>;
    fn jit_finish_data(&mut self, id: DataId) -> Result<()>;
}

impl<T> JITFinish for CraneliftBackend<T>
where
    T: Module + 'static,
{
    fn jit_finish_fn(&mut self, name: String, id: FuncId) -> Result<()> {
        if self.is_jit {
            let module = &mut self.module as &mut dyn Any;
            let module = module.downcast_mut::<JITModule>().unwrap();

            module.finalize_definitions()?;

            let (code, len) = module.get_finalized_function(id);

            self.bytecode.push((name, code, len));
        }

        Ok(())
    }

    fn jit_finish_data(&mut self, id: DataId) -> Result<()> {
        if self.is_jit {
            let module = &mut self.module as &mut dyn Any;
            let module = module.downcast_mut::<JITModule>().unwrap();

            module.finalize_definitions()?;

            let (code, len) = module.get_finalized_data(id);

            self.bytecode.push((String::new(), code, len));
        }

        Ok(())
    }
}
