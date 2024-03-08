use cranelift_codegen::{
    ir::{AbiParam, Function},
    isa::lookup,
    settings::{self, Configurable, Flags},
    Context,
};
use miette::{IntoDiagnostic, NamedSource, Result};
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

use crate::{
    context::{CodegenContext, CompilerContext},
    generator::{unify::BackendInternal, vars::func::FunctionCompiler, Backend},
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{default_libcall_names, DataDescription, DataId, Linkage, Module};
use qsc_ast::ast::{decl::func::FunctionNode, AbstractTree};
use qsc_object::{ObjectBuilder, ObjectModule, ObjectProduct};
use target_lexicon::Triple;

pub struct AotGenerator {
    pub ctx: RwLock<CompilerContext<ObjectModule>>,
    pub builder_ctx: FunctionBuilderContext,
}

impl AotGenerator {
    pub fn new(triple: Triple, name: String, source: String, tree: AbstractTree) -> Result<Self> {
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

        let builder = ObjectBuilder::new(isa, name.clone() + ".o", default_libcall_names())
            .into_diagnostic()?;
        let module = ObjectModule::new(builder);

        let ctx = CompilerContext {
            ctx: module.make_context(),
            data_desc: DataDescription::new(),
            module,
            functions: HashMap::new(),
            globals: HashMap::new(),
            code: Arc::new(RwLock::new(HashMap::new())),
            fns: Vec::new(),
            vcode: Vec::new(),
            source: NamedSource::new(name, source),
            tree,
        };

        Ok(Self {
            ctx: RwLock::new(ctx),
            builder_ctx: FunctionBuilderContext::new(),
        })
    }

    pub fn compile_function(&mut self, mut func: FunctionNode) -> Result<()> {
        self.setup_function(&mut func)?;
        self.compile_function_code(&func)?;
        self.finalize_funciton(func)?;

        Ok(())
    }

    pub fn setup_function(&mut self, func: &mut FunctionNode) -> Result<()> {
        if func.name == "main" {
            // Make the linker happy :)
            func.name = "_start".to_string();

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

    pub fn compile_function_code(&mut self, func: &FunctionNode) -> Result<()> {
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

        Self::compile_fn(&self.ctx, ctx, func)?;

        builder.into_inner().finalize();

        Ok(())
    }

    pub fn finalize_funciton(&mut self, func: FunctionNode) -> Result<()> {
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

        let code = self.ctx.read().ctx.compiled_code().unwrap().clone();

        self.ctx.write().vcode.push(code);

        {
            let mut ctx = self.ctx.write();
            let ctx_ref = unsafe { ((&mut ctx.ctx) as *mut Context).as_mut() }.unwrap();

            ctx.module.clear_context(ctx_ref);
        }

        debug!("Compiled function: {}", func.name);

        Ok(())
    }

    pub fn finalize(self) -> ObjectProduct {
        self.ctx.into_inner().module.finish()
    }
}

impl<'a> BackendInternal<ObjectModule> for AotGenerator {
    fn post_define(_cctx: &RwLock<CompilerContext<ObjectModule>>, _id: DataId) -> Result<()> {
        Ok(())
    }

    fn is_jit() -> bool {
        false
    }
}
