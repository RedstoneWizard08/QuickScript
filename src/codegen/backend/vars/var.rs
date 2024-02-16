use anyhow::Result;
use cranelift_codegen::{
    entity::EntityRef,
    ir::{InstBuilder, Value},
};
use cranelift_frontend::Variable;
use cranelift_module::{DataId, Module};

use crate::{
    ast::var::VariableData,
    codegen::{
        backend::Backend,
        context::{CodegenContext, CompilerContext},
    },
};

pub trait VariableCompiler<'a, M: Module>: Backend<'a, M> {
    type O;

    fn compile_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: VariableData,
    ) -> Result<Self::O>;

    fn compile_empty_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: VariableData,
    ) -> Result<Self::O>;

    fn compile_data_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: VariableData,
        data: DataId,
    ) -> Result<Self::O>;

    fn compile_value_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: VariableData,
        value: Value,
    ) -> Result<Self::O>;

    fn compile_named_var(ctx: &mut CodegenContext, ident: String) -> Result<Self::O>;
}

impl<'a, M: Module, T: Backend<'a, M>> VariableCompiler<'a, M> for T {
    type O = Value;

    fn compile_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: VariableData,
    ) -> Result<Self::O> {
        match var.clone().value {
            Some(value) => {
                let val = Self::compile(cctx, ctx, value.content)?;

                Self::compile_value_var(cctx, ctx, var, val)
            }

            None => Self::compile_empty_var(cctx, ctx, var),
        }
    }

    fn compile_empty_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: VariableData,
    ) -> Result<Self::O> {
        let ty = Self::query_type(cctx, var.type_.clone());
        let null = ctx.builder.ins().null(ty);
        let ref_ = Variable::new(ctx.vars.len());

        ctx.builder.declare_var(ref_, ty);
        ctx.builder.def_var(ref_, null);
        ctx.vars.insert(var.name.clone(), (ref_, var.type_.clone()));

        Ok(ctx.builder.use_var(ref_))
    }

    fn compile_data_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: VariableData,
        data: DataId,
    ) -> Result<Self::O> {
        let val = Self::get_global(cctx, ctx, data);

        let val = ctx
            .builder
            .ins()
            .symbol_value(Self::query_type(cctx, var.type_.clone()), val);

        let ty = Self::query_type(cctx, var.type_.clone());
        let ref_ = Variable::new(ctx.vars.len());

        ctx.builder.declare_var(ref_, ty);
        ctx.builder.def_var(ref_, val);
        ctx.vars.insert(var.name.clone(), (ref_, var.type_.clone()));

        Ok(ctx.builder.use_var(ref_))
    }

    fn compile_value_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: VariableData,
        val: Value,
    ) -> Result<Self::O> {
        let ty = Self::query_type(cctx, var.type_.clone());
        let ref_ = Variable::new(ctx.vars.len());

        ctx.builder.declare_var(ref_, ty);
        ctx.builder.def_var(ref_, val);
        ctx.vars.insert(var.name.clone(), (ref_, var.type_.clone()));

        Ok(ctx.builder.use_var(ref_))
    }

    fn compile_named_var(ctx: &mut CodegenContext, ident: String) -> Result<Self::O> {
        if ctx.vars.contains_key(&ident) {
            let (ref_, _) = *ctx.vars.get(&ident).unwrap();

            Ok(ctx.builder.use_var(ref_))
        } else {
            Err(anyhow::anyhow!("Variable {} not found", ident))
        }
    }
}
