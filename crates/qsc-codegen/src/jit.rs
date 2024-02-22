use std::collections::HashMap;

use anyhow::Result;
use cranelift_codegen::{
    ir::{AbiParam, Function},
    isa::lookup,
    settings::{self, Configurable, Flags},
    CompiledCode, Context,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{default_libcall_names, DataDescription, DataId, Linkage, Module};
use target_lexicon::Triple;

use qsc_ast::var::FunctionData;

use super::{
    backend::{unify::BackendInternal, vars::func::FunctionCompiler, Backend},
    context::{CodegenContext, CompilerContext},
};

pub struct JitGenerator {
    pub builder_ctx: FunctionBuilderContext,
    pub ctx: Context,
    pub data_desc: DataDescription,
    pub module: JITModule,
    pub functions: HashMap<String, FunctionData>,
    pub globals: HashMap<String, DataId>,
    pub fns: Vec<Function>,
    pub vcode: Vec<CompiledCode>,
    pub code: Vec<(String, *const u8)>,
}

impl JitGenerator {
    pub fn new(triple: Triple) -> Result<Self> {
        let mut flags = settings::builder();

        flags.set("use_colocated_libcalls", "false")?;
        flags.set("is_pic", "false")?;

        let isa = lookup(triple)?.finish(Flags::new(flags))?;
        let builder = JITBuilder::with_isa(isa, default_libcall_names());
        let module = JITModule::new(builder);

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
    ) -> (CompilerContext<'a, JITModule>, CodegenContext) {
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
                values: HashMap::new(),
                ret: func.return_type.clone(),
                func,
            },
        )
    }

    pub fn compile_function<'a>(&'a mut self, func: FunctionData) -> Result<()> {
        debug!("Compiling function: {}", func.name);

        for arg in func.args.clone() {
            self.ctx
                .func
                .signature
                .params
                .push(AbiParam::new(Self::query_type_with_pointer(
                    self.module.isa().pointer_type(),
                    arg.type_,
                )));
        }

        self.ctx
            .func
            .signature
            .returns
            .push(AbiParam::new(Self::query_type_with_pointer(
                self.module.isa().pointer_type(),
                func.return_type.clone(),
            )));

        let (mut cctx, mut ctx) = self.create_context(func.clone());

        Self::compile_fn(&mut cctx, &mut ctx, func.clone())?;

        ctx.builder.finalize();

        let id =
            self.module
                .declare_function(&func.name, Linkage::Export, &self.ctx.func.signature)?;

        self.module.define_function(id, &mut self.ctx)?;
        self.fns.push(self.ctx.func.clone());
        self.functions.insert(func.name.clone(), func.clone());
        self.vcode.push(self.ctx.compiled_code().unwrap().clone());
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions()?;

        let code = self.module.get_finalized_function(id);

        self.code.push((func.name.clone(), code));

        debug!("Compiled function: {}", func.name);

        Ok(())
    }

    pub fn exec(&self) -> Result<i32> {
        let mut main = None;

        for (name, code) in &self.code {
            if name == "main" {
                main = Some(unsafe { std::mem::transmute::<_, fn() -> i32>(*code) });

                debug!("Found main function!");
            }
        }

        debug!("Executing main function...");

        if let Some(main) = main {
            Ok(main())
        } else {
            Err(anyhow!("No main function found"))
        }
    }
}

impl BackendInternal<JITModule> for JitGenerator {
    fn post_define<'a>(cctx: &mut CompilerContext<'a, JITModule>, id: DataId) -> Result<()> {
        cctx.module.finalize_definitions()?;

        let (code, _) = cctx.module.get_finalized_data(id);

        cctx.code.push((String::new(), code));

        Ok(())
    }
}