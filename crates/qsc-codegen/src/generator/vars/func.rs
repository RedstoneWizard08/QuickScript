use anyhow::Result;
use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_module::Module;

use crate::{
    context::{CodegenContext, CompilerContext},
    generator::{Backend, RETURN_VAR},
};

use qsc_ast::ast::decl::func::FunctionNode;

use super::var::VariableCompiler;

pub trait FunctionCompiler<'i, 'a, M: Module>: Backend<'i, 'a, M> {
    fn compile_fn(
        cctx: &mut CompilerContext<'i, 'a, M>,
        ctx: &mut CodegenContext,
        func: FunctionNode<'i>,
    ) -> Result<Value>;
}

impl<'i, 'a, M: Module, T: Backend<'i, 'a, M>> FunctionCompiler<'i, 'a, M> for T {
    fn compile_fn(
        cctx: &mut CompilerContext<'i, 'a, M>,
        ctx: &mut CodegenContext,
        func: FunctionNode<'i>,
    ) -> Result<Value> {
        let entry = ctx.builder.create_block();

        ctx.builder.append_block_params_for_function_params(entry);
        ctx.builder.switch_to_block(entry);
        ctx.builder.seal_block(entry);

        for (idx, arg) in func.args.iter().enumerate() {
            let val = ctx.builder.block_params(entry)[idx];
            let var = Self::declare_var(cctx, ctx, arg.clone().into())?;

            ctx.builder.def_var(var, val);
        }

        for node in func.content.data {
            Self::compile(cctx, ctx, node)?;
        }

        if ctx.vars.contains_key(RETURN_VAR) {
            let val = Self::compile_named_var(cctx, ctx, RETURN_VAR)?;

            ctx.builder.ins().return_(&[val]);
        } else {
            ctx.builder.ins().return_(&[]);
        }

        Ok(Value::from_u32(0))
    }
}
