use anyhow::Result;
use cranelift_codegen::ir::{InstBuilder, StackSlotData, StackSlotKind, Value};
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

    fn compile_named_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        ident: String,
    ) -> Result<Self::O>;
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
        let slot = ctx.builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            Self::query_type(cctx, var.type_.clone()).bits(),
        ));

        let null = ctx
            .builder
            .ins()
            .null(Self::query_type(cctx, var.type_.clone()));

        ctx.builder.ins().stack_store(null, slot, 0);
        ctx.vars.insert(var.name.clone(), (slot, var.type_.clone()));

        Ok(ctx
            .builder
            .ins()
            .stack_load(Self::query_type(cctx, var.type_), slot, 0))
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

        let slot = ctx.builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            Self::query_type(cctx, var.type_.clone()).bits(),
        ));

        ctx.builder.ins().stack_store(val, slot, 0);
        ctx.vars.insert(var.name.clone(), (slot, var.type_.clone()));

        Ok(ctx
            .builder
            .ins()
            .stack_load(Self::query_type(cctx, var.type_), slot, 0))
    }

    fn compile_value_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        var: VariableData,
        value: Value,
    ) -> Result<Self::O> {
        let slot = ctx.builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            Self::query_type(cctx, var.type_.clone()).bits(),
        ));

        ctx.builder.ins().stack_store(value, slot, 0);
        ctx.vars.insert(var.name.clone(), (slot, var.type_.clone()));

        Ok(ctx
            .builder
            .ins()
            .stack_load(Self::query_type(cctx, var.type_), slot, 0))
    }

    fn compile_named_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        ident: String,
    ) -> Result<Self::O> {
        if ctx.vars.contains_key(&ident) {
            let (slot, ty) = ctx.vars.get(&ident).unwrap();

            Ok(ctx
                .builder
                .ins()
                .stack_load(Self::query_type(cctx, ty.clone()), *slot, 0))
        } else {
            Err(anyhow::anyhow!("Variable {} not found", ident))
        }
    }
}
