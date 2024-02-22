use anyhow::Result;
use qsc_tokenizer::{cursor::TokenCursor, data::TokenData, punct::Punct};

use super::expr::{Expr, ExprKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub data: Option<Box<Expr>>,
}

impl Return {
    pub fn parse(tokens: &mut TokenCursor) -> Result<Return> {
        debug!("Trying to parse return from token: {:?}", tokens.peek());

        Ok(match tokens.next() {
            Some(token) => match token.content {
                TokenData::Name(name) => match name.as_str() {
                    "return" => {
                        let data = match tokens.peek().unwrap().content {
                            TokenData::Punct(Punct::Semicolon) => None,

                            _ => Some(Box::new(Expr::parse(tokens)?)),
                        };

                        Return { data }
                    }

                    _ => Return { data: None },
                },

                _ => Return { data: None },
            },

            None => Return { data: None },
        })
    }

    pub fn clean(&mut self) -> Self {
        if let Some(data) = &self.data {
            Return {
                data: match data.content {
                    ExprKind::None => None,

                    _ => Some(Box::new(data.clone().clean())),
                },
            }
        } else {
            Return { data: None }
        }
    }
}
