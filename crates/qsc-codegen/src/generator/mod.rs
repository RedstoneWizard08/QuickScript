use cranelift_codegen::ir::{types, GlobalValue, InstBuilder, Type, Value};
use cranelift_module::{DataId, Module};
use miette::Result;
use parking_lot::RwLock;
use qsc_ast::ast::{
    decl::DeclarationNode,
    expr::{unary::UnaryExpr, ExpressionNode},
    node::{data::NodeData, Node},
    stmt::StatementNode,
};

use crate::alias::DeclareAliasedFunction;

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

pub trait Backend<'a, 'b, M: Module>: BackendInternal<'a, M> {
    fn query_type(cctx: &RwLock<CompilerContext<'a, M>>, ty: String) -> Type;
    fn query_type_with_pointer(ptr: Type, ty: String) -> Type;
    fn ptr(cctx: &RwLock<CompilerContext<'a, M>>) -> Type;
    fn null(ctx: &mut CodegenContext<'a, 'b>) -> Value;
    fn nullptr(cctx: &RwLock<CompilerContext<'a, M>>, ctx: &mut CodegenContext<'a, 'b>) -> Value;

    fn compile(
        cctx: &RwLock<CompilerContext<'a, M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        node: Node<'a>,
    ) -> Result<Value>;

    fn get_global(
        cctx: &RwLock<CompilerContext<'a, M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        id: DataId,
    ) -> GlobalValue;
}

impl<'a, 'b, M: Module + DeclareAliasedFunction, T: BackendInternal<'a, M>> Backend<'a, 'b, M>
    for T
{
    fn query_type(cctx: &RwLock<CompilerContext<'a, M>>, ty: String) -> Type {
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
            "str" | "ptr" | _ => ptr,
        }
    }

    fn ptr(cctx: &RwLock<CompilerContext<'a, M>>) -> Type {
        cctx.read().module.target_config().pointer_type()
    }

    fn null(ctx: &mut CodegenContext<'a, 'b>) -> Value {
        // one null byte
        ctx.builder.write().ins().null(types::I8)
    }

    fn nullptr(cctx: &RwLock<CompilerContext<'a, M>>, ctx: &mut CodegenContext<'a, 'b>) -> Value {
        let ptr = Self::ptr(cctx);

        ctx.builder.write().ins().null(ptr)
    }

    fn get_global(
        cctx: &RwLock<CompilerContext<'a, M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        id: DataId,
    ) -> GlobalValue {
        cctx.write()
            .module
            .declare_data_in_func(id, &mut ctx.builder.write().func)
    }

    fn compile(
        cctx: &RwLock<CompilerContext<'a, M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        node: Node<'a>,
    ) -> Result<Value> {
        debug!("Trying to compile: {:?}", node);

        let res = match *node.data {
            NodeData::Literal(literal) => Self::compile_literal(cctx, ctx, literal),
            NodeData::Symbol(symbol) => Self::compile_named_var(cctx, ctx, symbol.value),
            NodeData::Type(_) | NodeData::EOI => Ok(Self::null(ctx)),

            NodeData::Expr(expr) => match expr {
                ExpressionNode::Unary(UnaryExpr {
                    negative,
                    value,
                    span: _,
                }) => Ok(if negative {
                    let val = Self::compile(cctx, ctx, value)?;

                    ctx.builder.write().ins().ineg(val)
                } else {
                    Self::compile(cctx, ctx, value)?
                }),

                ExpressionNode::Binary(op) => Self::compile_binary_expr(cctx, ctx, op),
            },

            NodeData::Statement(stmt) => match stmt {
                StatementNode::Call(call) => Self::compile_call(cctx, ctx, call),
                StatementNode::Return(ret) => Self::compile_return(cctx, ctx, ret),
            },

            NodeData::Declaration(decl) => match decl {
                DeclarationNode::Variable(var) => Self::compile_var(cctx, ctx, var),
                DeclarationNode::Function(func) => Self::compile_fn(cctx, ctx, &func),
                DeclarationNode::Global(_global) => unimplemented!(),
            },

            NodeData::Block(block) => {
                let mut res = Self::null(ctx);

                for node in block.data {
                    res = Self::compile(cctx, ctx, node)?;
                }

                Ok(res)
            }
        };

        debug!("Compiled: {:?}", res);

        res
    }
}
