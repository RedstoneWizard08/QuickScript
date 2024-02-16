use std::collections::HashMap;

use anyhow::Result;
use cranelift_codegen::{
    ir::{AbiParam, Function},
    isa::lookup,
    settings::{self, Configurable, Flags},
    CompiledCode, Context,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{default_libcall_names, DataDescription, DataId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};
use target_lexicon::Triple;

use crate::{
    ast::var::FunctionData,
    codegen::backend::{unify::BackendInternal, vars::func::FunctionCompiler, Backend},
};

use super::context::{CodegenContext, CompilerContext};

pub struct AotGenerator {
    pub builder_ctx: FunctionBuilderContext,
    pub ctx: Context,
    pub data_desc: DataDescription,
    pub module: ObjectModule,
    pub functions: HashMap<String, FunctionData>,
    pub globals: HashMap<String, DataId>,
    pub fns: Vec<Function>,
    pub vcode: Vec<CompiledCode>,

    /// This isn't actually used, but it's required to make a `CompilerContext`
    pub code: Vec<(String, *const u8)>,
}

impl AotGenerator {
    pub fn new(triple: Triple) -> Result<Self> {
        let mut flags = settings::builder();

        flags.set("use_colocated_libcalls", "false")?;
        flags.set("is_pic", "true")?;
        flags.set("opt_level", "speed")?;
        flags.set("regalloc_checker", "true")?;
        flags.set("enable_alias_analysis", "true")?;
        flags.set("enable_verifier", "true")?;
        flags.set("enable_probestack", "false")?;

        let isa = lookup(triple)?.finish(Flags::new(flags))?;
        let builder = ObjectBuilder::new(isa, "qsc", default_libcall_names())?;
        let module = ObjectModule::new(builder);

        Ok(Self {
            builder_ctx: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_desc: DataDescription::new(),
            module,
            functions: HashMap::new(),
            globals: HashMap::new(),
            code: Vec::new(),
            fns: Vec::new(),
            vcode: Vec::new(),
        })
    }

    pub fn create_context<'a>(
        &'a mut self,
        func: FunctionData,
    ) -> (CompilerContext<'a, ObjectModule>, CodegenContext) {
        let builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);

        (
            CompilerContext {
                module: &mut self.module,
                data_desc: &mut self.data_desc,
                functions: &mut self.functions,
                globals: &mut self.globals,
                code: &mut self.code,
            },
            CodegenContext {
                builder,
                locals: HashMap::new(),
                vars: HashMap::new(),
                ret: func.return_type.clone(),
                func,
            },
        )
    }

    pub fn compile_function<'a>(&'a mut self, mut func: FunctionData) -> Result<()> {
        if func.name == "main" {
            // Make ld happy :)
            func.name = "_start".to_string();
        }

        let (mut cctx, ctx) = self.create_context(func.clone());

        for arg in func.args.clone() {
            ctx.builder
                .func
                .signature
                .params
                .push(AbiParam::new(Self::query_type(&mut cctx, arg.type_)));
        }

        ctx.builder
            .func
            .signature
            .returns
            .push(AbiParam::new(Self::query_type(
                &mut cctx,
                func.return_type.clone(),
            )));

        Self::compile_fn(cctx, ctx, func.clone())?;

        let id =
            self.module
                .declare_function(&func.name, Linkage::Export, &self.ctx.func.signature)?;

        self.module.define_function(id, &mut self.ctx)?;
        self.fns.push(self.ctx.func.clone());
        self.vcode.push(self.ctx.compiled_code().unwrap().clone());
        self.module.clear_context(&mut self.ctx);

        Ok(())
    }

    pub fn finalize(self) -> ObjectProduct {
        self.module.finish()
    }
}

impl BackendInternal<ObjectModule> for AotGenerator {
    fn post_define<'a>(_cctx: &mut CompilerContext<'a, ObjectModule>, _id: DataId) -> Result<()> {
        Ok(())
    }
}
