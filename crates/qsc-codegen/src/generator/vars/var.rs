use anyhow::Result;
use cranelift_codegen::{
    entity::EntityRef,
    ir::{InstBuilder, Value},
};
use cranelift_frontend::Variable;
use cranelift_module::{DataId, Module};

use qsc_ast::var::Variable as Var;

use crate::{
    context::{CodegenContext, CompilerContext},
    generator::Backend,
};

pub trait VariableCompiler<'a, M: Module>: Backend<'a, M> {
    type O;

    fn compile_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: Var,
    ) -> Result<Self::O>;

    fn declare_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: Var,
    ) -> Result<Variable>;

    fn compile_empty_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: Var,
    ) -> Result<Self::O>;

    fn compile_data_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: Var,
        data: DataId,
    ) -> Result<Self::O>;

    fn compile_value_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: Var,
        value: Value,
    ) -> Result<Self::O>;

    fn compile_named_var(ctx: &mut CodegenContext, ident: String) -> Result<Self::O>;
}

impl<'a, M: Module, T: Backend<'a, M>> VariableCompiler<'a, M> for T {
    type O = Value;

    fn compile_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: Var,
    ) -> Result<Self::O> {
        match var.clone().value {
            Some(value) => {
                let val = Self::compile(cctx, ctx, value.content)?;

                Self::compile_value_var(cctx, ctx, var, val)
            }

            None => Self::compile_empty_var(cctx, ctx, var),
        }
    }

    fn declare_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: Var,
    ) -> Result<Variable> {
        let ty = Self::query_type(cctx, var.type_.clone());
        let ref_ = Variable::new(ctx.vars.len());

        ctx.builder.declare_var(ref_, ty);
        ctx.vars.insert(var.name.clone(), (ref_, var.type_.clone()));

        Ok(ref_)
    }

    fn compile_empty_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: Var,
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
        var: Var,
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
        var: Var,
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
        } else if ctx.values.contains_key(&ident) {
            let (val, _) = *ctx.values.get(&ident).unwrap();

            Ok(val)
        } else {
            Err(anyhow::anyhow!("Variable {} not found", ident))
        }
    }
}
