use anyhow::Result;
use cranelift_codegen::ir::Value;

use crate::ast::var::Variable;

use self::{func::FunctionCompiler, var::VariableCompiler};

use super::Backend;

pub mod func;
pub mod var;

pub trait VariableExprCompiler<'a>: Backend<'a> {
    fn compile_var(&mut self, expr: Variable) -> Result<Value>;
}

impl<'a, T: Backend<'a>> VariableExprCompiler<'a> for T {
    fn compile_var(&mut self, expr: Variable) -> Result<Value> {
        match expr {
            Variable::Variable(var) => self.compile_var_data(var),
            Variable::Function(func) => self.compile_fn(func),

            Variable::None => Ok(self.null()),
        }
    }
}
