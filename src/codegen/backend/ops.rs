use anyhow::Result;
use cranelift_codegen::ir::{InstBuilder, Value};

use crate::ast::operation::Operation;

use super::Backend;

pub trait OperationCompiler<'a>: Backend<'a> {
    fn compile_op(&mut self, expr: Operation) -> Result<Value>;
}

impl<'a, T: Backend<'a>> OperationCompiler<'a> for T {
    fn compile_op(&mut self, expr: Operation) -> Result<Value> {
        let data = expr.data();
        let left = self.compile(data.left.content.clone())?;
        let right = self.compile(data.right.content.clone())?;

        match expr {
            Operation::Add(_) => Ok(self.builder().borrow_mut().ins().fadd(left, right)),
            Operation::Subtract(_) => Ok(self.builder().borrow_mut().ins().fsub(left, right)),
            Operation::Multiply(_) => Ok(self.builder().borrow_mut().ins().fmul(left, right)),
            Operation::Divide(_) => Ok(self.builder().borrow_mut().ins().fdiv(left, right)),

            _ => todo!("This operation is not implemented yet!"),
        }
    }
}
