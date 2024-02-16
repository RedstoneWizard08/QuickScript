use anyhow::Result;
use cranelift_codegen::{
    entity::EntityRef,
    ir::{InstBuilder, Value},
};
use cranelift_frontend::Variable;
use cranelift_module::DataId;

use crate::{ast::var::VariableData, codegen::backend::Backend};

pub trait VariableCompiler<'a>: Backend<'a> {
    type O;

    fn compile_var_data(&mut self, var: VariableData) -> Result<Self::O>;
    fn compile_empty_var(&mut self, var: VariableData) -> Result<Self::O>;
    fn compile_data_var(&mut self, var: VariableData, data: DataId) -> Result<Self::O>;
    fn compile_value_var(&mut self, var: VariableData, value: Value) -> Result<Self::O>;
    fn compile_named_var(&mut self, ident: String) -> Result<Self::O>;
}

impl<'a, T: Backend<'a>> VariableCompiler<'a> for T {
    type O = Value;

    fn compile_var_data(&mut self, var: VariableData) -> Result<Self::O> {
        match var.value {
            Some(value) => self.compile(value.content),
            None => self.compile_empty_var(var),
        }
    }

    fn compile_empty_var(&mut self, var: VariableData) -> Result<Self::O> {
        let ref_ = Variable::new(self.vars().len());
        let null = self.builder().borrow_mut().ins().null(Self::query_type(var.type_));

        self.builder().borrow_mut().def_var(ref_, null);
        self.vars().insert(var.name.clone(), ref_);

        Ok(self.builder().borrow_mut().use_var(ref_))
    }

    fn compile_data_var(&mut self, var: VariableData, data: DataId) -> Result<Self::O> {
        let ref_ = Variable::new(self.vars().len());
        let ptr = self.ptr();
        let val = self.get_global(data);
        let val = self.builder().borrow_mut().ins().symbol_value(ptr, val);

        self.builder().borrow_mut().def_var(ref_, val);
        self.vars().insert(var.name.clone(), ref_);

        Ok(self.builder().borrow_mut().use_var(ref_))
    }

    fn compile_value_var(&mut self, var: VariableData, value: Value) -> Result<Self::O> {
        let ref_ = Variable::new(self.vars().len());

        self.builder().borrow_mut().def_var(ref_, value);
        self.vars().insert(var.name.clone(), ref_);

        Ok(self.builder().borrow_mut().use_var(ref_))
    }

    fn compile_named_var(&mut self, ident: String) -> Result<Self::O> {
        if self.vars().contains_key(&ident) {
            let val = *self.vars().get(&ident).unwrap();

            Ok(self.builder().borrow_mut().use_var(val))
        } else {
            Err(anyhow::anyhow!("Variable {} not found", ident))
        }
    }
}
