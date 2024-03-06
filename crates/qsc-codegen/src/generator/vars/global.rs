use crate::{
    context::{CodegenContext, CompilerContext},
    generator::Backend,
};
use anyhow::Result;
use cranelift_module::{DataId, Module};
use qsc_ast::ast::decl::global::GlobalVariable;

pub trait GlobalVariableCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_global(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        var: GlobalVariable<'a>,
    ) -> Result<DataId>;
}

impl<'a, M: Module, T: Backend<'a, M>> GlobalVariableCompiler<'a, M> for T {
    fn compile_global(
        _cctx: &mut CompilerContext<'a, M>,
        _ctx: &mut CodegenContext<'a>,
        _var: GlobalVariable<'a>,
    ) -> Result<DataId> {
        todo!();

        // let ty = Self::query_type(cctx, var.type_.as_str());

        // cctx.data_desc.define(Self::compile_literal(cctx, ctx, var.value.data.as_literal()?)?);

        // let id = cctx
        //     .module
        //     .declare_data(&var.name, Linkage::Export, true, false)?;

        // cctx.module.define_data(id, cctx.data_desc)?;
        // cctx.data_desc.clear();

        // Self::post_define(cctx, id)?;

        // cctx.globals.insert(&var.name, id);

        // Ok(id)
    }
}
