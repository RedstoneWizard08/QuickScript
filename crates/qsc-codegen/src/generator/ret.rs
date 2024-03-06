use std::sync::{Arc, RwLock};

use cranelift_codegen::{entity::EntityRef, ir::Value};
use cranelift_frontend::Variable;
use cranelift_module::Module;
use miette::Result;

use crate::context::{CodegenContext, CompilerContext};
use qsc_ast::ast::stmt::{
    call::{CallArgument, CallNode},
    ret::ReturnNode,
};

use super::{Backend, CallCompiler, RETURN_VAR};

pub trait ReturnCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_return(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
        ctx: &mut CodegenContext<'a>,
        node: ReturnNode<'a>,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> ReturnCompiler<'a, M> for T {
    fn compile_return(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
        ctx: &mut CodegenContext<'a>,
        node: ReturnNode<'a>,
    ) -> Result<Value> {
        if let Some(value) = node.value {
            if ctx.func.name == "main" || ctx.func.name == "_start" {
                // main or _start need to exit instead of returning

                Self::compile_call(
                    cctx.clone(),
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

            let val = Self::compile(cctx.clone(), ctx, value)?;

            let ty = Self::query_type(
                cctx,
                ctx.ret.clone().map(|v| v.as_str()).unwrap_or(String::new()),
            );

            let ref_ = Variable::new(ctx.vars.len());
            let mut bctx = ctx.builder.write().unwrap();

            bctx.declare_var(ref_, ty);
            bctx.def_var(ref_, val);

            ctx.vars
                .insert(RETURN_VAR.to_string(), (ref_, ctx.ret.clone()));

            Ok(bctx.use_var(ref_))
        } else {
            Ok(Self::null(ctx))
        }
    }
}
