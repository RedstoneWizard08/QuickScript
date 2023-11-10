use crate::types::Type;
use cranelift::prelude::{types, Type as ClifType};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operation {
    /// lhs, rhs
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),

    /// var, value
    Assign(Box<Expression>, Box<Expression>),

    /// lhs, rhs
    Equals(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
    GtEq(Box<Expression>, Box<Expression>),
    LtEq(Box<Expression>, Box<Expression>),

    /// var, index
    Index(String, usize),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Definition {
    /// name, type, value.
    Variable(String, Type, Box<Expression>),

    /// name, args, return_type, content
    Function(String, Vec<Box<Definition>>, Type, Vec<Box<Expression>>),

    /// name, type
    Argument(String, Type),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Define(Definition),
    Type(Type),
    String(String),
    Number(i32),
    Float(f32),
    Identifier(String),
    Field(String, String),
    Operation(Operation),
    Block(Vec<Box<Expression>>),
    Return(Box<Expression>),
    None,

    /// TODO: Support nested properties
    Property(String, String),

    /// condition, block, elses (can be just a block or another if statement)
    If(Vec<Box<Expression>>, Vec<Box<Expression>>),

    /// name, args
    Call(String, Vec<Box<Expression>>),

    /// object name, name, args
    MethodCall(Vec<String>, String, Vec<Box<Expression>>),
}

impl Expression {
    pub fn get_type(self) -> ClifType {
        match self {
            Expression::String(_) => types::I64,
            Expression::Number(_) => types::I32,
            Expression::Float(_) => types::F32,
            Expression::Identifier(_) => types::I32,

            _ => types::I32,
        }
    }
}
