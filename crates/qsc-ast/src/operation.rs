use super::expr::Expr;

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
