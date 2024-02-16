use anyhow::Result;
use cranelift_codegen::ir::{InstBuilder, StackSlotData, StackSlotKind, Value};
use cranelift_module::Module;

use crate::{
    ast::ret::Return,
    codegen::context::{CodegenContext, CompilerContext},
};

use super::{Backend, RETURN_VAR};

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
            let val = Self::compile(cctx, ctx, data.content)?;

            let slot = ctx.builder.create_sized_stack_slot(StackSlotData::new(
                StackSlotKind::ExplicitSlot,
                Self::query_type(cctx, ctx.ret.clone()).bits(),
            ));

            ctx.builder.ins().stack_store(val, slot, 0);

            ctx.vars
                .insert(RETURN_VAR.to_string(), (slot, ctx.ret.clone()));

            Ok(ctx
                .builder
                .ins()
                .stack_load(Self::query_type(cctx, ctx.ret.clone()), slot, 0))
        } else {
            Ok(Self::null(ctx))
        }
    }
}
