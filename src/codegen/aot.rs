use super::backend::CraneliftBackend;
use anyhow::Result;
use cranelift_codegen::{
    isa::lookup,
    settings::{builder as flag_builder, Configurable, Flags},
};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_module::{default_libcall_names, DataDescription, Module};
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};
use target_lexicon::Triple;

impl CraneliftBackend<ObjectModule> {
    pub fn new(target: Triple, disasm: bool) -> Result<Self> {
        let mut flags = flag_builder();

        flags.set("use_colocated_libcalls", "false")?;
        flags.set("is_pic", "true")?;
        flags.set("opt_level", "speed")?;
        flags.set("regalloc_checker", "true")?;
        flags.set("enable_alias_analysis", "true")?;
        flags.set("enable_verifier", "true")?;
        flags.set("enable_probestack", "false")?;

        let isa = lookup(target)?.finish(Flags::new(flags))?;
        let builder = ObjectBuilder::new(isa, "qsc", default_libcall_names())?;
        let module = ObjectModule::new(builder);

        Ok(Self {
            builder_ctx: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_desc: DataDescription::new(),
            module,
            fns: Vec::new(),
            code: Vec::new(),
            disasm,
            is_jit: false,
            bytecode: Vec::new(),
            watch_mode: false,
        })
    }

    pub fn finalize(self) -> ObjectProduct {
        self.module.finish()
    }

    pub fn asm(self) -> String {
        let capstone = self.module.isa().to_capstone().unwrap();
        let product = self.finalize();
        let data = product.emit().unwrap();
        let disasm = capstone.disasm_all(&*data.into_boxed_slice(), 0x0).unwrap();

        disasm
            .iter()
            .map(|v| format!("{} {}", v.mnemonic().unwrap(), v.op_str().unwrap()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
