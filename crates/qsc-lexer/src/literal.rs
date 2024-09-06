use chumsky::{select, Parser};

use qsc_ast::{
    expr::Expr,
    literal::Literal,
    token::{Token, Value},
};

use crate::CodeParser;

pub fn literal<'t, 's: 't>() -> impl CodeParser<'t, 's> {
    let val = select! {
        Token::Value(Value::String(s)) => Literal::String(s.to_string()),
        Token::Value(Value::Bool(b)) => Literal::Bool(b),
        Token::Value(Value::Int(i)) => Literal::Int(i),
        Token::Value(Value::Float(f)) => Literal::Float(f),
        Token::Value(Value::Char(c)) => Literal::Char(c),
    }
    .labelled("literal");

    val.map_with(|v, x| (Expr::Literal(v), x.span())).map(|v| {
        debug!("Found literal: {:?}", v);

        v
    })
}
