use std::sync::{Arc, RwLock};

use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_module::Module;
use miette::Result;

use crate::context::{CodegenContext, CompilerContext};
use qsc_ast::ast::expr::{binary::BinaryExpr, operator::Operator};

use super::Backend;

pub trait OperationCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_binary_expr(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
        ctx: &mut CodegenContext<'a>,
        expr: BinaryExpr<'a>,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> OperationCompiler<'a, M> for T {
    fn compile_binary_expr(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
        ctx: &mut CodegenContext<'a>,
        expr: BinaryExpr<'a>,
    ) -> Result<Value> {
        let left = Self::compile(cctx.clone(), ctx, expr.lhs)?;
        let right = Self::compile(cctx, ctx, expr.rhs)?;
        let mut bctx = ctx.builder.write().unwrap();

        match expr.operator {
            Operator::Add => Ok(bctx.ins().iadd(left, right)),
            Operator::Subtract => Ok(bctx.ins().isub(left, right)),
            Operator::Multiply => Ok(bctx.ins().imul(left, right)),
            Operator::Divide => Ok(bctx.ins().fdiv(left, right)),

            _ => todo!("This operation is not implemented yet!"),
        }
    }
}
