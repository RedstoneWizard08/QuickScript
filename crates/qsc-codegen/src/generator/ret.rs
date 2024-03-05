use anyhow::Result;
use cranelift_codegen::{entity::EntityRef, ir::Value};
use cranelift_frontend::Variable;
use cranelift_module::Module;

use crate::context::{CodegenContext, CompilerContext};
use qsc_ast::ast::stmt::{call::{CallArgument, CallNode}, ret::ReturnNode};

use super::{Backend, CallCompiler, RETURN_VAR};

pub trait ReturnCompiler<'i, 'a, M: Module>: Backend<'i, 'a, M> {
    fn compile_return(
        cctx: &mut CompilerContext<'i, 'a, M>,
        ctx: &mut CodegenContext,
        node: ReturnNode<'i>,
    ) -> Result<Value>;
}

impl<'i, 'a, M: Module, T: Backend<'i, 'a, M>> ReturnCompiler<'i, 'a, M> for T {
    fn compile_return(
        cctx: &mut CompilerContext<'i, 'a, M>,
        ctx: &mut CodegenContext,
        node: ReturnNode<'i>,
    ) -> Result<Value> {
        if let Some(value) = node.value {
            if ctx.func.name == "main" || ctx.func.name == "_start" {
                // main or _start need to exit instead of returning

                Self::compile_call(
                    cctx,
                    ctx,
                    CallNode {
                        span: node.span,
                        func: "exit",
                        args: vec![CallArgument {
                            span: node.span,
                            value: value.clone(),
                        }],
                    },
                )?;
            }

            let val = Self::compile(cctx, ctx, value)?;
            let ty = Self::query_type(cctx, ctx.ret.clone().map(|v| v.as_str()).unwrap_or(String::new()));
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
