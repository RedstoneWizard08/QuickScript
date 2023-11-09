//! Args:
//! [(name), (:), (type...), (,), (...)]

use anyhow::Result;

use crate::{
    tokenizer::{
        consumer::Cursor,
        token::{operator::Operator, ttype::TokenType},
    },
    types::Type,
};

use super::AstParser;

impl AstParser {
    pub fn parse_args(args: Vec<TokenType>) -> Result<Vec<(String, Type)>> {
        let mut iter = Cursor::new(args);
        let mut args = Vec::new();

        while let Some(item) = iter.next() {
            let name = item.get_name()?;

            iter.next_is(TokenType::Operator(Operator::Colon))?;

            let type_name = iter.next_result("Could not get a type name!")?;
            let type_;

            if iter.next_is(TokenType::Operator(Operator::Lt)).is_ok() {
                let mut args = iter.read_until_counted(
                    TokenType::Operator(Operator::Lt),
                    TokenType::Operator(Operator::Gt),
                );

                args.insert(0, TokenType::Operator(Operator::Lt));
                args.insert(0, type_name);

                type_ = Type::parse(args)?;

                iter.next();
            } else {
                type_ = Type::parse(vec![type_name])?;
            }

            args.push((name, type_));
        }

        Ok(args)
    }
}
