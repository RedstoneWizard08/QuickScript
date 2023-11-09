use anyhow::Result;

use crate::{
    tokenizer::token::{operator::Operator, ttype::TokenType},
    types::Type,
};

use super::{
    expr::{Definition, Expression},
    AstParser,
};

impl AstParser {
    pub fn parse_fn(&mut self) -> Result<Expression> {
        let name = self
            .iter
            .next_result("Could not read a function name!")?
            .get_name()?;

        self.iter
            .next_is(TokenType::Operator(Operator::OpenParens))?;

        let args = self.iter.read_until_counted(
            TokenType::Operator(Operator::OpenParens),
            TokenType::Operator(Operator::CloseParens),
        );

        let args = AstParser::parse_args(args)?;

        let mut ret_type = vec![
            TokenType::Operator(Operator::OpenParens),
            TokenType::Operator(Operator::CloseParens),
        ];

        if self
            .iter
            .next_is_peek(TokenType::Operator(Operator::Subtract), 0)
            .is_ok()
            && self
                .iter
                .next_is_peek(TokenType::Operator(Operator::Gt), 1)
                .is_ok()
        {
            // Skip 2 to skip the return type symbol
            self.iter.next(); // -
            self.iter.next(); // >

            ret_type = self
                .iter
                .read_until(TokenType::Operator(Operator::OpenCurly));
        } else {
            self.iter
                .next_is(TokenType::Operator(Operator::OpenCurly))?;
        }

        let ret_type = Type::parse(ret_type)?;

        let content = self.iter.read_until_counted(
            TokenType::Operator(Operator::OpenCurly),
            TokenType::Operator(Operator::CloseCurly),
        );

        let mut parser = AstParser::new(content);

        parser.parse()?;

        let content = parser
            .exprs
            .iter()
            .map(|v| Box::new(v.clone()))
            .collect::<Vec<Box<Expression>>>();

        let args = args
            .iter()
            .map(|(n, t)| Box::new(Definition::Argument(n.clone(), t.clone())))
            .collect::<Vec<Box<Definition>>>();

        Ok(Expression::Define(Definition::Function(
            name, args, ret_type, content,
        )))
    }
}
