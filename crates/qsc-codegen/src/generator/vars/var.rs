use anyhow::Result;
use cranelift_codegen::{
    entity::EntityRef,
    ir::{InstBuilder, Value},
};
use cranelift_frontend::Variable;
use cranelift_module::{DataId, Linkage, Module};

use qsc_ast::ast::decl::var::VariableNode as Var;

use crate::{
    context::{CodegenContext, CompilerContext},
    generator::Backend,
};

pub trait VariableCompiler<'a, M: Module>: Backend<'a, M> {
    type O;

    fn compile_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
    ) -> Result<Self::O>;

    fn declare_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
    ) -> Result<Variable>;

    fn compile_empty_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
    ) -> Result<Self::O>;

    fn compile_data_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
        data: DataId,
    ) -> Result<Self::O>;

    fn compile_value_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
        value: Value,
    ) -> Result<Self::O>;

    fn compile_named_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        ident: &'a str,
    ) -> Result<Self::O>;
}

impl<'a, M: Module, T: Backend<'a, M>> VariableCompiler<'a, M> for T {
    type O = Value;

    fn compile_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
    ) -> Result<Self::O> {
        match var.clone().value {
            Some(value) => {
                let val = Self::compile(cctx, ctx, value)?;

                Self::compile_value_var(cctx, ctx, var, val)
            }

            None => Self::compile_empty_var(cctx, ctx, var),
        }
    }

    fn declare_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
    ) -> Result<Variable> {
        let ty = Self::query_type(
            cctx,
            var.type_
                .clone()
                .map(|v| v.as_str())
                .unwrap_or(String::new()),
        );
        let ref_ = Variable::new(ctx.vars.len());

        ctx.builder.declare_var(ref_, ty);
        ctx.vars.insert(var.name, (ref_, var.type_));

        Ok(ref_)
    }

    fn compile_empty_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
    ) -> Result<Self::O> {
        let ty = Self::query_type(
            cctx,
            var.type_
                .clone()
                .map(|v| v.as_str())
                .unwrap_or(String::new()),
        );
        let null = ctx.builder.ins().null(ty);
        let ref_ = Variable::new(ctx.vars.len());

        ctx.builder.declare_var(ref_, ty);
        ctx.builder.def_var(ref_, null);
        ctx.vars.insert(var.name, (ref_, var.type_.clone()));

        Ok(ctx.builder.use_var(ref_))
    }

    fn compile_data_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
        data: DataId,
    ) -> Result<Self::O> {
        let val = Self::get_global(cctx, ctx, data);

        let val = ctx.builder.ins().symbol_value(
            Self::query_type(
                cctx,
                var.type_
                    .clone()
                    .map(|v| v.as_str())
                    .unwrap_or(String::new()),
            ),
            val,
        );

        let ty = Self::query_type(
            cctx,
            var.type_
                .clone()
                .map(|v| v.as_str())
                .unwrap_or(String::new()),
        );
        let ref_ = Variable::new(ctx.vars.len());

        ctx.builder.declare_var(ref_, ty);
        ctx.builder.def_var(ref_, val);
        ctx.vars.insert(var.name, (ref_, var.type_.clone()));

        Ok(ctx.builder.use_var(ref_))
    }

    fn compile_value_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: Var<'a>,
        val: Value,
    ) -> Result<Self::O> {
        let ty = Self::query_type(
            cctx,
            var.type_
                .clone()
                .map(|v| v.as_str())
                .unwrap_or(String::new()),
        );
        let ref_ = Variable::new(ctx.vars.len());

        ctx.builder.declare_var(ref_, ty);
        ctx.builder.def_var(ref_, val);
        ctx.vars.insert(var.name, (ref_, var.type_.clone()));

        Ok(ctx.builder.use_var(ref_))
    }

    fn compile_named_var(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        ident: &'a str,
    ) -> Result<Self::O> {
        if ctx.vars.contains_key(ident) {
            let (ref_, _) = *ctx.vars.get(ident).unwrap();

            Ok(ctx.builder.use_var(ref_))
        } else if cctx.globals.contains_key(ident) {
            let sym = cctx
                .module
                .declare_data(ident, Linkage::Export, true, false)?;
            let local_id = cctx.module.declare_data_in_func(sym, ctx.builder.func);
            let ptr = Self::ptr(cctx);
            let val = ctx.builder.ins().symbol_value(ptr, local_id);

            Ok(val)
        } else if ctx.values.contains_key(ident) {
            let (val, _) = *ctx.values.get(ident).unwrap();

            Ok(val)
        } else {
            Err(anyhow::anyhow!("Variable {} not found", ident))
        }
    }
}
