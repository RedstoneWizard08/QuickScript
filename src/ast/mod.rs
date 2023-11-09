use anyhow::{Error, Result};

use crate::{
    tokenizer::{
        consumer::Cursor,
        token::{operator::Operator, ttype::TokenType},
    },
    util::AsCharVec,
};

use self::expr::{Expression, Operation};

pub mod args;
pub mod call;
pub mod expr;
pub mod func;
pub mod ifelse;
pub mod var;

pub fn or_err<T>(opt: Option<T>, err: Error) -> Result<T> {
    opt.ok_or(err)
}

pub fn is_next_decimal(iter: &mut Cursor<TokenType>) -> bool {
    if let Some(item) = iter.peek(1) {
        if let Ok(op) = item.get_operator() {
            if op == Operator::Decimal {
                return true;
            }
        }
    }

    return false;
}

pub struct AstParser {
    pub iter: Cursor<TokenType>,
    pub tokens: Vec<TokenType>,
    pub exprs: Vec<Expression>,
}

impl AstParser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Self {
            iter: Cursor::new(tokens.clone()),
            tokens,
            exprs: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        while self.iter.has_next() {
            let expr = self.parse_one()?;

            self.exprs.push(expr);
        }

        self.exprs = self
            .exprs
            .iter()
            .cloned()
            .filter(|v| v != &Expression::None)
            .collect();

        Ok(())
    }

    pub fn parse_one(&mut self) -> Result<Expression> {
        Ok(match self.iter.next().unwrap() {
            TokenType::Name(name) => match String::from_iter(name).as_str() {
                "fn" => self.parse_fn()?,
                "if" => self.parse_if()?,
                "let" => self.parse_let()?,
                "return" => Expression::Return(Box::new(self.parse_one()?)),

                name => {
                    let peeked = self
                        .iter
                        .peek_until(TokenType::Operator(Operator::OpenParens));

                    if peeked
                        .iter()
                        .all(|v| v == &TokenType::Operator(Operator::Decimal) || v.is_name())
                        && peeked.contains(&TokenType::Operator(Operator::Decimal))
                    {
                        let mut names = self
                            .iter
                            .read_until(TokenType::Operator(Operator::OpenParens));

                        names.insert(0, TokenType::Name(name.as_char_vec()));

                        let mut names = names
                            .iter()
                            .cloned()
                            .filter(|v| v != &TokenType::Operator(Operator::Decimal))
                            .collect::<Vec<TokenType>>();

                        let method = names.pop().unwrap();

                        let names = names
                            .iter()
                            .map(|v| v.get_name().unwrap())
                            .collect::<Vec<String>>();

                        let args = self.iter.read_until_counted(
                            TokenType::Operator(Operator::OpenParens),
                            TokenType::Operator(Operator::CloseParens),
                        );

                        let mut parser = AstParser::new(args);

                        parser.parse()?;

                        let args = parser
                            .exprs
                            .iter()
                            .map(|v| Box::new(v.clone()))
                            .collect::<Vec<Box<Expression>>>();

                        return Ok(Expression::MethodCall(names, method.get_name()?, args));
                    } else if self
                        .iter
                        .next_is_peek(TokenType::Operator(Operator::OpenParens), 0)
                        .is_ok()
                    {
                        self.iter.next();

                        let args = self.iter.read_until_counted(
                            TokenType::Operator(Operator::OpenParens),
                            TokenType::Operator(Operator::CloseParens),
                        );

                        let mut parser = AstParser::new(args);

                        parser.parse()?;

                        let args = parser
                            .exprs
                            .iter()
                            .map(|v| Box::new(v.clone()))
                            .collect::<Vec<Box<Expression>>>();

                        return Ok(Expression::Call(name.to_string(), args));
                    } else if self
                        .iter
                        .next_is_peek(TokenType::Operator(Operator::OpenSqBracket), 0)
                        .is_ok()
                    {
                        self.iter.next();

                        let idx = self
                            .iter
                            .next_result("Cannot read index from array!")?
                            .get_number()?;

                        self.iter.next();

                        return Ok(Expression::Operation(Operation::Index(
                            name.to_string(),
                            idx as usize,
                        )));
                    }

                    Expression::Identifier(name.to_string())
                }
            },

            TokenType::String(val) => Expression::String(String::from_iter(val)),

            TokenType::Number(val) => {
                let n: i32 =
                    String::from_iter(val.iter().map(|v| v.to_string()).collect::<Vec<String>>())
                        .parse()?;

                if self
                    .iter
                    .next_is_peek(TokenType::Operator(Operator::Decimal), 0)
                    .is_ok()
                    && self.iter.peek(1).unwrap_or(TokenType::None).is_number()
                {
                    self.iter.next();

                    let n2 = self.iter.next().unwrap().get_number()?;
                    let flt = format!("{}.{}", n, n2).parse::<f32>()?;

                    Expression::Float(flt)
                } else {
                    Expression::Number(n)
                }
            }

            TokenType::Operator(op) => match op {
                Operator::Gt => {
                    let last = self.exprs.pop().unwrap();

                    if self
                        .iter
                        .next_is_peek(TokenType::Operator(Operator::Equals), 0)
                        .is_ok()
                    {
                        self.iter.next();

                        Expression::Operation(Operation::GtEq(
                            Box::new(last),
                            Box::new(self.parse_one()?),
                        ))
                    } else {
                        Expression::Operation(Operation::Gt(
                            Box::new(last),
                            Box::new(self.parse_one()?),
                        ))
                    }
                }

                _ => Expression::None,
            },

            TokenType::None => Expression::None,
        })
    }
}
