use anyhow::Result;
use cranelift_codegen::{entity::EntityRef, ir::Value};
use cranelift_frontend::Variable;
use cranelift_module::Module;

use crate::context::{CodegenContext, CompilerContext};
use qsc_ast::{call::Call, ret::Return};

use super::{Backend, CallCompiler, RETURN_VAR};

pub trait ReturnCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_return(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        expr: Return,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> ReturnCompiler<'a, M> for T {
    fn compile_return(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        expr: Return,
    ) -> Result<Value> {
        if let Some(data) = expr.data {
            if ctx.func.name == "main" || ctx.func.name == "_start" {
                // main or _start need to exit instead of returning

                Self::compile_call(
                    cctx,
                    ctx,
                    Call {
                        name: "exit".to_string(),
                        args: vec![*data.clone()],
                    },
                )?;
            }

            let val = Self::compile(cctx, ctx, data.content)?;
            let ty = Self::query_type(cctx, ctx.ret.clone());
            let ref_ = Variable::new(ctx.vars.len());

            ctx.builder.declare_var(ref_, ty);
            ctx.builder.def_var(ref_, val);

            ctx.vars
                .insert(RETURN_VAR.to_string(), (ref_, ctx.ret.clone()));

            Ok(ctx.builder.use_var(ref_))
        } else {
            Ok(Self::null(ctx))
        }
    }
}
