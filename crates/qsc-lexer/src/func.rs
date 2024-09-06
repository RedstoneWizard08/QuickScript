use std::collections::HashMap;

use chumsky::{
    error::Rich,
    primitive::{choice, just},
    IterParser, Parser,
};
use qsc_ast::{
    block::Block,
    func::{Argument, Function},
    token::Token,
};

use crate::{
    block::raw_block,
    parser::{make_parser, raw_ident},
    vis::visibility,
    CodeParser, Spanned,
};

pub fn func<'t, 's: 't>(
    expr: impl CodeParser<'t, 's>,
) -> impl CodeParser<'t, 's, Spanned<(Function, Spanned<String>)>> {
    let body = raw_block(expr.clone())
        .map(|v| {
            debug!("Found and parsed function body");
            v
        })
        .or(just(Token::Control(';'))
            .map_with(|_, x| (Block::empty(), x.span()))
            .map(|v| {
                debug!("Found no body, using a blank block");
                v
            }));

    visibility()
        .then(just(Token::Extern).or_not().map(|v| v.is_some()))
        .then_ignore(just(Token::Fn))
        .then(raw_ident().map_with(|v, x| (v, x.span())))
        .then(args())
        .then(ret())
        .map(|v| {
            debug!("Got past return type parsing (previous stack overflow spot)");
            debug!("Trying to parse a block");
            v
        })
        .then(body)
        .map_with(|(((((vis, external), name), args), ret), block), x| {
            let val = (
                (
                    {
                        Function {
                            args: args.0,
                            name: name.clone().0,
                            vis: vis.0,
                            ret: ret.0.map(|v| v.to_string()),
                            block: block.0,
                            external,
                        }
                    },
                    name,
                ),
                x.span(),
            );

            debug!("Parsed function: {:?}", val);

            val
        })
        .labelled("function")
}

fn ret<'t, 's: 't>() -> impl CodeParser<'t, 's, Spanned<Option<String>>> {
    just(Token::Operation("->"))
        .ignored()
        .then(raw_ident())
        .map(|(_, v)| v)
        .or_not()
        .map_with(|v, x| (v, x.span()))
        .labelled("function")
        .map(|v| {
            debug!("Found return type: {:?}", v);

            v
        })
}

fn args<'t, 's: 't>() -> impl CodeParser<'t, 's, Spanned<Vec<Argument>>> {
    let args = just(Token::Mut)
        .or_not()
        .map(|v| v.is_some())
        .then(raw_ident())
        .then_ignore(just(Token::Control(':')))
        .then(raw_ident())
        .map(|((mutable, name), typ)| Argument { name, typ, mutable })
        .separated_by(just(Token::Control(',')))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(just(Token::Control('(')), just(Token::Control(')')))
        .map_with(|v, x| (v, x.span()));

    let none = just(Token::Control('('))
        .ignored()
        .then_ignore(just(Token::Control(')')))
        .map_with(|_, x| (Vec::new(), x.span()));

    choice((none, args)).labelled("arguments").map(|v| {
        debug!("Found arguments: {:?}", v);

        v
    })
}

pub fn funcs<'t, 's: 't>() -> impl CodeParser<'t, 's, Spanned<HashMap<String, Spanned<Function>>>> {
    func(make_parser())
        .repeated()
        .collect::<Vec<_>>()
        .validate(|fs, _, emitter| {
            let mut funcs = HashMap::new();

            debug!("Collecting functions...");

            for ((f, (name, name_span)), s) in fs {
                if funcs.insert(name.clone(), (f, s)).is_some() {
                    emitter.emit(Rich::custom(
                        name_span,
                        format!("Function '{}' already exists", name),
                    ));
                }
            }

            funcs
        })
        .map_with(|v, x| (v, x.span()))
}
