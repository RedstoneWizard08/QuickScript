#![feature(trait_alias, box_into_inner, type_alias_impl_trait)]

use chumsky::{error::Rich, extra::Err, input::SpannedInput, span::SimpleSpan, Parser};
use qsc_ast::{expr::Expr, token::Token};

pub type Span = SimpleSpan<usize>;
pub type Spanned<T, S = Span> = (T, S);
pub type ParserInput<'t, 's> = SpannedInput<Token<'s>, Span, &'t [(Token<'s>, Span)]>;

pub trait RawCodeParser<'t, 's: 't, T = Spanned<Expr>> =
    Parser<'t, ParserInput<'t, 's>, T, Err<Rich<'t, Token<'s>, Span>>>;

pub trait CodeParser<'t, 's: 't, T = Spanned<Expr>> = RawCodeParser<'t, 's, T> + Clone;

#[macro_use]
extern crate log;

pub mod block;
pub mod call;
pub mod cond;
pub mod func;
pub mod lexer;
pub mod literal;
pub mod op;
pub mod parser;
pub mod ret;
pub mod token;
pub mod var;
pub mod vis;

#[cfg(test)]
pub mod test;
