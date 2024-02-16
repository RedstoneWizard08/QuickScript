use anyhow::Result;

use crate::{
    lexer::cursor::TokenCursor,
    throw,
    tokenizer::{data::TokenData, error::Error, punct::Punct},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub enum Literal {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Char(char),

    #[default]
    None,
}

impl Literal {
    pub fn parse(tokens: &mut TokenCursor) -> Result<Literal> {
        debug!("Trying to parse literal from token: {:?}", tokens.peek());

        Ok(match tokens.next() {
            Some(token) => match token.content {
                TokenData::Number(number) => match tokens.peek() {
                    Some(token) => match token.content {
                        TokenData::Punct(Punct::Dot) => {
                            let dot = tokens.next().unwrap();
                            let next_token = tokens.next().unwrap();

                            match next_token.content {
                                TokenData::Number(next_number) => {
                                    Literal::Float(format!("{}.{}", number, next_number).parse()?)
                                }

                                _ => {
                                    let err = Error::UnexpectedToken {
                                        token: String::from("."),
                                        file: tokens.cursor.file.clone(),
                                        data: tokens.cursor.all_data.clone(),
                                        pos: dot.position,
                                    };

                                    throw!(err);
                                }
                            }
                        }

                        _ => Literal::Integer(number),
                    },

                    None => Literal::Integer(number),
                },

                TokenData::String(string) => Literal::String(string),
                TokenData::Boolean(boolean) => Literal::Boolean(boolean),
                TokenData::Char(char) => Literal::Char(char),

                _ => Literal::None,
            },

            None => Literal::None,
        })
    }
}
