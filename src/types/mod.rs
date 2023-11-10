use anyhow::Result;

use crate::tokenizer::{
    consumer::Cursor,
    token::{operator::Operator, ttype::TokenType},
};

pub mod clif;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Type {
    pub name: String,
    pub generic_args: Vec<Box<Type>>,
}

impl Type {
    pub fn parse(tokens: Vec<TokenType>) -> Result<Self> {
        if tokens
            == vec![
                TokenType::Operator(Operator::OpenParens),
                TokenType::Operator(Operator::CloseParens),
            ]
        {
            return Ok(Self {
                name: String::from("VOID"),
                generic_args: Vec::new(),
            });
        }

        if tokens == vec![] {
            return Ok(Self {
                name: String::from("ANY"),
                generic_args: Vec::new(),
            });
        }

        let mut iter = Cursor::new(tokens);

        let name = iter
            .next()
            .ok_or(anyhow!("Could not read type name!"))?
            .get_name()?;

        let mut generic_args = Vec::new();

        if iter.has_next() {
            iter.next_is(TokenType::Operator(Operator::Lt))?;

            let generics = iter.read_until_counted(
                TokenType::Operator(Operator::Lt),
                TokenType::Operator(Operator::Gt),
            );

            let mut generics = Cursor::new(generics);

            while let Some(generic) = generics.next() {
                if generic == TokenType::Operator(Operator::Comma) {
                    continue;
                }

                if generics
                    .next_is_peek(TokenType::Operator(Operator::Lt), 0)
                    .is_ok()
                {
                    let mut args = generics.read_until_counted(
                        TokenType::Operator(Operator::Lt),
                        TokenType::Operator(Operator::Gt),
                    );

                    args.insert(0, generic);
                    args.insert(0, TokenType::Operator(Operator::Lt));
                    args.push(TokenType::Operator(Operator::Gt));

                    generic_args.push(Box::new(Type::parse(args)?));
                } else {
                    generic_args.push(Box::new(Type::parse(vec![generic])?));
                }
            }
        }

        Ok(Self { name, generic_args })
    }
}
