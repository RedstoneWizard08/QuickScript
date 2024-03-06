use super::Backend;
use crate::context::{CodegenContext, CompilerContext};
use cranelift_codegen::ir::{AbiParam, InstBuilder, Value};
use cranelift_module::{Linkage, Module};
use miette::{IntoDiagnostic, Result};
use qsc_ast::ast::{literal::LiteralNode, stmt::call::CallNode};

pub trait CallCompiler<'a, M: Module>: Backend<'a, M> {
    fn compile_call(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        call: CallNode<'a>,
    ) -> Result<Value>;
}

impl<'a, M: Module, T: Backend<'a, M>> CallCompiler<'a, M> for T {
    fn compile_call(
        cctx: &mut CompilerContext<'a, M>,
        ctx: &mut CodegenContext<'a>,
        call: CallNode<'a>,
    ) -> Result<Value> {
        let mut sig = cctx.module.make_signature();

        if cctx.functions.contains_key(call.func) {
            let ptr = Self::ptr(cctx);
            let func = cctx.functions.get(call.func).unwrap();

            let args = func
                .args
                .iter()
                .map(|v| v.type_.as_str())
                .collect::<Vec<String>>()
                .join(", ");

            debug!(
                "Using local function for call: {}({}) -> {}",
                call.func,
                args,
                func.ret
                    .clone()
                    .map(|v| v.as_str())
                    .unwrap_or(String::new())
            );

            sig.params.append(
                &mut func
                    .args
                    .iter()
                    .map(|p| AbiParam::new(Self::query_type_with_pointer(ptr, p.type_.as_str())))
                    .collect(),
            );

            sig.returns.push(AbiParam::new(Self::query_type(
                cctx,
                func.ret
                    .clone()
                    .map(|v| v.as_str())
                    .unwrap_or(String::new()),
            )));
        } else {
            let args = call
                .args
                .iter()
                .map(|arg| {
                    if let Ok(ident) = arg.value.data.as_symbol() {
                        if ctx.vars.contains_key(ident.value) {
                            return ctx
                                .vars
                                .get(ident.value)
                                .unwrap()
                                .1
                                .clone()
                                .map(|v| v.as_str())
                                .unwrap_or("i32".to_string());
                        }
                    }

                    if let Ok(literal) = arg.value.data.as_literal() {
                        return match literal {
                            LiteralNode::Bool(_) => "bool",
                            LiteralNode::Char(_) => "char",
                            LiteralNode::Float(_) => "f64",
                            LiteralNode::Int(_) => "i32",
                            LiteralNode::String(_) => "str",
                        }
                        .to_string();
                    }

                    "ptr".to_string()
                })
                .collect::<Vec<String>>();

            debug!(
                "Using imported function for call (Linkage::Import): {}({}) -> i32",
                call.func,
                args.join(", ")
            );

            sig.params.append(
                &mut args
                    .iter()
                    .map(|ty| AbiParam::new(Self::query_type(cctx, ty.clone())))
                    .collect(),
            );

            sig.returns
                .push(AbiParam::new(Self::query_type(cctx, "i32".to_string())));
        }

        let callee = cctx
            .module
            .declare_function(&call.func, Linkage::Import, &sig)
            .into_diagnostic()?;

        let local_callee = cctx
            .module
            .declare_func_in_func(callee, &mut ctx.builder.func);

        let mut args = Vec::new();

        for arg in call.args {
            args.push(Self::compile(cctx, ctx, arg.value)?);
        }

        let call = ctx.builder.ins().call(local_callee, &args);
        let result = ctx.builder.inst_results(call)[0];

        Ok(result)
    }
}
