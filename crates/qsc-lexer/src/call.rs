use chumsky::{primitive::just, IterParser, Parser};
use qsc_ast::{call::Call, expr::Expr, token::Token};

use crate::{parser::raw_ident, CodeParser, Spanned};

pub fn call<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's> {
    raw_ident()
        .then_ignore(just(Token::Control('(')))
        .then(
            expr.clone()
                .separated_by(just(Token::Control(',')))
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(just(Token::Control(')')))
        .map_with(|(func, args): (String, Vec<Spanned<Expr>>), x| {
            (
                Expr::Call(Call {
                    func,
                    args: args.iter().map(|v| v.0.clone()).collect(),
                }),
                x.span(),
            )
        })
        .map(|v| {
            debug!("Found call: {:?}", v);

            v
        })
}
