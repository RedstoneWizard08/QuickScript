use std::sync::Arc;

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

pub trait FunctionCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_fn(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
        ctx: &mut CodegenContext<'a>,
        func: &FunctionNode<'a>,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> FunctionCompiler<'a, M> for T {
    fn compile_fn(
        cctx: Arc<RwLock<CompilerContext<'a, M>>>,
        ctx: &mut CodegenContext<'a>,
        func: &FunctionNode<'a>,
    ) -> Result<Value> {
        let entry;

        {
            let mut bctx = ctx.builder.write();
            entry = bctx.create_block();

            bctx.append_block_params_for_function_params(entry);
            bctx.switch_to_block(entry);
            bctx.seal_block(entry);
        }

        for (idx, arg) in func.args.iter().enumerate() {
            let val = ctx.builder.write().block_params(entry)[idx];
            let var = Self::declare_var(cctx.clone(), ctx, arg.clone().into())?;

            ctx.builder.write().def_var(var, val);
        }

        for node in func.content.data.clone() {
            Self::compile(cctx.clone(), ctx, node)?;
        }

        if ctx.vars.contains_key(RETURN_VAR) {
            let val = Self::compile_named_var(cctx, ctx, RETURN_VAR)?;

            ctx.builder.write().ins().return_(&[val]);
        } else {
            ctx.builder.write().ins().return_(&[]);
        }

        Ok(Value::from_u32(0))
    }
}
