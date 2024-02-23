use anyhow::Result;
use cranelift_codegen::ir::{types, GlobalValue, InstBuilder, Type, Value};
use cranelift_module::{DataId, Module};

use qsc_ast::expr::ExprKind;

use self::{
    call::CallCompiler,
    literal::LiteralCompiler,
    ops::OperationCompiler,
    ret::ReturnCompiler,
    unify::BackendInternal,
    vars::{func::FunctionCompiler, var::VariableCompiler},
};

use super::context::{CodegenContext, CompilerContext};

pub mod call;
pub mod literal;
pub mod ops;
pub mod ret;
pub mod unify;
pub mod vars;

pub const RETURN_VAR: &str = "__func_return__";

pub trait Backend<'a, M: Module>: BackendInternal<M> {
    fn query_type(cctx: &mut CompilerContext<'a, M>, ty: String) -> Type;
    fn query_type_with_pointer(ptr: Type, ty: String) -> Type;
    fn ptr(cctx: &mut CompilerContext<'a, M>) -> Type;
    fn null(ctx: &mut CodegenContext) -> Value;
    fn nullptr(cctx: &mut CompilerContext<'a, M>, ctx: &mut CodegenContext) -> Value;

    fn compile(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        expr: ExprKind,
    ) -> Result<Value>;

    fn get_global(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        id: DataId,
    ) -> GlobalValue;
}

impl<'a, M: Module, T: BackendInternal<M>> Backend<'a, M> for T {
    fn query_type(cctx: &mut CompilerContext<'a, M>, ty: String) -> Type {
        Self::query_type_with_pointer(Self::ptr(cctx), ty)
    }

    fn query_type_with_pointer(ptr: Type, ty: String) -> Type {
        match ty.as_str() {
            "i8" | "u8" => types::I8,
            "i16" | "u16" => types::I16,
            "i32" | "u32" => types::I32,
            "i64" | "u64" => types::I64,
            "i128" | "u128" => types::I128,
            "f32" => types::F32,
            "f64" => types::F64,
            "bool" => Type::int(1).unwrap(),
            "char" => types::I32,
            "str" | "ptr" => ptr,

            _ => types::I32,
        }
    }

    fn ptr(cctx: &mut CompilerContext<'a, M>) -> Type {
        cctx.module.target_config().pointer_type()
    }

    fn null(ctx: &mut CodegenContext) -> Value {
        // one null byte
        ctx.builder.ins().null(types::I8)
    }

    fn nullptr(cctx: &mut CompilerContext<'a, M>, ctx: &mut CodegenContext) -> Value {
        let ptr = Self::ptr(cctx);

        ctx.builder.ins().null(ptr)
    }

    fn get_global(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        id: DataId,
    ) -> GlobalValue {
        cctx.module.declare_data_in_func(id, &mut ctx.builder.func)
    }

    fn compile(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        expr: ExprKind,
    ) -> Result<Value> {
        debug!("Trying to compile: {:?}", expr);

        let res = match expr {
            ExprKind::None => Ok(Self::null(ctx)),

            ExprKind::Literal(literal) => Self::compile_literal(cctx, ctx, literal),
            ExprKind::Call(call) => Self::compile_call(cctx, ctx, call),
            ExprKind::Eof => Ok(Self::null(ctx)),
            ExprKind::Identifer(ident) => Self::compile_named_var(ctx, ident),
            ExprKind::Operation(op) => Self::compile_op(cctx, ctx, op),
            ExprKind::Return(ret) => Self::compile_return(cctx, ctx, ret.map(|v| *v.clone())),
            ExprKind::Type(_, _) => Ok(Self::null(ctx)),

            ExprKind::Unary(negative, val) => Ok(if negative {
                // ctx.builder.ins().neg()
                todo!()
            } else {
                Self::compile(cctx, ctx, *val)?
            }),

            ExprKind::Variable(var) => Self::compile_var(cctx, ctx, var),
            ExprKind::Function(func) => Self::compile_fn(cctx, ctx, func),

            ExprKind::Block(block) => {
                let mut res = Self::null(ctx);

                for expr in block {
                    res = Self::compile(cctx, ctx, expr.content)?;
                }

                Ok(res)
            }
        };

        debug!("Compiled: {:?}", res);

        res
    }
}
