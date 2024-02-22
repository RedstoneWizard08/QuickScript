use super::expr::Expr;

use anyhow::Result;
use qsc_tokenizer::{cursor::TokenCursor, data::TokenData, operator::Operator as TOperator};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Operation {
    Add(OperationData),
    Subtract(OperationData),
    Multiply(OperationData),
    Divide(OperationData),
    Modulo(OperationData),
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

        match tokens.peek_at(1) {
            Some(tkn) => match tkn.content {
                TokenData::Operator(op) => {
                    tokens.remove(1);

                    match op {
                        TOperator::Add => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Add(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Sub => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Subtract(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Mul => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Multiply(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Div => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Divide(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Mod => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Modulo(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::And => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::And(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Or => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Or(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Xor => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Xor(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Not => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Not(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::BitwiseAnd => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::BitwiseAnd(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::BitwiseOr => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::BitwiseOr(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::BitwiseNot => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::BitwiseNot(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Equal => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Equal(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::NotEqual => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::NotEqual(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Greater => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Greater(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Less => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Less(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::GreaterEqual => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::GreaterEqual(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::LessEqual => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::LessEqual(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::Assign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::Assign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::AddAssign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::AddAssign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::SubAssign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::SubtractAssign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::MulAssign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::MultiplyAssign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::DivAssign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::DivideAssign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::ModAssign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::ModuloAssign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::BitwiseAndAssign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::BitwiseAndAssign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::BitwiseOrAssign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::BitwiseOrAssign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::BitwiseNotAssign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::BitwiseNotAssign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        TOperator::XorAssign => {
                            let left = Expr::parse(tokens)?;
                            let right = Expr::parse(tokens)?;

                            Ok(Operation::XorAssign(OperationData {
                                left: Box::new(left),
                                right: Box::new(right),
                            }))
                        }

                        _ => Ok(Operation::None),
                    }
                }

                _ => Ok(Operation::None),
            },

            _ => Ok(Operation::None),
        }
    }

    pub fn clean(&mut self) -> Self {
        match self {
            Operation::Add(data) => Operation::Add(data.clean()),
            Operation::Subtract(data) => Operation::Subtract(data.clean()),
            Operation::Multiply(data) => Operation::Multiply(data.clean()),
            Operation::Divide(data) => Operation::Divide(data.clean()),
            Operation::Modulo(data) => Operation::Modulo(data.clean()),
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
