use chumsky::{primitive::just, Parser};
use qsc_ast::{expr::Expr, token::Token, var::Variable};

use crate::{parser::raw_ident, CodeParser, Spanned};

pub fn var<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's> {
    just(Token::Let)
        .ignore_then(just(Token::Mut).or_not().map(|v| v.is_some()))
        .then(raw_ident())
        .then(maybe_type())
        .then(maybe_val(expr.clone()))
        .map_with(|(((mutable, name), typ), value), x| {
            (
                Expr::Variable(Variable {
                    name,
                    typ,
                    value: value.map(|v| Box::new(v.0)),
                    mutable,
                }),
                x.span(),
            )
        })
        .map(|v| {
            debug!("Found variable: {:?}", v);

            v
        })
}

fn maybe_type<'t, 's: 't>() -> impl CodeParser<'t, 's, Option<String>> {
    just(Token::Control(':')).ignore_then(raw_ident()).or_not()
}

fn maybe_val<'t, 's: 't>(
    expr: impl CodeParser<'t, 's>,
) -> impl CodeParser<'t, 's, Option<Spanned<Expr>>> {
    just(Token::Operation("="))
        .ignore_then(expr.clone())
        .map(|v| v)
        .or_not()
}
