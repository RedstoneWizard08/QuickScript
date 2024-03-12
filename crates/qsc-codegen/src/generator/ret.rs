use cranelift_codegen::{
    entity::EntityRef,
    ir::{InstBuilder, Value},
};
use cranelift_frontend::Variable;
use cranelift_module::Module;
use miette::Result;
use parking_lot::RwLock;

use crate::{
    alias::DeclareAliasedFunction,
    context::{CodegenContext, CompilerContext},
};
use qsc_ast::ast::stmt::{
    call::{CallArgument, CallNode},
    ret::ReturnNode,
};

use super::{Backend, CallCompiler, RETURN_VAR};

pub trait ReturnCompiler<'a, 'b, M: Module>: Backend<'a, 'b, M> {
    fn compile_return(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        node: ReturnNode,
    ) -> Result<Value>;
}

impl<'a, 'b, M: Module + DeclareAliasedFunction, T: Backend<'a, 'b, M>> ReturnCompiler<'a, 'b, M>
    for T
{
    fn compile_return(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        node: ReturnNode,
    ) -> Result<Value> {
        if let Some(value) = node.value {
            if (ctx.func.name == "main" || ctx.func.name == "_start") && !Self::is_jit() {
                // main or _start need to exit instead of returning

                Self::compile_call(
                    cctx,
                    ctx,
                    CallNode {
                        span: node.span.clone(),
                        func: "exit".to_string(),
                        args: vec![CallArgument {
                            span: node.span.clone(),
                            value: value.clone(),
                        }],
                    },
                )?;
            }

            let val = Self::compile(cctx, ctx, value)?;

            let ty = Self::query_type(
                cctx,
                ctx.ret.clone().map(|v| v.as_str()).unwrap_or(String::new()),
            );

            let ref_ = Variable::new(ctx.vars.len());
            let mut bctx = ctx.builder.write();

            bctx.declare_var(ref_, ty);
            bctx.def_var(ref_, val);

            ctx.vars
                .insert(RETURN_VAR.to_string(), (ref_, ctx.ret.clone()));

            let val = bctx.use_var(ref_);

            bctx.ins().return_(&[val]);

            Ok(val)
        } else {
            ctx.builder.write().ins().return_(&[]);

            Ok(Self::null(ctx))
        }
    }
}
