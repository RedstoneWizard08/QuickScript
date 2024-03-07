use super::Backend;
use crate::{
    alias::DeclareAliasedFunction,
    context::{CodegenContext, CompilerContext},
};
use cranelift_codegen::ir::{AbiParam, Function, InstBuilder, Value};
use cranelift_module::{Linkage, Module};
use miette::{IntoDiagnostic, Result};
use parking_lot::{RwLock, RwLockWriteGuard};
use qsc_ast::ast::{literal::LiteralNode, stmt::call::CallNode};
use qsc_core::util::random_string;

pub trait CallCompiler<'a, 'b, M: Module>: Backend<'a, 'b, M> {
    fn compile_call(
        cctx: &RwLock<CompilerContext<'a, M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        call: CallNode<'a>,
    ) -> Result<Value>;
}

impl<'a, 'b, M: Module + DeclareAliasedFunction, T: Backend<'a, 'b, M>> CallCompiler<'a, 'b, M>
    for T
{
    fn compile_call(
        cctx: &RwLock<CompilerContext<'a, M>>,
        ctx: &mut CodegenContext<'a, 'b>,
        call: CallNode<'a>,
    ) -> Result<Value> {
        debug!("Trying to compile call: {:?}", call);

        let ptr = Self::ptr(cctx);
        let mut wctx = cctx.write();
        let mut sig = wctx.module.make_signature();
        let mut func_name = call.func.to_string();

        if wctx.functions.contains_key(call.func) {
            let func = wctx.functions.get(call.func).unwrap();

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

            sig.returns
                .push(AbiParam::new(Self::query_type_with_pointer(
                    ptr,
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

            func_name = format!("__qsc::alias::{}_{}", call.func, random_string(8));

            debug!(
                "Using imported function for call (name = {}, Linkage::Import): {}({}) -> i32",
                func_name,
                call.func,
                args.join(", ")
            );

            sig.params.append(
                &mut args
                    .iter()
                    .map(|ty| AbiParam::new(Self::query_type_with_pointer(ptr, ty.clone())))
                    .collect(),
            );

            sig.returns
                .push(AbiParam::new(Self::query_type_with_pointer(
                    ptr,
                    "i32".to_string(),
                )));
        }

        debug!("Emitting call instruction...");

        let callee = wctx
            .module
            .declare_aliased_function(&func_name, &call.func, Linkage::Import, &sig)
            .into_diagnostic()?;

        let func_ref = unsafe { ((&mut wctx.ctx.func) as *mut Function).as_mut() }.unwrap();
        let local_callee = wctx.module.declare_func_in_func(callee, func_ref);
        let mut args = Vec::new();

        RwLockWriteGuard::unlock_fair(wctx);

        for arg in call.args {
            args.push(Self::compile(cctx, ctx, arg.value)?);
        }

        let call = ctx.builder.write().ins().call(local_callee, &args);
        let result = ctx.builder.write().inst_results(call)[0];

        Ok(result)
    }
}
