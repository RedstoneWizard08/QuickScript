use chumsky::{primitive::choice, recursive::recursive, select, Parser};
use qsc_ast::{expr::Expr, token::Token};

use crate::{call::call, cond::cond, literal::literal, op::op, ret::ret, var::var, CodeParser};

pub fn make_parser<'t, 's: 't>() -> impl CodeParser<'t, 's> {
    debug!("Creating parser...");

    recursive(|expr| {
        choice((
            literal(),
            ident(),
            op(expr.clone()),
            call(expr.clone()),
            ret(expr.clone()),
            var(expr.clone()),
            cond(expr.clone()),
        ))
        .labelled("expression")
        .as_context()
    })
}

pub fn ident<'t, 's: 't>() -> impl CodeParser<'t, 's> {
    raw_ident()
        .labelled("identifier")
        .map_with(|v, x| (Expr::Name(v), x.span()))
}

pub fn raw_ident<'t, 's: 't>() -> impl CodeParser<'t, 's, String> {
    select! { Token::Ident(val) => val.to_string() }
        .labelled("identifier")
        .map(|v| {
            debug!("Found identifier: {}", v);

            v
        })
}
