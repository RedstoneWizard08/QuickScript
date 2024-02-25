use crate::{expr::Expr, var::Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub body: Box<Vec<Expr>>,
    pub return_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionArg {
    pub name: String,
    pub type_: String,
    pub is_mutable: bool,
}

impl Into<Variable> for FunctionArg {
    fn into(self) -> Variable {
        Variable {
            name: self.name,
            type_: self.type_,
            value: None,
            is_mutable: self.is_mutable,
        }
    }
}
