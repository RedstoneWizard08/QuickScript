use cranelift_codegen::{
    ir::{AbiParam, Function},
    isa::lookup,
    settings::{self, Configurable, Flags},
    Context,
};
use miette::{IntoDiagnostic, Result};
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

use crate::{
    context::{CodegenContext, CompilerContext},
    generator::{unify::BackendInternal, vars::func::FunctionCompiler, Backend},
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{default_libcall_names, DataDescription, DataId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};
use qsc_ast::ast::decl::func::FunctionNode;
use target_lexicon::Triple;

pub struct AotGenerator<'a> {
    pub ctx: Arc<RwLock<CompilerContext<'a, ObjectModule>>>,
    pub builder_ctx: FunctionBuilderContext,
}

impl<'a> AotGenerator<'a> {
    pub fn new(triple: Triple, name: String) -> Result<Self> {
        let mut flags = settings::builder();

        flags
            .set("use_colocated_libcalls", "false")
            .into_diagnostic()?;

        flags.set("is_pic", "true").into_diagnostic()?;
        flags.set("opt_level", "speed").into_diagnostic()?;
        flags.set("regalloc_checker", "true").into_diagnostic()?;

        flags
            .set("enable_alias_analysis", "true")
            .into_diagnostic()?;

        flags.set("enable_verifier", "true").into_diagnostic()?;
        flags.set("enable_probestack", "false").into_diagnostic()?;

        let isa = lookup(triple)
            .into_diagnostic()?
            .finish(Flags::new(flags))
            .into_diagnostic()?;

        let builder =
            ObjectBuilder::new(isa, name + ".o", default_libcall_names()).into_diagnostic()?;
        let module = ObjectModule::new(builder);

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
            ctx: Arc::new(RwLock::new(ctx)),
            builder_ctx: FunctionBuilderContext::new(),
        })
    }

    pub fn compile_function(&mut self, mut func: FunctionNode<'a>) -> Result<()> {
        self.setup_function(&mut func)?;
        self.compile_function_code(&func)?;
        self.finalize_funciton(func)?;

        Ok(())
    }

    pub fn setup_function(&mut self, func: &mut FunctionNode<'a>) -> Result<()> {
        if func.name == "main" {
            // Make the linker happy :)
            func.name = "_start";

            debug!("Compiling function: _start (main)");
        } else {
            debug!("Compiling function: {}", func.name);
        }

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
        let cctx = self.ctx.clone();
        let builder;

        {
            let mut ctx = self.ctx.write();

            builder = FunctionBuilder::new(
                unsafe { ((&mut ctx.ctx.func) as *mut Function).as_mut() }.unwrap(),
                unsafe { ((&mut self.builder_ctx) as *mut FunctionBuilderContext).as_mut() }
                    .unwrap(),
            );
        }

        let builder = Arc::new(RwLock::new(builder));

        let ctx = &mut CodegenContext {
            builder: builder.clone(),
            locals: HashMap::new(),
            vars: HashMap::new(),
            values: HashMap::new(),
            ret: func.ret.clone(),
            func: func.clone(),
        };

        Self::compile_fn(cctx, ctx, func)?;

        let builder = Arc::into_inner(builder).unwrap();
        let builder = RwLock::into_inner(builder);

        builder.finalize();

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

        debug!("Compiled function: {}", func.name);

        Ok(())
    }

    pub fn finalize(self) -> ObjectProduct {
        unsafe { Arc::try_unwrap(self.ctx).unwrap_unchecked() }
            .into_inner()
            .module
            .finish()
    }
}

impl<'a> BackendInternal<'a, ObjectModule> for AotGenerator<'a> {
    fn post_define(_cctx: &mut CompilerContext<'a, ObjectModule>, _id: DataId) -> Result<()> {
        Ok(())
    }
}
