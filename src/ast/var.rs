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
    pub fn parse_let(&mut self) -> Result<Expression> {
        let name = self
            .iter
            .next_result("Could not read variable name!")?
            .get_name()?;

        let type_;

        if self
            .iter
            .next_is(TokenType::Operator(Operator::Colon))
            .is_ok()
        {
            let type_content = self.iter.read_until(TokenType::Operator(Operator::Equals));

            type_ = Type::parse(type_content)?;
        } else {
            type_ = Type::parse(vec![])?;
        }

        let value = self
            .iter
            .read_until(TokenType::Operator(Operator::Semicolon));

        let mut parser = AstParser::new(value);

        parser.parse()?;

        let value = parser.exprs.get(0).unwrap();

        Ok(Expression::Define(Definition::Variable(
            name,
            type_,
            Box::new(value.clone()),
        )))
    }
}
