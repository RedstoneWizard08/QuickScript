use anyhow::Result;
use cranelift_codegen::ir::{types, InstBuilder, Type, Value};
use cranelift_module::{Linkage, Module};
use qsc_ast::ast::literal::{boolean::BoolNode, char::CharNode, float::FloatNode, int::IntNode, string::StringNode, LiteralNode};
use qsc_core::util::random_string;

use crate::context::{CodegenContext, CompilerContext};

use super::Backend;

pub trait LiteralCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_bool(ctx: &mut CodegenContext<'a>, value: BoolNode<'a>) -> Value;
    fn compile_int(ctx: &mut CodegenContext<'a>, value: IntNode<'a>) -> Value;
    fn compile_float(ctx: &mut CodegenContext<'a>, value: FloatNode<'a>) -> Value;
    fn compile_char(ctx: &mut CodegenContext<'a>, value: CharNode<'a>) -> Value;

    fn compile_string(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        value: StringNode<'a>,
    ) -> Result<Value>;

    fn compile_literal(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        node: LiteralNode<'a>,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> LiteralCompiler<'a, M> for T {
    fn compile_literal(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        expr: LiteralNode<'a>,
    ) -> Result<Value> {
        Ok(match expr {
            LiteralNode::Bool(b) => Self::compile_bool(ctx, b),
            LiteralNode::Int(i) => Self::compile_int(ctx, i),
            LiteralNode::Float(f) => Self::compile_float(ctx, f),
            LiteralNode::String(s) => Self::compile_string(cctx, ctx, s)?,
            LiteralNode::Char(c) => Self::compile_char(ctx, c),
        })
    }

    fn compile_bool(ctx: &mut CodegenContext<'a>, value: BoolNode<'a>) -> Value {
        ctx.builder
            .ins()
            .iconst(Type::int(1).unwrap(), i64::from(value.value))
    }

    fn compile_int(ctx: &mut CodegenContext<'a>, value: IntNode<'a>) -> Value {
        ctx.builder.ins().iconst(types::I32, i64::from(value.value))
    }

    fn compile_float(ctx: &mut CodegenContext<'a>, value: FloatNode<'a>) -> Value {
        ctx.builder.ins().f64const(value.value)
    }

    fn compile_string(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        value: StringNode<'a>,
    ) -> Result<Value> {
        cctx.data_desc.define(
            format!("{}\0", value.value)
                .as_bytes()
                .to_vec()
                .into_boxed_slice(),
        );

        let name = format!("literal_string_{}", random_string(10));

        let id = cctx
            .module
            .declare_data(&name, Linkage::Export, true, false)?;

        cctx.module.define_data(id, &cctx.data_desc)?;
        cctx.data_desc.clear();

        let local_id = cctx.module.declare_data_in_func(id, ctx.builder.func);

        Ok(ctx.builder.ins().global_value(Self::ptr(cctx), local_id))
    }

    fn compile_char(ctx: &mut CodegenContext<'a>, value: CharNode<'a>) -> Value {
        ctx.builder
            .ins()
            .iconst(types::I32, i64::from(value.value as u32))
    }
}
