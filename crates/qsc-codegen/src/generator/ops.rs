use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_module::Module;
use miette::Result;
use parking_lot::RwLock;

use crate::context::{CodegenContext, CompilerContext};
use qsc_ast::ast::expr::{binary::BinaryExpr, operator::Operator};

use super::Backend;

pub trait OperationCompiler<'a, 'b, M: Module>: Backend<'a, 'b, M> {
    fn compile_binary_expr(
        cctx: &RwLock<CompilerContext<'a, M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        expr: BinaryExpr<'a>,
    ) -> Result<Value>;
}

impl<'a, 'b, M: Module, T: Backend<'a, 'b, M>> OperationCompiler<'a, 'b, M> for T {
    fn compile_binary_expr(
        cctx: &RwLock<CompilerContext<'a, M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        expr: BinaryExpr<'a>,
    ) -> Result<Value> {
        let left = Self::compile(cctx, ctx, expr.lhs)?;
        let right = Self::compile(cctx, ctx, expr.rhs)?;
        let mut bctx = ctx.builder.write();

        match expr.operator {
            Operator::Add => Ok(bctx.ins().iadd(left, right)),
            Operator::Subtract => Ok(bctx.ins().isub(left, right)),
            Operator::Multiply => Ok(bctx.ins().imul(left, right)),
            Operator::Divide => Ok(bctx.ins().fdiv(left, right)),

            _ => todo!("This operation is not implemented yet!"),
        }
    }
}
