use pest::iterators::Pair;
use qsc_ast::ast::stmt::call::{CallArgument, CallNode};

use crate::{
    lexer::{Lexer, Result},
    parser::Rule,
};

impl<'i> Lexer<'i> {
    pub fn call(&'i self, pair: &Pair<'i, Rule>) -> Result<CallNode<'i>> {
        let mut inner = pair.clone().into_inner();
        let func = inner.next().unwrap().as_str().trim();

        let args = inner
            .next()
            .map(|pair| {
                pair.into_inner()
                    .map(|pair| self.call_arg(pair).unwrap())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Ok(CallNode {
            span: pair.as_span(),
            func,
            args,
        })
    }

    pub fn call_arg(&'i self, pair: Pair<'i, Rule>) -> Result<CallArgument<'i>> {
        Ok(CallArgument {
            span: pair.as_span(),
            value: self.parse(pair)?,
        })
    }
}
