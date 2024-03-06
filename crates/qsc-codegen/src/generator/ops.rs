use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_module::Module;
use miette::Result;

use crate::context::{CodegenContext, CompilerContext};
use qsc_ast::ast::expr::{binary::BinaryExpr, operator::Operator};

use super::Backend;

pub trait OperationCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_binary_expr(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        expr: BinaryExpr<'a>,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> OperationCompiler<'a, M> for T {
    fn compile_binary_expr(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        expr: BinaryExpr<'a>,
    ) -> Result<Value> {
        let left = Self::compile(cctx, ctx, expr.lhs)?;
        let right = Self::compile(cctx, ctx, expr.rhs)?;

        match expr.operator {
            Operator::Add => Ok(ctx.builder.ins().iadd(left, right)),
            Operator::Subtract => Ok(ctx.builder.ins().isub(left, right)),
            Operator::Multiply => Ok(ctx.builder.ins().imul(left, right)),
            Operator::Divide => Ok(ctx.builder.ins().fdiv(left, right)),

            _ => todo!("This operation is not implemented yet!"),
        }
    }
}
