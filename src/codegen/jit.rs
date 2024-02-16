use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use anyhow::Result;
use cranelift_codegen::{
    settings::{self, Configurable, Flags},
    Context,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{default_libcall_names, DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_native::builder;
use qsc_cranelift_jit::{JITBuilder, JITModule};

use crate::ast::var::FunctionData;

pub struct JitGenerator<'a> {
    pub builder_ctx: FunctionBuilderContext,
    pub ctx: Context,
    pub data_desc: DataDescription,
    pub module: JITModule,
    pub functions: HashMap<String, FunctionData>,
    pub builder: Option<Rc<RefCell<FunctionBuilder<'a>>>>,
    pub globals: HashMap<String, DataId>,
    pub locals: HashMap<String, DataId>,
    pub vars: HashMap<String, Variable>,
}

impl<'a> JitGenerator<'a> {
    pub fn new() -> Result<Self> {
        let mut flags = settings::builder();

        flags.set("use_colocated_libcalls", "false")?;
        flags.set("is_pic", "false")?;

        let isa = builder().map_err(|msg| anyhow!("Host machine is not supported: {}", msg))?;
        let isa = isa.finish(Flags::new(flags))?;

        let builder = JITBuilder::with_isa(isa, default_libcall_names());
        let module = JITModule::new(builder);

        Ok(Self {
            builder_ctx: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_desc: DataDescription::new(),
            module,
            functions: HashMap::new(),
            builder: None,
            globals: HashMap::new(),
            locals: HashMap::new(),
            vars: HashMap::new(),
        })
    }

    pub(crate) fn internal_finalize_builder(&mut self) -> Result<()> {
        if self.builder.is_some() {
            unsafe {
                Rc::try_unwrap(self.builder.clone().unwrap())
                    .unwrap_unchecked()
                    .into_inner()
                    .finalize();
            }

            self.builder = None;
        }

        Ok(())
    }

    pub(crate) fn internal_complete_define_func(&mut self, id: FuncId) -> Result<()> {
        self.module.define_function(id, &mut self.ctx)?;
        self.module.clear_context(&mut self.ctx);

        Ok(())
    }

    pub(crate) fn internal_declare_func(&mut self, name: &str, linkage: Linkage) -> Result<FuncId> {
        Ok(self
            .module
            .declare_function(&name, linkage, &self.ctx.func.signature)?)
    }

    pub(crate) fn internal_new_builder(&mut self) {
        let b = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);
        let c = RefCell::new(b);

        self.builder = Some(Rc::new(c));
    }
}
