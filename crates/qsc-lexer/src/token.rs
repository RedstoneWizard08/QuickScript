use chumsky::{
    error::Rich,
    extra::Err,
    primitive::{any, choice, end, just, none_of, one_of},
    recovery::skip_then_retry_until,
    text::{digits, ident, int},
    IterParser, Parser,
};
use itertools::Itertools;
use qsc_ast::{
    op::Operator,
    token::{Token, Value},
};

use crate::{Span, Spanned};

pub fn tokens<'s>() -> impl Parser<'s, &'s str, Vec<Spanned<Token<'s>>>, Err<Rich<'s, char, Span>>>
{
    let b10_int = int(10)
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Value::Int)
        .map(Token::Value);

    let hex_int = just("0x").ignore_then(
        int::<&'s str, char, Err<Rich<'s, char, Span>>>(16)
            .to_slice()
            .map(|v| i64::from_str_radix(v, 16))
            .unwrapped()
            .map(Value::Int)
            .map(Token::Value),
    );

    let bin_int = just("0b").ignore_then(
        int::<&'s str, char, Err<Rich<'s, char, Span>>>(2)
            .to_slice()
            .map(|v| i64::from_str_radix(v, 2))
            .unwrapped()
            .map(Value::Int)
            .map(Token::Value),
    );

    let float = int(10)
        .then(just('.').then(digits(10)))
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Value::Float)
        .map(Token::Value);

    let num = float.or(b10_int).or(hex_int).or(bin_int);

    let string = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .to_slice()
        .map(Value::String)
        .map(Token::Value);

    let ch = just('\'')
        .ignore_then(none_of('\''))
        .then_ignore(just('\''))
        .map(Value::Char)
        .map(Token::Value);

    let ops = Operator::values()
        .iter()
        .map(|v| v.to_string().chars().collect::<Vec<char>>())
        .flatten()
        .unique()
        .collect::<String>();

    let op = one_of(ops)
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Token::Operation);

    let ctrl = one_of("()[]{}:;,").map(Token::Control);

    let ident = ident().map(|ident: &str| match ident {
        "fn" => Token::Fn,
        "let" => Token::Let,
        "mut" => Token::Mut,
        "if" => Token::If,
        "else" => Token::Else,
        "true" => Token::Value(Value::Bool(true)),
        "false" => Token::Value(Value::Bool(false)),
        "extern" => Token::Extern,
        "return" => Token::Return,
        _ => Token::Ident(ident),
    });

    let token = choice((num, string, ch, op, ctrl, ident));

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
