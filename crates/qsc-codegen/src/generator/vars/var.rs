use cranelift_codegen::{
    entity::EntityRef,
    ir::{InstBuilder, Value},
};
use cranelift_frontend::Variable;
use cranelift_module::{DataId, Linkage, Module};
use miette::{IntoDiagnostic, Result};

use parking_lot::RwLock;
use qsc_ast::ast::{decl::var::VariableNode, node::sym::SymbolNode};
use qsc_core::conv::IntoSourceSpan;

use crate::{
    context::{CodegenContext, CompilerContext},
    error::CodegenError,
    generator::Backend,
};

pub trait VariableCompiler<'a, 'b, M: Module>: Backend<'a, 'b, M> {
    type O;

    fn compile_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
    ) -> Result<Self::O>;

    fn declare_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
    ) -> Result<Variable>;

    fn compile_empty_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
    ) -> Result<Self::O>;

    fn compile_data_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
        data: DataId,
    ) -> Result<Self::O>;

    fn compile_value_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
        value: Value,
    ) -> Result<Self::O>;

    fn compile_named_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        ident: SymbolNode,
    ) -> Result<Self::O>;
}

impl<'a, 'b, M: Module, T: Backend<'a, 'b, M>> VariableCompiler<'a, 'b, M> for T {
    type O = Value;

    fn compile_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
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
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
    ) -> Result<Variable> {
        let ty = Self::query_type(
            cctx,
            var.type_
                .clone()
                .map(|v| v.as_str())
                .unwrap_or(String::new()),
        );
        let ref_ = Variable::new(ctx.vars.len());

        ctx.builder.write().declare_var(ref_, ty);
        ctx.vars.insert(var.name, (ref_, var.type_));

        Ok(ref_)
    }

    fn compile_empty_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
    ) -> Result<Self::O> {
        let ty = Self::query_type(
            cctx,
            var.type_
                .clone()
                .map(|v| v.as_str())
                .unwrap_or(String::new()),
        );

        let mut bctx = ctx.builder.write();
        let null = bctx.ins().null(ty);
        let ref_ = Variable::new(ctx.vars.len());

        bctx.declare_var(ref_, ty);
        bctx.def_var(ref_, null);
        ctx.vars.insert(var.name, (ref_, var.type_.clone()));

        Ok(bctx.use_var(ref_))
    }

    fn compile_data_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
        data: DataId,
    ) -> Result<Self::O> {
        let val = Self::get_global(cctx, ctx, data);

        let val = ctx.builder.write().ins().symbol_value(
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
        let mut bctx = ctx.builder.write();

        bctx.declare_var(ref_, ty);
        bctx.def_var(ref_, val);
        ctx.vars.insert(var.name, (ref_, var.type_.clone()));

        Ok(bctx.use_var(ref_))
    }

    fn compile_value_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: VariableNode,
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
        let mut bctx = ctx.builder.write();

        bctx.declare_var(ref_, ty);
        bctx.def_var(ref_, val);
        ctx.vars.insert(var.name, (ref_, var.type_.clone()));

        Ok(bctx.use_var(ref_))
    }

    fn compile_named_var(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        ident: SymbolNode,
    ) -> Result<Self::O> {
        let ptr = Self::ptr(cctx);
        let mut wctx = cctx.write();
        let mut bctx = ctx.builder.write();

        if ctx.vars.contains_key(&ident.value) {
            let (ref_, _) = *ctx.vars.get(&ident.value).unwrap();

            Ok(bctx.use_var(ref_))
        } else if wctx.globals.contains_key(&ident.value) {
            let sym = wctx
                .module
                .declare_data(&ident.value, Linkage::Export, true, false)
                .into_diagnostic()?;

            let local_id = wctx.module.declare_data_in_func(sym, bctx.func);
            let val = bctx.ins().symbol_value(ptr, local_id);

            Ok(val)
        } else if ctx.values.contains_key(&ident.value) {
            let (val, _) = *ctx.values.get(&ident.value).unwrap();

            Ok(val)
        } else {
            Err(CodegenError {
                error: miette!("Variable {} not found", ident.value),
                location: ident.span.into_source_span(),
                src: wctx.source.clone(),
            }
            .into())
        }
    }
}
