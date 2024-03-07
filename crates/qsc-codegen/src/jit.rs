use std::collections::HashMap;

use cranelift_codegen::{
    ir::{AbiParam, Function},
    isa::lookup,
    settings::{self, Configurable, Flags},
    Context,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{default_libcall_names, DataDescription, DataId, Linkage, Module};
use miette::{IntoDiagnostic, Result};
use parking_lot::RwLock;
use target_lexicon::Triple;
use qsc_ast::ast::decl::func::FunctionNode;

use super::{
    context::{CodegenContext, CompilerContext},
    generator::{unify::BackendInternal, vars::func::FunctionCompiler, Backend},
};

pub struct JitGenerator<'a> {
    pub ctx: RwLock<CompilerContext<'a, JITModule>>,
    pub builder_ctx: FunctionBuilderContext,
}

impl<'a> JitGenerator<'a> {
    pub fn new(triple: Triple) -> Result<Self> {
        let mut flags = settings::builder();

        flags
            .set("use_colocated_libcalls", "false")
            .into_diagnostic()?;
        flags.set("is_pic", "false").into_diagnostic()?;

        let isa = lookup(triple)
            .into_diagnostic()?
            .finish(Flags::new(flags))
            .into_diagnostic()?;
        let builder = JITBuilder::with_isa(isa, default_libcall_names());
        let module = JITModule::new(builder);

        let ctx = CompilerContext {
            builder_ctx: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_desc: DataDescription::new(),
            module,
            functions: HashMap::new(),
            globals: HashMap::new(),
            code: Vec::new(),
            fns: Vec::new(),
            vcode: Vec::new(),
        };

        Ok(Self {
            ctx: RwLock::new(ctx),
            builder_ctx: FunctionBuilderContext::new(),
        })
    }

    pub fn compile_function(&mut self, func: FunctionNode<'a>) -> Result<()> {
        self.setup_function(&func)?;
        self.compile_function_code(&func)?;
        self.finalize_funciton(func)?;

        Ok(())
    }

    pub fn setup_function(&mut self, func: &FunctionNode<'a>) -> Result<()> {
        debug!("Compiling function: {}", func.name);

        let ptr = self.ctx.read().module.isa().pointer_type();

        for arg in func.args.clone() {
            self.ctx
                .write()
                .ctx
                .func
                .signature
                .params
                .push(AbiParam::new(Self::query_type_with_pointer(
                    ptr,
                    arg.type_.as_str(),
                )));
        }

        self.ctx
            .write()
            .ctx
            .func
            .signature
            .returns
            .push(AbiParam::new(Self::query_type_with_pointer(
                ptr,
                func.ret
                    .clone()
                    .map(|v| v.as_str())
                    .unwrap_or("void".to_string()),
            )));

        Ok(())
    }

    pub fn compile_function_code(&mut self, func: &FunctionNode<'a>) -> Result<()> {
        let builder;

        {
            let mut ctx = self.ctx.write();

            builder = FunctionBuilder::new(
                unsafe { ((&mut ctx.ctx.func) as *mut Function).as_mut() }.unwrap(),
                unsafe { ((&mut self.builder_ctx) as *mut FunctionBuilderContext).as_mut() }
                    .unwrap(),
            );
        }

        let builder = RwLock::new(builder);

        let ctx = &mut CodegenContext {
            builder: &builder,
            locals: HashMap::new(),
            vars: HashMap::new(),
            values: HashMap::new(),
            ret: func.ret.clone(),
            func: func.clone(),
        };

        debug!("Compiling function: {}", func.name);

        Self::compile_fn(&self.ctx, ctx, func)?;

        debug!("Finalizing function: {}", func.name);

        builder.into_inner().finalize();

        debug!("Completed compilation for function: {}", func.name);

        Ok(())
    }

    pub fn finalize_funciton(&mut self, func: FunctionNode<'a>) -> Result<()> {
        let sig = self.ctx.read().ctx.func.signature.clone();

        let id = self
            .ctx
            .write()
            .module
            .declare_function(&func.name, Linkage::Export, &sig)
            .into_diagnostic()?;

        {
            let mut ctx = self.ctx.write();
            let ctx_ref = unsafe { ((&mut ctx.ctx) as *mut Context).as_mut() }.unwrap();

            ctx.module.define_function(id, ctx_ref).into_diagnostic()?;
        }

        let cg_func = self.ctx.read().ctx.func.clone();

        self.ctx.write().fns.push(cg_func);

        self.ctx
            .write()
            .functions
            .insert(func.name.to_string(), func.clone());

        self.ctx
            .write()
            .vcode
            .push(self.ctx.write().ctx.compiled_code().unwrap().clone());

        self.ctx
            .write()
            .module
            .clear_context(&mut self.ctx.write().ctx);

        self.ctx
            .write()
            .module
            .finalize_definitions()
            .into_diagnostic()?;

        let code = self.ctx.read().module.get_finalized_function(id);

        self.ctx.write().code.push((func.name.to_string(), code));

        debug!("Compiled function: {}", func.name);

        Ok(())
    }

    pub fn exec(&self) -> Result<i32> {
        let mut main = None;

        for (name, code) in &self.ctx.read().code {
            if name == "main" {
                main = Some(unsafe { std::mem::transmute::<_, fn() -> i32>(code) });

                debug!("Found main function!");
            }
        }

        debug!("Executing main function...");

        if let Some(main) = main {
            Ok(main())
        } else {
            Err(miette!("No main function found"))
        }
    }
}

impl<'a> BackendInternal<'a, JITModule> for JitGenerator<'a> {
    fn post_define(cctx: &mut CompilerContext<'a, JITModule>, id: DataId) -> Result<()> {
        cctx.module.finalize_definitions().into_diagnostic()?;

        let (code, _) = cctx.module.get_finalized_data(id);

        cctx.code.push((String::new(), code));

        Ok(())
    }
}
