use anyhow::Result;
use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_module::Module;

use crate::{
    ast::var::FunctionData,
    codegen::{
        backend::{Backend, RETURN_VAR},
        context::{CodegenContext, CompilerContext},
    },
};

use super::var::VariableCompiler;

pub trait FunctionCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_fn(
        cctx: CompilerContext<'a, M>,
        ctx: CodegenContext,
        func: FunctionData,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> FunctionCompiler<'a, M> for T {
    fn compile_fn(
        mut cctx: CompilerContext<'a, M>,
        mut ctx: CodegenContext,
        func: FunctionData,
    ) -> Result<Value> {
        let entry = ctx.builder.create_block();

        ctx.builder.append_block_params_for_function_params(entry);
        ctx.builder.switch_to_block(entry);
        ctx.builder.seal_block(entry);

        for expr in &*func.body {
            Self::compile(&mut cctx, &mut ctx, expr.content.clone())?;
        }

        if ctx.vars.contains_key(&RETURN_VAR.to_string()) {
            let val = Self::compile_named_var(&mut ctx, RETURN_VAR.to_string())?;

            ctx.builder.ins().return_(&[val]);
        }

        ctx.builder.finalize();

        Ok(Value::from_u32(0))
    }
}
