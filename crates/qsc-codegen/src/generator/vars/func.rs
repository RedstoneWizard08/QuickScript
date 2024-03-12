use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_module::Module;
use miette::Result;
use parking_lot::RwLock;

use crate::{
    context::{CodegenContext, CompilerContext},
    generator::{Backend, RETURN_VAR},
};

use qsc_ast::ast::{decl::func::FunctionNode, node::sym::SymbolNode};

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
        let end;

        debug!("Creating entry block for function: {}", func.name);

        {
            let mut bctx = ctx.builder.write();
            entry = bctx.create_block();
            end = bctx.create_block();

            bctx.append_block_params_for_function_params(entry);
            bctx.switch_to_block(entry);
            bctx.seal_block(entry);
        }

        ctx.end = Some(end);

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

        // Is this needed?
        // ctx.builder.write().ins().jump(end, &[]);

        // debug_assert!(
        //     self.position.is_none()
        //         || self.is_unreachable()
        //         || self.is_pristine(self.position.unwrap())
        //         || self.is_filled(self.position.unwrap()),
        //     "you have to fill your block before switching"
        // );

        if !ctx.builder.read().position.is_none()
            && !ctx.builder.read().is_unreachable()
            && !ctx.builder.read().is_pristine(entry)
            && !ctx.builder.read().is_filled(entry)
        {
            ctx.builder.write().ins().jump(end, &[]);
        }

        ctx.builder.write().switch_to_block(end);
        ctx.builder.write().seal_block(end);

        debug!("Compiled all nodes for function: {}", func.name);

        if ctx.vars.contains_key(RETURN_VAR) {
            let val = Self::compile_named_var(
                cctx,
                ctx,
                SymbolNode {
                    value: RETURN_VAR.to_string(),
                    span: func.span.clone(),
                },
            )?;

            ctx.builder.write().ins().return_(&[val]);
        } else {
            ctx.builder.write().ins().return_(&[]);
        }

        debug!("Compiled function: {}", func.name);

        Ok(Value::from_u32(0))
    }
}
