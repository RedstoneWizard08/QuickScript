use cranelift_codegen::{
    ir::{AbiParam, Function},
    isa::lookup,
    settings::{self, Configurable, Flags},
    CompiledCode, Context,
};
use miette::{IntoDiagnostic, Result};
use std::{cell::Cell, collections::HashMap};

use crate::generator::{unify::BackendInternal, vars::func::FunctionCompiler, Backend};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{default_libcall_names, DataDescription, DataId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};
use qsc_ast::ast::decl::func::FunctionNode as Func;
use target_lexicon::Triple;

use super::context::{CodegenContext, CompilerContext};

pub struct AotGenerator<'a> {
    pub builder_ctx: FunctionBuilderContext,
    pub ctx: Context,
    pub data_desc: DataDescription,
    pub module: ObjectModule,
    pub functions: HashMap<String, Func<'a>>,
    pub globals: HashMap<String, DataId>,
    pub fns: Vec<Function>,
    pub vcode: Vec<CompiledCode>,

    /// This isn't actually used, but it's required to make a `CompilerContext` because JIT needs it.
    pub code: Vec<(String, *const u8)>,
}

impl<'a> AotGenerator<'a> {
    pub fn new(triple: Triple) -> Result<Self> {
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
        let builder = ObjectBuilder::new(isa, "qsc", default_libcall_names()).into_diagnostic()?;
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

    pub fn create_context(
        &'a mut self,
        func: Func<'a>,
    ) -> (CompilerContext<'a, ObjectModule>, CodegenContext) {
        let builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);

        (
            CompilerContext {
                module: &mut self.module,
                data_desc: &mut self.data_desc,
                functions: &mut self.functions,
                globals: &mut self.globals,
                code: &mut self.code,
                fns: &mut self.fns,
                vcode: &mut self.vcode,
            },
            CodegenContext {
                builder,
                locals: HashMap::new(),
                vars: HashMap::new(),
                values: HashMap::new(),
                ret: func.ret.clone(),
                func,
            },
        )
    }

    pub fn compile_function(&'a mut self, mut func: Func<'a>) -> Result<()> {
        let me = Cell::new(self);

        unsafe {
            let me_ref = me.as_ptr().as_mut().unwrap();

            me_ref.setup_function(&mut func)?;
        }

        unsafe {
            let me_ref = me.as_ptr().as_mut().unwrap();

            me_ref.compile_function_code(&func)?;
        }

        unsafe {
            let me_ref = me.as_ptr().as_mut().unwrap();

            me_ref.finalize_funciton(func)?;
        }

        Ok(())
    }

    pub fn setup_function(&'a mut self, func: &mut Func<'a>) -> Result<()> {
        if func.name == "main" {
            // Make the linker happy :)
            func.name = "_start";

            debug!("Compiling function: _start (main)");
        } else {
            debug!("Compiling function: {}", func.name);
        }

        for arg in func.args.clone() {
            self.ctx
                .func
                .signature
                .params
                .push(AbiParam::new(Self::query_type_with_pointer(
                    self.module.isa().pointer_type(),
                    arg.type_.as_str(),
                )));
        }

        self.ctx
            .func
            .signature
            .returns
            .push(AbiParam::new(Self::query_type_with_pointer(
                self.module.isa().pointer_type(),
                func.ret
                    .clone()
                    .map(|v| v.as_str())
                    .unwrap_or("void".to_string()),
            )));

        Ok(())
    }

    pub fn compile_function_code(&'a mut self, func: &Func<'a>) -> Result<()> {
        let (mut cctx, mut ctx) = self.create_context(func.clone());

        Self::compile_fn(&mut cctx, &mut ctx, func)?;

        ctx.builder.finalize();

        Ok(())
    }

    pub fn finalize_funciton(&'a mut self, func: Func<'a>) -> Result<()> {
        let id = self
            .module
            .declare_function(&func.name, Linkage::Export, &self.ctx.func.signature)
            .into_diagnostic()?;

        self.module
            .define_function(id, &mut self.ctx)
            .into_diagnostic()?;
        self.fns.push(self.ctx.func.clone());
        self.functions.insert(func.name.to_string(), func.clone());
        self.vcode.push(self.ctx.compiled_code().unwrap().clone());
        self.module.clear_context(&mut self.ctx);

        debug!("Compiled function: {}", func.name);

        Ok(())
    }

    pub fn finalize(self) -> ObjectProduct {
        self.module.finish()
    }
}

impl<'a> BackendInternal<'a, ObjectModule> for AotGenerator<'a> {
    fn post_define(_cctx: &mut CompilerContext<'a, ObjectModule>, _id: DataId) -> Result<()> {
        Ok(())
    }
}
