use super::expr::Expr;
use crate::lexer::cursor::TokenCursor;
use anyhow::Result;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Operation {
    Add(OperationData),
    Subtract(OperationData),
    Multiply(OperationData),
    Divide(OperationData),
    Modulo(OperationData),
    Power(OperationData),
    And(OperationData),
    Or(OperationData),
    Xor(OperationData),
    Not(OperationData),
    BitwiseAnd(OperationData),
    BitwiseOr(OperationData),
    BitwiseNot(OperationData),
    Equal(OperationData),
    NotEqual(OperationData),
    Greater(OperationData),
    Less(OperationData),
    GreaterEqual(OperationData),
    LessEqual(OperationData),
    Assign(OperationData),
    AddAssign(OperationData),
    SubtractAssign(OperationData),
    MultiplyAssign(OperationData),
    DivideAssign(OperationData),
    ModuloAssign(OperationData),
    BitwiseAndAssign(OperationData),
    BitwiseOrAssign(OperationData),
    BitwiseNotAssign(OperationData),
    XorAssign(OperationData),

    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationData {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl Operation {
    pub fn parse(tokens: &mut TokenCursor) -> Result<Operation> {
        debug!("Trying to parse operation from token: {:?}", tokens.peek());

        tokens.next();

        Ok(Operation::None)
    }

    pub fn clean(&mut self) -> Self {
        match self {
            Operation::Add(data) => Operation::Add(data.clean()),
            Operation::Subtract(data) => Operation::Subtract(data.clean()),
            Operation::Multiply(data) => Operation::Multiply(data.clean()),
            Operation::Divide(data) => Operation::Divide(data.clean()),
            Operation::Modulo(data) => Operation::Modulo(data.clean()),
            Operation::Power(data) => Operation::Power(data.clean()),
            Operation::And(data) => Operation::And(data.clean()),
            Operation::Or(data) => Operation::Or(data.clean()),
            Operation::Xor(data) => Operation::Xor(data.clean()),
            Operation::Not(data) => Operation::Not(data.clean()),
            Operation::BitwiseAnd(data) => Operation::BitwiseAnd(data.clean()),
            Operation::BitwiseOr(data) => Operation::BitwiseOr(data.clean()),
            Operation::BitwiseNot(data) => Operation::BitwiseNot(data.clean()),
            Operation::Equal(data) => Operation::Equal(data.clean()),
            Operation::NotEqual(data) => Operation::NotEqual(data.clean()),
            Operation::Greater(data) => Operation::Greater(data.clean()),
            Operation::Less(data) => Operation::Less(data.clean()),
            Operation::GreaterEqual(data) => Operation::GreaterEqual(data.clean()),
            Operation::LessEqual(data) => Operation::LessEqual(data.clean()),
            Operation::Assign(data) => Operation::Assign(data.clean()),
            Operation::AddAssign(data) => Operation::AddAssign(data.clean()),
            Operation::SubtractAssign(data) => Operation::SubtractAssign(data.clean()),
            Operation::MultiplyAssign(data) => Operation::MultiplyAssign(data.clean()),
            Operation::DivideAssign(data) => Operation::DivideAssign(data.clean()),
            Operation::ModuloAssign(data) => Operation::ModuloAssign(data.clean()),
            Operation::BitwiseAndAssign(data) => Operation::BitwiseAndAssign(data.clean()),
            Operation::BitwiseOrAssign(data) => Operation::BitwiseOrAssign(data.clean()),
            Operation::BitwiseNotAssign(data) => Operation::BitwiseNotAssign(data.clean()),
            Operation::XorAssign(data) => Operation::XorAssign(data.clean()),

            Operation::None => Operation::None,
        }
    }

    pub fn data(&self) -> &OperationData {
        match self {
            Operation::Add(data) => data,
            Operation::Subtract(data) => data,
            Operation::Multiply(data) => data,
            Operation::Divide(data) => data,
            Operation::Modulo(data) => data,
            Operation::Power(data) => data,
            Operation::And(data) => data,
            Operation::Or(data) => data,
            Operation::Xor(data) => data,
            Operation::Not(data) => data,
            Operation::BitwiseAnd(data) => data,
            Operation::BitwiseOr(data) => data,
            Operation::BitwiseNot(data) => data,
            Operation::Equal(data) => data,
            Operation::NotEqual(data) => data,
            Operation::Greater(data) => data,
            Operation::Less(data) => data,
            Operation::GreaterEqual(data) => data,
            Operation::LessEqual(data) => data,
            Operation::Assign(data) => data,
            Operation::AddAssign(data) => data,
            Operation::SubtractAssign(data) => data,
            Operation::MultiplyAssign(data) => data,
            Operation::DivideAssign(data) => data,
            Operation::ModuloAssign(data) => data,
            Operation::BitwiseAndAssign(data) => data,
            Operation::BitwiseOrAssign(data) => data,
            Operation::BitwiseNotAssign(data) => data,
            Operation::XorAssign(data) => data,

            Operation::None => panic!("Operation::None has no data!"),
        }
    }
}

impl OperationData {
    pub fn clean(&mut self) -> Self {
        OperationData {
            left: Box::new(self.left.clean()),
            right: Box::new(self.right.clean()),
        }
    }
}
