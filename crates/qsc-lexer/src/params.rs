use pest::iterators::Pair;
use qsc_ast::ast::decl::func::FunctionArgument;

use crate::{
    lexer::{Lexer, Result},
    parser::Rule,
};

impl<'i> Lexer<'i> {
    pub fn params(&'i self, pair: Pair<'i, Rule>) -> Vec<FunctionArgument<'i>> {
        pair.into_inner()
            .map(|pair| self.param(&pair).unwrap())
            .collect()
    }

    pub fn param(&self, pair: &Pair<'i, Rule>) -> Result<FunctionArgument> {
        let mut inner = pair.clone().into_inner();

        let mutable = inner.peek().unwrap().as_str().trim() == "mut";

        if mutable {
            inner.next();
        }

        let name = inner.next().unwrap().as_str().trim();
        let type_ = self.ty(&inner.next().unwrap())?;

        Ok(FunctionArgument {
            span: pair.as_span(),
            name,
            type_,
            mutable,
        })
    }
}
