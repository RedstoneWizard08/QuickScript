use cranelift_codegen::ir::{types, InstBuilder, Value};
use cranelift_module::{Linkage, Module};
use miette::{IntoDiagnostic, Result};
use parking_lot::{RwLock, RwLockWriteGuard};
use qsc_ast::ast::literal::{
    boolean::BoolNode, char::CharNode, float::FloatNode, int::IntNode, string::StringNode,
    LiteralNode,
};
use qsc_core::util::random_string;

use crate::context::{CodegenContext, CompilerContext};

use super::Backend;

pub trait LiteralCompiler<'a, 'b, M: Module>: Backend<'a, 'b, M> {
    fn compile_bool(ctx: &mut CodegenContext<'a, 'b>, value: BoolNode) -> Value;
    fn compile_int(ctx: &mut CodegenContext<'a, 'b>, value: IntNode) -> Value;
    fn compile_float(ctx: &mut CodegenContext<'a, 'b>, value: FloatNode) -> Value;
    fn compile_char(ctx: &mut CodegenContext<'a, 'b>, value: CharNode) -> Value;

    fn compile_string(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        value: StringNode,
    ) -> Result<Value>;

    fn compile_literal(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        node: LiteralNode,
    ) -> Result<Value>;
}

impl<'a, 'b, M: Module, T: Backend<'a, 'b, M>> LiteralCompiler<'a, 'b, M> for T {
    fn compile_literal(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        expr: LiteralNode,
    ) -> Result<Value> {
        Ok(match expr {
            LiteralNode::Bool(b) => Self::compile_bool(ctx, b),
            LiteralNode::Int(i) => Self::compile_int(ctx, i),
            LiteralNode::Float(f) => Self::compile_float(ctx, f),
            LiteralNode::String(s) => Self::compile_string(cctx, ctx, s)?,
            LiteralNode::Char(c) => Self::compile_char(ctx, c),
        })
    }

    fn compile_bool(ctx: &mut CodegenContext<'a, 'b>, value: BoolNode) -> Value {
        ctx.builder
            .write()
            .ins()
            .iconst(types::I8.as_truthy(), if value.value { 1 } else { 0 })
    }

    fn compile_int(ctx: &mut CodegenContext<'a, 'b>, value: IntNode) -> Value {
        ctx.builder.write().ins().iconst(types::I32, value.value)
    }

    fn compile_float(ctx: &mut CodegenContext<'a, 'b>, value: FloatNode) -> Value {
        ctx.builder.write().ins().f64const(value.value)
    }

    fn compile_string(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        value: StringNode,
    ) -> Result<Value> {
        let ptr = Self::ptr(cctx);
        let mut bctx = ctx.builder.write();
        let mut wctx = cctx.write();

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

        let mut ddesc = wctx.data_desc.clone();

        wctx.module.define_data(id, &ddesc).into_diagnostic()?;
        ddesc.clear();
        wctx.data_desc = ddesc;

        RwLockWriteGuard::unlock_fair(wctx);
        Self::post_define(cctx, id)?;

        let local_id = cctx.write().module.declare_data_in_func(id, bctx.func);

        Ok(bctx.ins().global_value(ptr, local_id))
    }

    fn compile_char(ctx: &mut CodegenContext<'a, 'b>, value: CharNode) -> Value {
        ctx.builder
            .write()
            .ins()
            .iconst(types::I32, i64::from(value.value as u32))
    }
}
