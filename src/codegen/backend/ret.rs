use anyhow::Result;
use cranelift_codegen::{entity::EntityRef, ir::Value};
use cranelift_frontend::Variable;

use crate::ast::ret::Return;

use super::{Backend, RETURN_VAR};

pub trait ReturnCompiler<'a>: Backend<'a> {
    fn compile_return(&mut self, expr: Return) -> Result<Value>;
}

impl<'a, T: Backend<'a>> ReturnCompiler<'a> for T {
    fn compile_return(&mut self, expr: Return) -> Result<Value> {
        if let Some(data) = expr.data {
            let val = self.compile(data.content)?;
            let ref_ = Variable::new(self.vars().len());

            self.builder().borrow_mut().def_var(ref_, val);
            self.vars().insert(RETURN_VAR.to_string(), ref_);

            Ok(self.builder().borrow_mut().use_var(ref_))
        } else {
            Ok(self.null())
        }
    }
}
