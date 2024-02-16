use anyhow::Result;
use cranelift_codegen::ir::{AbiParam, InstBuilder, Value};
use cranelift_module::Linkage;

use crate::ast::call::Call;

use super::Backend;

pub trait CallCompiler<'a>: Backend<'a> {
    fn compile_call(&mut self, call: Call) -> Result<Value>;
}

impl<'a, T: Backend<'a>> CallCompiler<'a> for T {
    fn compile_call(&mut self, call: Call) -> Result<Value> {
        let mut sig = self.module().make_signature();

        if self.functions().contains_key(&call.name) {
            let func = self.functions().get(&call.name).unwrap();

            sig.params.append(
                &mut func
                    .args
                    .iter()
                    .map(|p| AbiParam::new(Self::query_type(p.type_.clone())))
                    .collect(),
            );

            sig.returns
                .push(AbiParam::new(Self::query_type(func.return_type.clone())));
        } else {
            sig.params.append(
                &mut call
                    .args
                    .iter()
                    .map(|_| AbiParam::new(Self::query_type("i32".to_string())))
                    .collect(),
            );

            sig.returns
                .push(AbiParam::new(Self::query_type("i32".to_string())));
        }

        let callee = self
            .module()
            .declare_function(&call.name, Linkage::Import, &sig)?;

        let mut func = self.builder().borrow_mut().func.clone();
        let local_callee = self.module().declare_func_in_func(callee, &mut func);
        
        *self.builder().borrow_mut().func = func;

        let mut args = Vec::new();

        for arg in call.args {
            args.push(self.compile(arg.content)?);
        }

        let call = self.builder().borrow_mut().ins().call(local_callee, &args);
        let result = self.builder().borrow_mut().inst_results(call)[0];

        Ok(result)
    }
}
