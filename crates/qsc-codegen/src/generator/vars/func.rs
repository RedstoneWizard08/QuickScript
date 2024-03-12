use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_module::Module;
use miette::Result;
use parking_lot::RwLock;

use crate::{
    context::{CodegenContext, CompilerContext},
    generator::{Backend, RETURN_VAR},
};

use qsc_ast::ast::decl::func::FunctionNode;

use super::var::VariableCompiler;

pub trait FunctionCompiler<'a, 'b, M: Module>: Backend<'a, 'b, M> {
    fn compile_fn(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        func: &FunctionNode,
    ) -> Result<Value>;
}

impl<'a, 'b, M: Module, T: Backend<'a, 'b, M>> FunctionCompiler<'a, 'b, M> for T {
    fn compile_fn(
        cctx: &RwLock<CompilerContext<M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        func: &FunctionNode,
    ) -> Result<Value> {
        let entry;

        debug!("Creating entry block for function: {}", func.name);

        {
            let mut bctx = ctx.builder.write();

            entry = bctx.create_block();
            bctx.append_block_params_for_function_params(entry);
            bctx.switch_to_block(entry);
            bctx.seal_block(entry);
        }

        debug!("Declaring argument variables for function: {}", func.name);

        for (idx, arg) in func.args.iter().enumerate() {
            let val = ctx.builder.write().block_params(entry)[idx];
            let var = Self::declare_var(cctx, ctx, arg.clone().into())?;

            ctx.builder.write().def_var(var, val);
        }

        debug!("Compiling nodes for function: {}", func.name);

        for node in func.content.data.clone() {
            Self::compile(cctx, ctx, node)?;
        }

        debug!("Compiled all nodes for function: {}", func.name);

        if !ctx.vars.contains_key(RETURN_VAR) {
            ctx.builder.write().ins().return_(&[]);
        }

        debug!("Compiled function: {}", func.name);

        Ok(Value::from_u32(0))
    }
}
