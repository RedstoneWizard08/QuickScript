use std::sync::{Arc, RwLock};

use cranelift_codegen::ir::{types, InstBuilder, Type, Value};
use cranelift_module::{Linkage, Module};
use miette::{IntoDiagnostic, Result};
use qsc_ast::ast::literal::{
    boolean::BoolNode, char::CharNode, float::FloatNode, int::IntNode, string::StringNode,
    LiteralNode,
};
use qsc_core::util::random_string;

use crate::context::{CodegenContext, CompilerContext};

use super::Backend;

pub trait LiteralCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_bool(ctx: &mut CodegenContext<'a>, value: BoolNode<'a>) -> Value;
    fn compile_int(ctx: &mut CodegenContext<'a>, value: IntNode<'a>) -> Value;
    fn compile_float(ctx: &mut CodegenContext<'a>, value: FloatNode<'a>) -> Value;
    fn compile_char(ctx: &mut CodegenContext<'a>, value: CharNode<'a>) -> Value;

    fn compile_string(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
        ctx: &mut CodegenContext<'a>,
        value: StringNode<'a>,
    ) -> Result<Value>;

    fn compile_literal(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
        ctx: &mut CodegenContext<'a>,
        node: LiteralNode<'a>,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> LiteralCompiler<'a, M> for T {
    fn compile_literal(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
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
            .write()
            .unwrap()
            .ins()
            .iconst(Type::int(1).unwrap(), i64::from(value.value))
    }

    fn compile_int(ctx: &mut CodegenContext<'a>, value: IntNode<'a>) -> Value {
        ctx.builder
            .write()
            .unwrap()
            .ins()
            .iconst(types::I32, value.value)
    }

    fn compile_float(ctx: &mut CodegenContext<'a>, value: FloatNode<'a>) -> Value {
        ctx.builder.write().unwrap().ins().f64const(value.value)
    }

    fn compile_string(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
        ctx: &mut CodegenContext<'a>,
        value: StringNode<'a>,
    ) -> Result<Value> {
        let ddesc = cctx.read().unwrap().data_desc.clone();
        let mut bctx = ctx.builder.write().unwrap();
        let mut wctx = cctx.write().unwrap();

        wctx.data_desc.define(
            format!("{}\0", value.value)
                .as_bytes()
                .to_vec()
                .into_boxed_slice(),
        );

        let name = format!("literal_string_{}", random_string(10));

        let id = wctx
            .module
            .declare_data(&name, Linkage::Export, true, false)
            .into_diagnostic()?;

        wctx.module.define_data(id, &ddesc).into_diagnostic()?;

        wctx.data_desc.clear();

        let local_id = wctx.module.declare_data_in_func(id, bctx.func);

        Ok(bctx.ins().global_value(Self::ptr(cctx.clone()), local_id))
    }

    fn compile_char(ctx: &mut CodegenContext<'a>, value: CharNode<'a>) -> Value {
        ctx.builder
            .write()
            .unwrap()
            .ins()
            .iconst(types::I32, i64::from(value.value as u32))
    }
}
