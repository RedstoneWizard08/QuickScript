use std::ops::Range;

use anyhow::Result;

use crate::{
    lexer::cursor::TokenCursor,
    tokenizer::{data::TokenData, punct::Punct},
};

use super::{call::Call, literal::Literal, operation::Operation, ret::Return, var::Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub content: ExprKind,
    pub position: Range<usize>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ExprKind {
    Identifer(String),
    Literal(Literal),
    Operation(Operation),
    Variable(Variable),
    Call(Call),
    Return(Return),
    Eof,

    #[default]
    None,
}

impl Expr {
    pub fn parse(tokens: &mut TokenCursor) -> Result<Expr> {
        let start = tokens.position;

        debug!("Trying to parse expr from token: {:?}", tokens.peek());

        Ok(match tokens.peek() {
            Some(token) => match token.content {
                TokenData::Punct(Punct::Semicolon) => {
                    tokens.next();

                    Expr {
                        content: ExprKind::None,

                        position: Range {
                            start,
                            end: tokens.position + 1,
                        },
                    }
                }

                TokenData::Name(name) => match name.as_str() {
                    "fn" | "let" => match Variable::parse(tokens) {
                        Ok(variable) => Expr {
                            content: ExprKind::Variable(variable),

                            position: Range {
                                start,
                                end: tokens.position + 1,
                            },
                        },

                        Err(_) => Expr {
                            content: ExprKind::None,

                            position: Range {
                                start,
                                end: tokens.position + 1,
                            },
                        },
                    },

                    _ => Expr {
                        content: if tokens.peek_at(1).unwrap().content
                            == TokenData::Punct(Punct::OpenParen)
                        {
                            Call::parse(tokens).map(ExprKind::Call)?
                        } else if tokens.peek_at(1).unwrap().content
                            == TokenData::Name("return".to_string())
                        {
                            Return::parse(tokens).map(ExprKind::Return)?
                        } else {
                            ExprKind::Identifer(tokens.next().unwrap().content.as_name()?)
                        },

                        position: Range {
                            start,
                            end: tokens.position + 1,
                        },
                    },
                },

                _ => Expr {
                    content: match Literal::parse(tokens) {
                        Ok(literal) => ExprKind::Literal(literal),

                        Err(_) => match Operation::parse(tokens) {
                            Ok(operation) => ExprKind::Operation(operation),

                            Err(_) => ExprKind::None,
                        },
                    },

                    position: Range {
                        start,
                        end: tokens.position + 1,
                    },
                },
            },

            None => Expr {
                content: ExprKind::Eof,

                position: Range {
                    start,
                    end: tokens.position + 1,
                },
            },
        })
    }

    pub fn clean(&mut self) -> Self {
        self.content = match self.content.clone() {
            ExprKind::Call(mut call) => ExprKind::Call(call.clean()),
            ExprKind::Return(mut ret) => ExprKind::Return(ret.clean()),

            ExprKind::Operation(mut operation) => match operation {
                Operation::None => ExprKind::None,
                _ => ExprKind::Operation(operation.clean()),
            },

            ExprKind::Variable(mut variable) => match variable {
                Variable::None => ExprKind::None,
                _ => ExprKind::Variable(variable.clean()),
            },

            ExprKind::Literal(literal) => match literal {
                Literal::None => ExprKind::None,
                _ => ExprKind::Literal(literal),
            },

            ExprKind::Identifer(ident) => match ident.as_str() {
                "" => ExprKind::None,
                _ => ExprKind::Identifer(ident),
            },

            ExprKind::Eof => ExprKind::Eof,
            ExprKind::None => ExprKind::None,
        };

        self.clone()
    }
}
