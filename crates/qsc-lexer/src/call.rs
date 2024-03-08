use pest::iterators::Pair;
use qsc_ast::ast::stmt::call::{CallArgument, CallNode};

use crate::{
    lexer::{Lexer, Result},
    parser::Rule,
};

impl<'i> Lexer {
    pub fn call(&self, pair: Pair<'i, Rule>) -> Result<CallNode> {
        let mut inner = pair.clone().into_inner();
        let func = inner.next().unwrap().as_str().trim().to_string();

        let args = inner
            .next()
            .map(|pair| {
                pair.into_inner()
                    .map(|pair| self.call_arg(pair).unwrap())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Ok(CallNode {
            span: pair.as_span().into(),
            func,
            args,
        })
    }

    pub fn call_arg(&self, pair: Pair<'i, Rule>) -> Result<CallArgument> {
        Ok(CallArgument {
            span: pair.as_span().into(),
            value: self.parse(pair)?,
        })
    }
}
