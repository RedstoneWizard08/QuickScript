use pest::iterators::Pair;
use qsc_ast::{
    expr::ExprKind,
    operation::{Operation, OperationData},
};

use crate::parser::{Lexer, Rule};

impl Lexer {
    pub fn binary_op<'i>(&self, pair: &Pair<'i, Rule>) -> ExprKind {
        let mut inner = pair.clone().into_inner();
        let left = self.parse_expr(inner.next().unwrap());
        let op = inner.next().unwrap().as_str().trim().to_string();
        let right = self.parse_expr(inner.next().unwrap());

        ExprKind::Operation(match op.as_str() {
            "+" => Operation::Add(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "-" => Operation::Subtract(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "*" => Operation::Multiply(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "/" => Operation::Divide(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "%" => Operation::Modulo(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "&&" => Operation::And(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "||" => Operation::Or(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "^" => Operation::Xor(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "!" => Operation::Not(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "&" => Operation::BitwiseAnd(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "|" => Operation::BitwiseOr(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "~" => Operation::BitwiseNot(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "==" => Operation::Equal(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "!=" => Operation::NotEqual(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            ">" => Operation::Greater(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "<" => Operation::Less(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            ">=" => Operation::GreaterEqual(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "<=" => Operation::LessEqual(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "=>" => Operation::Assign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "+=" => Operation::AddAssign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "-=" => Operation::SubtractAssign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "*=" => Operation::MultiplyAssign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "/=" => Operation::DivideAssign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "%=" => Operation::ModuloAssign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "&=" => Operation::BitwiseAndAssign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "|=" => Operation::BitwiseOrAssign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "~=" => Operation::BitwiseNotAssign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            "^=" => Operation::XorAssign(OperationData {
                left: Box::new(left),
                right: Box::new(right),
            }),

            _ => unreachable!(),
        })
    }
}
