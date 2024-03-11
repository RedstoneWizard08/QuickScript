use pest::iterators::Pair;
use qsc_ast::ast::decl::var::VariableNode;
use qsc_core::error::Result;

use crate::{lexer::Lexer, parser::Rule};

impl<'i> Lexer {
    pub fn var(&self, pair: Pair<'i, Rule>) -> Result<VariableNode> {
        let mut inner = pair.clone().into_inner();

        let mutable = inner
            .peek()
            .map(|pair| pair.as_str().trim() == "mut")
            .unwrap_or(false);

        if mutable {
            inner.next();
        }

        let name = inner.next().unwrap().as_str().trim().to_string();

        let type_ = if inner
            .peek()
            .map(|pair| pair.as_rule() == Rule::r#type)
            .unwrap_or(false)
        {
            Some(self.ty(inner.next().unwrap())?)
        } else {
            None
        };

        let value = inner.next().map(|pair| self.parse(pair).unwrap());

        Ok(VariableNode {
            span: pair.as_span().into(),
            name,
            type_,
            value,
            mutable,
        })
    }
}
