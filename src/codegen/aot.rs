use anyhow::Result;
use cranelift_codegen::settings::{builder as flag_builder, Configurable, Flags};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_module::{default_libcall_names, DataDescription, Module};
use cranelift_native::builder as isa_builder;
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};

use super::backend::CraneliftBackend;

impl CraneliftBackend<ObjectModule> {
    pub fn new() -> Result<Self> {
        let mut flags = flag_builder();

        flags.set("use_colocated_libcalls", "false")?;
        flags.set("is_pic", "false")?;

        let isa_builder = isa_builder().map_err(|v| anyhow!(v))?;
        let isa = isa_builder.finish(Flags::new(flags))?;
        let builder = ObjectBuilder::new(isa, "qsc", default_libcall_names())?;
        let module = ObjectModule::new(builder);

        Ok(Self {
            builder_ctx: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_desc: DataDescription::new(),
            module,
            fns: Vec::new(),
        })
    }

    pub fn finalize(self) -> ObjectProduct {
        self.module.finish()
    }
}
