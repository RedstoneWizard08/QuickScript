use anyhow::Result;

use qsc_core::{expect, throw};

use qsc_tokenizer::{
    cursor::TokenCursor, data::TokenData, error::Error, operator::Operator, punct::Punct,
};

use super::expr::{Expr, ExprKind};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Variable {
    Function(FunctionData),
    Variable(VariableData),

    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionData {
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub body: Box<Vec<Expr>>,
    pub return_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionArg {
    pub name: String,
    pub type_: String,
    pub is_mutable: bool,
}

impl Into<VariableData> for FunctionArg {
    fn into(self) -> VariableData {
        VariableData {
            name: self.name,
            type_: self.type_,
            value: None,
            is_mutable: self.is_mutable,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableData {
    pub name: String,
    pub type_: String,
    pub value: Option<Box<Expr>>,
    pub is_mutable: bool,
}

impl Variable {
    pub fn parse(tokens: &mut TokenCursor) -> Result<Variable> {
        debug!("Trying to parse variable from token: {:?}", tokens.peek());

        Ok(match tokens.next() {
            Some(token) => match token.content {
                TokenData::Name(name) => match name.as_str() {
                    "fn" => {
                        let name = tokens.next().unwrap().content.as_name()?;
                        let mut args = Vec::new();
                        let mut body = Vec::new();

                        expect!(
                            tokens,
                            tokens.next().unwrap(),
                            TokenData::Punct(Punct::OpenParen)
                        );

                        while tokens.peek().unwrap().content != TokenData::Punct(Punct::CloseParen)
                        {
                            let mut name = tokens.next().unwrap().content.as_name()?;
                            let mut is_mutable = false;

                            if name == "mut" {
                                is_mutable = true;
                                name = tokens.next().unwrap().content.as_name()?;
                            }

                            expect!(
                                tokens,
                                tokens.next().unwrap(),
                                TokenData::Punct(Punct::Colon)
                            );

                            let type_ = tokens.next().unwrap().content.as_name()?;

                            args.push(FunctionArg {
                                name,
                                type_,
                                is_mutable,
                            });

                            match tokens.peek().unwrap().content {
                                TokenData::Punct(Punct::Comma) => {
                                    tokens.next();
                                }

                                _ => {}
                            }
                        }

                        tokens.next();

                        let mut return_type = "void".to_string();

                        if let Some(TokenData::Operator(Operator::Sub)) =
                            tokens.peek().map(|t| t.content)
                        {
                            if let Some(TokenData::Operator(Operator::Greater)) =
                                tokens.peek_at(1).map(|t| t.content)
                            {
                                tokens.next();
                                tokens.next();

                                return_type = tokens.next().unwrap().content.as_name()?;

                                expect!(
                                    tokens,
                                    tokens.next().unwrap(),
                                    TokenData::Punct(Punct::OpenBrace)
                                );
                            } else {
                                let token = tokens.next().unwrap();

                                let err = Error::UnexpectedToken {
                                    token: format!("{:?}", token.content.clone()),
                                    data: tokens.cursor.all_data.clone(),
                                    file: tokens.cursor.file.clone(),
                                    pos: token.position,
                                };

                                throw!(err);
                            }
                        } else {
                            expect!(
                                tokens,
                                tokens.next().unwrap(),
                                TokenData::Punct(Punct::OpenBrace)
                            );
                        }

                        while tokens.peek().unwrap().content != TokenData::Punct(Punct::CloseBrace)
                        {
                            body.push(Expr::parse(tokens)?);
                        }

                        tokens.next();

                        Variable::Function(FunctionData {
                            name,
                            args,
                            body: Box::new(body),
                            return_type,
                        })
                    }

                    "let" => {
                        let is_mutable = match tokens.peek().unwrap().content {
                            TokenData::Name(name) => {
                                if name == "mut" {
                                    tokens.next();

                                    true
                                } else {
                                    false
                                }
                            }

                            _ => false,
                        };

                        let name = tokens.next().unwrap().content.as_name()?;

                        expect!(
                            tokens,
                            tokens.next().unwrap(),
                            TokenData::Punct(Punct::Colon)
                        );

                        let type_ = tokens.next().unwrap().content.as_name()?;

                        let value = match tokens.peek().unwrap().content {
                            TokenData::Operator(Operator::Assign) => {
                                tokens.next();

                                Some(Box::new(Expr::parse(tokens)?))
                            }

                            _ => None,
                        };

                        Variable::Variable(VariableData {
                            name,
                            type_,
                            value,
                            is_mutable,
                        })
                    }

                    _ => Variable::None,
                },

                _ => Variable::None,
            },

            None => Variable::None,
        })
    }

    pub fn clean(&mut self) -> Self {
        match self {
            Variable::Function(data) => Variable::Function(data.clean()),
            Variable::Variable(data) => Variable::Variable(data.clean()),

            Variable::None => Variable::None,
        }
    }
}

impl FunctionData {
    pub fn clean(&mut self) -> Self {
        let mut new_body = Vec::new();

        for mut expr in (&*self.body).clone() {
            if expr.content == ExprKind::None {
                continue;
            }

            new_body.push(expr.clean());
        }

        FunctionData {
            name: self.name.clone(),
            args: self.args.clone(),
            body: Box::new(new_body),
            return_type: self.return_type.clone(),
        }
    }
}

impl VariableData {
    pub fn clean(&mut self) -> Self {
        let value = match &self.value {
            Some(value) => match value.content {
                ExprKind::None => None,

                _ => Some(Box::new(value.clone().clean())),
            },

            None => None,
        };

        VariableData {
            name: self.name.clone(),
            type_: self.type_.clone(),
            value,
            is_mutable: self.is_mutable,
        }
    }
}
