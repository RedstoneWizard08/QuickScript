use chumsky::{primitive::just, IterParser, Parser};
use qsc_ast::{
    block::Block,
    cond::{Conditional, Else},
    expr::Expr,
    token::Token,
};

use crate::{block::raw_block, CodeParser};

pub fn cond<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's> {
    just(Token::If)
        .ignore_then(expr.clone())
        .then(raw_block(expr.clone()))
        .then(maybe_elses(expr.clone()))
        .then(maybe_else(expr.clone()))
        .map_with(|(((cond, body), elses), catch_all), x| {
            (
                Expr::Conditional(Conditional {
                    cond: Box::new(cond.0),
                    body: body.0,
                    elses,
                    catch_all,
                }),
                x.span(),
            )
        })
        .labelled("conditional")
        .map(|v| {
            debug!("Found conditional: {:?}", v);

            v
        })
}

fn maybe_elses<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's, Vec<Else>> {
    just(Token::Else)
        .ignored()
        .then_ignore(just(Token::If))
        .then(expr.clone())
        .then(raw_block(expr.clone()))
        .map(|((_, cond), body)| Else {
            cond: cond.0,
            body: body.0,
        })
        .repeated()
        .collect()
}

fn maybe_else<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's, Option<Block>> {
    just(Token::Else)
        .ignore_then(raw_block(expr.clone()))
        .map(|v| v.0)
        .or_not()
}
