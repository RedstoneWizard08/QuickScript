use anyhow::Result;

use crate::{
    expect,
    lexer::cursor::TokenCursor,
    tokenizer::{data::TokenData, punct::Punct},
};

use super::expr::{Expr, ExprKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub name: String,
    pub args: Vec<Expr>,
}

impl Call {
    pub fn parse(tokens: &mut TokenCursor) -> Result<Call> {
        debug!("Trying to parse call from token: {:?}", tokens.peek());

        let name = tokens.next().unwrap().content.as_name()?;
        let mut args = Vec::new();

        expect!(
            tokens,
            tokens.next().unwrap(),
            TokenData::Punct(Punct::OpenParen)
        );

        while tokens.peek().unwrap().content != TokenData::Punct(Punct::CloseParen) {
            args.push(Expr::parse(tokens)?);

            match tokens.peek().unwrap().content {
                TokenData::Punct(Punct::Comma) => {
                    tokens.next();
                }

                _ => {}
            }
        }

        expect!(
            tokens,
            tokens.next().unwrap(),
            TokenData::Punct(Punct::CloseParen)
        );

        Ok(Call { name, args })
    }

    pub fn clean(&mut self) -> Self {
        let mut new_args = Vec::new();

        for mut arg in self.args.clone() {
            if arg.content == ExprKind::None {
                continue;
            }

            new_args.push(arg.clean());
        }

        self.args = new_args;
        self.clone()
    }
}
