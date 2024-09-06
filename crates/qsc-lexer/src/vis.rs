use chumsky::{
    primitive::{choice, just},
    Parser,
};
use qsc_ast::{token::Token, vis::Visibility};

use crate::{CodeParser, Spanned};

pub fn visibility<'t, 's: 't>() -> impl CodeParser<'t, 's, Spanned<Visibility>> {
    let vals = Visibility::values()
        .iter()
        .map(|v| just(Token::Ident(v.to_string())).to(*v))
        .collect::<Vec<_>>();

    choice(vals)
        .or_not()
        .map_with(|v, x| {
            (
                match v {
                    Some(val) => val,
                    None => Visibility::Public,
                },
                x.span(),
            )
        })
        .labelled("visibility")
}
