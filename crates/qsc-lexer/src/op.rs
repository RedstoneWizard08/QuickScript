use chumsky::{
    primitive::{choice, just},
    Parser,
};

use qsc_ast::{
    expr::Expr,
    op::{BinaryOperation, Operation, Operator, UnaryOperation},
    token::Token,
};

use crate::CodeParser;

pub fn unary<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's> {
    just(Token::Operation("-"))
        .ignored()
        .then(expr.clone())
        .map(|(_, value)| UnaryOperation {
            negative: true,
            value: Box::new(value.0),
        })
        .or(just(Token::Operation("+"))
            .ignored()
            .then(expr.clone())
            .map(|(_, value)| UnaryOperation {
                negative: false,
                value: Box::new(value.0),
            }))
        .or(expr.map(|value| UnaryOperation {
            negative: false,
            value: Box::new(value.0),
        }))
        .map_with(|v, x| (Expr::Operation(Operation::Unary(v)), x.span()))
}

pub fn operator<'t, 's: 't>() -> impl CodeParser<'t, 's, Operator> {
    let vals = Operator::values()
        .iter()
        .map(|v| just(Token::Operation(v.to_string())).to(*v))
        .collect::<Vec<_>>();

    choice(vals)
}

pub fn binary<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's> {
    unary(expr.clone())
        .then(operator())
        .then(unary(expr))
        .map(|((lhs, op), rhs)| {
            Expr::Operation(Operation::Binary(BinaryOperation {
                left: Box::new(lhs.0),
                op,
                right: Box::new(rhs.0),
            }))
        })
        .map_with(|v, x| (v, x.span()))
}

pub fn op<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's> {
    binary(expr.clone()).or(unary(expr)).map(|v| {
        debug!("Found operation: {:?}", v);

        v
    })
}
