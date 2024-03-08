use crate::{
    context::{CodegenContext, CompilerContext},
    generator::Backend,
};
use cranelift_module::{DataId, Module};
use miette::Result;
use qsc_ast::ast::decl::global::GlobalVariable;

pub trait GlobalVariableCompiler<'a, 'b, M: Module>: Backend<'a, 'b, M> {
    fn compile_global(
        cctx: &mut CompilerContext<M>,
        ctx: &mut CodegenContext<'a, 'b>,
        var: GlobalVariable,
    ) -> Result<DataId>;
}

impl<'a, 'b, M: Module, T: Backend<'a, 'b, M>> GlobalVariableCompiler<'a, 'b, M> for T {
    fn compile_global(
        _cctx: &mut CompilerContext<M>,
        _ctx: &mut CodegenContext<'a, 'b>,
        _var: GlobalVariable,
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
