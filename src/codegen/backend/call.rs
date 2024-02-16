use anyhow::Result;
use cranelift_codegen::ir::{AbiParam, InstBuilder, Value};
use cranelift_module::{Linkage, Module};

use crate::{
    ast::{call::Call, expr::ExprKind},
    codegen::context::{CodegenContext, CompilerContext},
};

use super::Backend;

pub trait CallCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_call(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        call: Call,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> CallCompiler<'a, M> for T {
    fn compile_call(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext,
        call: Call,
    ) -> Result<Value> {
        let mut sig = cctx.module.make_signature();

        if cctx.functions.contains_key(&call.name) {
            let ptr = Self::ptr(cctx);
            let func = cctx.functions.get(&call.name).unwrap();

            sig.params.append(
                &mut func
                    .args
                    .iter()
                    .map(|p| AbiParam::new(Self::query_type_with_pointer(ptr, p.type_.clone())))
                    .collect(),
            );

            sig.returns.push(AbiParam::new(Self::query_type(
                cctx,
                func.return_type.clone(),
            )));
        } else {
            sig.params.append(
                &mut call
                    .args
                    .iter()
                    .map(|arg| {
                        if let ExprKind::Identifer(ident) = arg.content.clone() {
                            if ctx.vars.contains_key(&ident) {
                                return AbiParam::new(Self::query_type(
                                    cctx,
                                    ctx.vars.get(&ident).unwrap().1.clone(),
                                ));
                            }
                        }

                        AbiParam::new(Self::query_type(cctx, arg.type_name()))
                    })
                    .collect(),
            );

            sig.returns
                .push(AbiParam::new(Self::query_type(cctx, "i32".to_string())));
        }

        let callee = cctx
            .module
            .declare_function(&call.name, Linkage::Import, &sig)?;

        let local_callee = cctx
            .module
            .declare_func_in_func(callee, &mut ctx.builder.func);
        
        let mut args = Vec::new();

        for arg in call.args {
            args.push(Self::compile(cctx, ctx, arg.content)?);
        }

        let call = ctx.builder.ins().call(local_callee, &args);
        let result = ctx.builder.inst_results(call)[0];

        Ok(result)
    }
}
