use pest::iterators::Pair;
use qsc_ast::ast::stmt::cond::ConditionalNode;
use qsc_core::error::Result;

use crate::{lexer::Lexer, parser::Rule};

impl<'i> Lexer {
    pub fn condition(&self, pair: Pair<'i, Rule>) -> Result<ConditionalNode> {
        let mut inner = pair.clone().into_inner();
        let condition = self.parse(inner.next().unwrap())?;
        let block = self.parse_data(inner.next().unwrap())?.as_block()?;

        Ok(ConditionalNode {
            span: pair.as_span().into(),
            condition,
            block,
        })
    }
}
