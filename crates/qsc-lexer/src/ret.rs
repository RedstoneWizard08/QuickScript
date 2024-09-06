use chumsky::{primitive::just, Parser};
use qsc_ast::{expr::Expr, token::Token};

use crate::CodeParser;

pub fn ret<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's> {
    just(Token::Return)
        .ignore_then(expr.clone().or_not())
        .then_ignore(just(Token::Control(';')))
        .map_with(|v, x| (Expr::Return(v.map(|v| Box::new(v.0))), x.span()))
        .map(|v| {
            debug!("Found return: {:?}", v);

            v
        })
}
