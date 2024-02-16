use anyhow::Result;
use cranelift_codegen::ir::{AbiParam, InstBuilder, Value};
use cranelift_module::Linkage;

use crate::{
    ast::var::FunctionData,
    codegen::backend::{Backend, RETURN_VAR},
};

use super::var::VariableCompiler;

pub trait FunctionCompiler<'a>: Backend<'a> {
    fn compile_fn(&mut self, func: FunctionData) -> Result<Value>;
}

impl<'a, T: Backend<'a>> FunctionCompiler<'a> for T {
    fn compile_fn(&mut self, func: FunctionData) -> Result<Value> {
        for arg in func.args {
            self.ctx()
                .func
                .signature
                .params
                .push(AbiParam::new(Self::query_type(arg.type_)));
        }

        self.ctx()
            .func
            .signature
            .returns
            .push(AbiParam::new(Self::query_type(func.return_type)));

        self.new_builder();

        let entry = self.builder().borrow_mut().create_block();

        self.builder().borrow_mut()
            .append_block_params_for_function_params(entry);
        
        self.builder().borrow_mut().switch_to_block(entry);
        self.builder().borrow_mut().seal_block(entry);

        for expr in &*func.body {
            self.compile(expr.content.clone())?;
        }

        if self.vars().contains_key(&RETURN_VAR.to_string()) {
            let val = self.compile_named_var(RETURN_VAR.to_string())?;

            self.builder().borrow_mut().ins().return_(&[val]);
        }

        self.finalize_builder();

        let id = self.declare_func(&func.name, Linkage::Export)?;

        self.complete_define_func(id)?;
        self.post_define()?;

        Ok(self.nullptr())
    }
}
