use anyhow::Result;
use cranelift_codegen::ir::{types, InstBuilder, Type, Value};
use cranelift_module::{Linkage, Module};

use crate::{
    ast::literal::Literal,
    codegen::context::{CodegenContext, CompilerContext},
    util::random_string,
};

use super::Backend;

pub trait LiteralCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_literal(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        expr: Literal,
    ) -> Result<Value>;
    fn compile_none(ctx: &mut CodegenContext) -> Value;
    fn compile_bool(ctx: &mut CodegenContext, value: bool) -> Value;
    fn compile_int(ctx: &mut CodegenContext, value: i32) -> Value;
    fn compile_float(ctx: &mut CodegenContext, value: f32) -> Value;
    fn compile_string(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        value: String,
    ) -> Result<Value>;
    fn compile_char(ctx: &mut CodegenContext, value: char) -> Value;
}

impl<'a, M: Module, T: Backend<'a, M>> LiteralCompiler<'a, M> for T {
    fn compile_literal(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        expr: Literal,
    ) -> Result<Value> {
        Ok(match expr {
            Literal::None => Self::compile_none(ctx),
            Literal::Boolean(b) => Self::compile_bool(ctx, b),
            Literal::Integer(i) => Self::compile_int(ctx, i),
            Literal::Float(f) => Self::compile_float(ctx, f),
            Literal::String(s) => Self::compile_string(cctx, ctx, s)?,
            Literal::Char(c) => Self::compile_char(ctx, c),
        })
    }

    fn compile_none(ctx: &mut CodegenContext) -> Value {
        Self::null(ctx)
    }

    fn compile_bool(ctx: &mut CodegenContext, value: bool) -> Value {
        ctx.builder
            .ins()
            .iconst(Type::int(1).unwrap(), i64::from(value))
    }

    fn compile_int(ctx: &mut CodegenContext, value: i32) -> Value {
        ctx.builder.ins().iconst(types::I32, i64::from(value))
    }

    fn compile_float(ctx: &mut CodegenContext, value: f32) -> Value {
        ctx.builder.ins().f32const(value)
    }

    fn compile_string(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        value: String,
    ) -> Result<Value> {
        cctx.data_desc
            .define(format!("{}\0", value).as_bytes().to_vec().into_boxed_slice());

        let name = format!("literal_string_{}", random_string(10));

        let id = cctx
            .module
            .declare_data(&name, Linkage::Export, true, false)?;

        cctx.module.define_data(id, &cctx.data_desc)?;
        cctx.data_desc.clear();

        let local_id = cctx.module.declare_data_in_func(id, ctx.builder.func);

        Ok(ctx.builder.ins().global_value(Self::ptr(cctx), local_id))
    }

    fn compile_char(ctx: &mut CodegenContext, value: char) -> Value {
        ctx.builder
            .ins()
            .iconst(types::I32, i64::from(value as u32))
    }
}
