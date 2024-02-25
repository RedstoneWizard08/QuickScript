use std::ops::Range;

use crate::func::Function;

use super::{call::Call, literal::Literal, operation::Operation, var::Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub content: ExprKind,
    pub position: Range<usize>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ExprKind {
    Identifer(String),
    Literal(Literal),
    Operation(Operation),
    Variable(Variable),
    Function(Function),
    Call(Call),
    Return(Option<Box<Expr>>),
    Type(String, Option<Vec<Expr>>),
    Unary(bool, Box<ExprKind>),
    Block(Vec<Expr>),
    Eof,

    #[default]
    None,
}

impl Expr {
    pub fn type_name(&self) -> String {
        match &self.content {
            ExprKind::Literal(literal) => match literal {
                Literal::None => "ptr".to_string(),
                Literal::Boolean(_) => "bool".to_string(),
                Literal::Integer(_) => "i32".to_string(),
                Literal::Float(_) => "f32".to_string(),
                Literal::String(_) => "str".to_string(),
                Literal::Char(_) => "char".to_string(),
            },

            ExprKind::Operation(op) => {
                let data = op.data();

                if data.left.type_name() == data.right.type_name() {
                    data.left.type_name()
                } else {
                    "ptr".to_string()
                }
            }

            _ => "ptr".to_string(),
        }
    }
}
