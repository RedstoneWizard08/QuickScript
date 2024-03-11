use cranelift_codegen::ir::{
    condcodes::{FloatCC, IntCC},
    InstBuilder, Value,
};
use cranelift_module::Module;
use miette::Result;
use parking_lot::RwLock;
use qsc_core::{conv::IntoSourceSpan, error::compiler::CompilerError};

use crate::context::{CodegenContext, CompilerContext};
use qsc_ast::ast::expr::{binary::BinaryExpr, operator::Operator};

use super::Backend;

pub trait OperationCompiler<'a, 'b, M: Module>: Backend<'a, 'b, M> {
    fn compile_binary_expr(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        expr: BinaryExpr,
    ) -> Result<Value>;
}

impl<'a, 'b, M: Module, T: Backend<'a, 'b, M>> OperationCompiler<'a, 'b, M> for T {
    fn compile_binary_expr(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        expr: BinaryExpr,
    ) -> Result<Value> {
        let left = Self::compile(cctx, ctx, expr.lhs.clone())?;
        let right = Self::compile(cctx, ctx, expr.rhs.clone())?;
        let tree = cctx.read().tree.clone();

        let fn_name = Some(if ctx.func.name == "_start" {
            "main".to_string()
        } else {
            ctx.func.name.clone()
        });

        let mut bctx = ctx.builder.write();

        if expr.lhs.data.is_int(&fn_name, &tree)? {
            if expr.rhs.data.is_int(&fn_name, &tree)? {
                match expr.operator {
                    Operator::Add => Ok(bctx.ins().iadd(left, right)),
                    Operator::Subtract => Ok(bctx.ins().isub(left, right)),
                    Operator::Multiply => Ok(bctx.ins().imul(left, right)),
                    Operator::Divide => Ok(bctx.ins().fdiv(left, right)),
                    Operator::Equal => Ok(bctx.ins().icmp(IntCC::Equal, left, right)),
                    Operator::NotEqual => Ok(bctx.ins().icmp(IntCC::NotEqual, left, right)),

                    _ => todo!("This operation is not implemented yet!"),
                }
            } else if expr.rhs.data.is_float(&fn_name, &tree)? {
                match expr.operator {
                    Operator::Add => Ok(bctx.ins().fadd(left, right)),
                    Operator::Subtract => Ok(bctx.ins().fsub(left, right)),
                    Operator::Multiply => Ok(bctx.ins().fmul(left, right)),
                    Operator::Divide => Ok(bctx.ins().fdiv(left, right)),
                    Operator::Equal => Ok(bctx.ins().fcmp(FloatCC::Equal, left, right)),
                    Operator::NotEqual => Ok(bctx.ins().fcmp(FloatCC::NotEqual, left, right)),

                    _ => todo!("This operation is not implemented yet!"),
                }
            } else {
                Err(CompilerError {
                    location: expr.span.into_source_span(),
                    src: tree.src.clone().into(),
                    error: miette!(
                        "An integer cannot be converted to a {}!",
                        expr.rhs.data.get_type(&fn_name, &tree)?
                    ),
                }
                .into())
            }
        } else if expr.lhs.data.is_str(&fn_name, &tree)? {
            if expr.rhs.data.is_str(&fn_name, &tree)? {
                match expr.operator {
                    Operator::Equal => Ok(bctx.ins().icmp(IntCC::Equal, left, right)),
                    Operator::NotEqual => Ok(bctx.ins().icmp(IntCC::NotEqual, left, right)),

                    _ => todo!("This operation is not implemented yet!"),
                }
            } else {
                Err(CompilerError {
                    location: expr.span.into_source_span(),
                    src: tree.src.clone().into(),
                    error: miette!(
                        "A string cannot be converted to a {}!",
                        expr.rhs.data.get_type(&fn_name, &tree)?
                    ),
                }
                .into())
            }
        } else if expr.lhs.data.is_char(&fn_name, &tree)? {
            if expr.rhs.data.is_char(&fn_name, &tree)? {
                match expr.operator {
                    Operator::Equal => Ok(bctx.ins().icmp(IntCC::Equal, left, right)),
                    Operator::NotEqual => Ok(bctx.ins().icmp(IntCC::NotEqual, left, right)),

                    _ => todo!("This operation is not implemented yet!"),
                }
            } else {
                Err(CompilerError {
                    location: expr.span.into_source_span(),
                    src: tree.src.clone().into(),
                    error: miette!(
                        "A char cannot be converted to a {}!",
                        expr.rhs.data.get_type(&fn_name, &tree)?
                    ),
                }
                .into())
            }
        } else {
            Err(CompilerError {
                location: expr.span.into_source_span(),
                src: tree.src.clone().into(),
                error: miette!("Cannot compile an operation with unknown or unsupported types!"),
            }
            .into())
        }
    }
}
