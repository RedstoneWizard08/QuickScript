use anyhow::Result;

use crate::tokenizer::token::{operator::Operator, ttype::TokenType};

use super::{expr::Expression, AstParser};

impl AstParser {
    #[allow(unused)]
    pub fn parse_if(&mut self) -> Result<Expression> {
        self.iter
            .next_is(TokenType::Operator(Operator::OpenParens))?;

        let cond = self.iter.read_until_counted(
            TokenType::Operator(Operator::OpenParens),
            TokenType::Operator(Operator::CloseParens),
        );

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

        let mut parser = AstParser::new(cond);

        parser.parse()?;

        let cond = parser
            .exprs
            .iter()
            .map(|v| Box::new(v.clone()))
            .collect::<Vec<Box<Expression>>>();

        Ok(Expression::If(cond, content))
    }
}
