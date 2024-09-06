use crate::{CodeParser, Spanned};
use chumsky::{
    primitive::{empty, just},
    IterParser,
};
use qsc_ast::{block::Block, expr::Expr, token::Token};

pub fn raw_block<'t, 's: 't>(
    expr: impl CodeParser<'t, 's>,
) -> impl CodeParser<'t, 's, Spanned<Block>> {
    empty()
        .map(|v| {
            debug!("Beginning block parsing...");
            v
        })
        .ignore_then(
            expr.clone()
                .separated_by(just(Token::Control(';')))
                .collect::<Vec<Spanned<Expr>>>(),
        )
        .delimited_by(just(Token::Control('{')), just(Token::Control('}')))
        .map_with(|body, x| {
            (
                Block {
                    body: body.iter().map(|v| v.0.clone()).collect(),
                },
                x.span(),
            )
        })
        .labelled("block")
        .map(|v| {
            debug!("Found block: {:?}", v);

            v
        })
}

pub fn block<'t, 's: 't>(expr: impl CodeParser<'t, 's>) -> impl CodeParser<'t, 's> {
    raw_block(expr.clone())
        .map(|v| (Expr::Block(v.0), v.1))
        .labelled("block")
}
