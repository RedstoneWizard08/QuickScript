use anyhow::Result;
use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_module::Module;

use crate::{
    backend::{Backend, RETURN_VAR},
    context::{CodegenContext, CompilerContext},
};
use qsc_ast::var::FunctionData;

use super::var::VariableCompiler;

pub trait FunctionCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_fn(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        func: FunctionData,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> FunctionCompiler<'a, M> for T {
    fn compile_fn(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        func: FunctionData,
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

        for expr in &*func.body {
            Self::compile(cctx, ctx, expr.content.clone())?;
        }

        if ctx.vars.contains_key(&RETURN_VAR.to_string()) {
            let val = Self::compile_named_var(ctx, RETURN_VAR.to_string())?;

            ctx.builder.ins().return_(&[val]);
        } else {
            ctx.builder.ins().return_(&[]);
        }

        Ok(Value::from_u32(0))
    }
}
