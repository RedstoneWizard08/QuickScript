use std::{collections::HashMap, sync::Arc};

use cranelift_codegen::{
    ir::{AbiParam, Function},
    isa::lookup,
    settings::{self, Configurable, Flags},
    Context,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{default_libcall_names, DataDescription, DataId, Linkage, Module};
use miette::{IntoDiagnostic, NamedSource, Result};
use parking_lot::RwLock;
use qsc_ast::ast::{decl::func::FunctionNode, AbstractTree};
use qsc_jit::{JITBuilder, JITModule};
use target_lexicon::Triple;

use crate::lookup::lookup_symbol;

use super::{
    context::{CodegenContext, CompilerContext},
    generator::{unify::BackendInternal, vars::func::FunctionCompiler, Backend},
};

pub struct JitGenerator {
    pub ctx: RwLock<CompilerContext<JITModule>>,
    pub builder_ctx: FunctionBuilderContext,
}

impl JitGenerator {
    pub fn new(
        triple: Triple,
        name: String,
        source: String,
        tree: AbstractTree,
        libs: Vec<String>,
    ) -> Result<Self> {
        let mut flags = settings::builder();

        flags
            .set("use_colocated_libcalls", "false")
            .into_diagnostic()?;

        flags.set("is_pic", "false").into_diagnostic()?;

        let isa = lookup(triple)
            .into_diagnostic()?
            .finish(Flags::new(flags))
            .into_diagnostic()?;

        let map = Arc::new(RwLock::new(HashMap::new()));
        let mut builder = JITBuilder::with_isa(isa, default_libcall_names());

        builder.symbol_lookup_fn(lookup_symbol(Arc::clone(&map)));

        for lib in libs {
            builder.lib(lib);
        }

        let module = JITModule::new(builder);

        let ctx = CompilerContext {
            ctx: module.make_context(),
            data_desc: DataDescription::new(),
            module,
            functions: HashMap::new(),
            globals: HashMap::new(),
            code: map,
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

    pub fn compile_function(&mut self, func: FunctionNode) -> Result<()> {
        self.setup_function(&func)?;
        self.compile_function_code(&func)?;
        self.finalize_funciton(func)?;

        Ok(())
    }

    pub fn setup_function(&mut self, func: &FunctionNode) -> Result<()> {
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

        self.ctx
            .write()
            .module
            .finalize_definitions()
            .into_diagnostic()?;

        let (code, size) = self.ctx.read().module.get_finalized_function(id);

        self.ctx
            .write()
            .code
            .write()
            .insert(func.name.to_string(), (func.name.to_string(), code, size));

        debug!("Compiled function: {}", func.name);

        Ok(())
    }

    pub fn exec(&self) -> Result<i32> {
        let mut main = None;

        for (name, code, _) in self.ctx.read().code.read().values() {
            if name == "main" {
                main = Some(unsafe { std::mem::transmute::<_, fn() -> i32>(*code) });

                debug!("Found main function!");
            }
        }

        debug!("Executing main function...");

        if let Some(main) = main {
            let res = main();

            Ok(res)
        } else {
            Err(miette!("No main function found"))
        }
    }

    pub fn dlclose_all(self) {
        RwLock::into_inner(self.ctx).module.dlclose_all();
    }
}

impl BackendInternal<JITModule> for JitGenerator {
    fn post_define(cctx: &RwLock<CompilerContext<JITModule>>, id: DataId) -> Result<()> {
        let mut wctx = cctx.write();

        wctx.module.finalize_definitions().into_diagnostic()?;

        let (code, size) = wctx.module.get_finalized_data(id);

        wctx.code
            .write()
            .insert(String::new(), (String::new(), code, size));

        Ok(())
    }

    fn is_jit() -> bool {
        true
    }
}
