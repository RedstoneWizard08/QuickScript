use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_module::Module;
use miette::Result;
use parking_lot::{RwLock, RwLockWriteGuard};

use crate::context::{CodegenContext, CompilerContext};
use qsc_ast::ast::stmt::cond::ConditionalNode;

use super::Backend;

pub trait ConditionalCompiler<'a, 'b, M: Module>: Backend<'a, 'b, M> {
    fn compile_conditional(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        cond: ConditionalNode,
    ) -> Result<Value>;
}

impl<'a, 'b, M: Module, T: Backend<'a, 'b, M>> ConditionalCompiler<'a, 'b, M> for T {
    fn compile_conditional(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        cond: ConditionalNode,
    ) -> Result<Value> {
        let ptr = Self::ptr(cctx);
        let cond_value = Self::compile(cctx, ctx, cond.condition)?;
        let mut builder = ctx.builder.write();

        let then = builder.create_block();
        let else_ = builder.create_block();
        let merge = builder.create_block();

        builder.append_block_param(merge, ptr);
        builder.ins().brif(cond_value, then, &[], else_, &[]);

        builder.switch_to_block(then);
        builder.seal_block(then);

        let then_ret = builder.ins().iconst(ptr, 0);

        RwLockWriteGuard::unlock_fair(builder);

        for node in &cond.block.data {
            Self::compile(cctx, ctx, node.clone())?;
        }

        let mut builder = ctx.builder.write();

        if !cond
            .block
            .data
            .last()
            .map(|v| v.data.as_stmt().map(|v| v.is_return()).unwrap_or(false))
            .unwrap_or(false)
        {
            builder.ins().jump(merge, &[then_ret]);
        }

        builder.switch_to_block(else_);
        builder.seal_block(else_);

        let else_ret = builder.ins().iconst(ptr, 0);

        RwLockWriteGuard::unlock_fair(builder);

        if let Some(else_block) = cond.else_block {
            for node in &else_block.data {
                Self::compile(cctx, ctx, node.clone())?;
            }
        }

        let mut builder = ctx.builder.write();

        builder.ins().jump(merge, &[else_ret]);
        builder.switch_to_block(merge);
        builder.seal_block(merge);

        let phi = builder.block_params(merge)[0];

        Ok(phi)
    }
}
