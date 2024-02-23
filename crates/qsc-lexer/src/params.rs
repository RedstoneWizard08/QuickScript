use pest::iterators::Pair;
use qsc_ast::func::FunctionArg;

use crate::parser::{Lexer, Rule};

impl Lexer {
    pub fn params<'i>(&self, pair: Pair<'i, Rule>) -> Vec<FunctionArg> {
        pair.into_inner().map(|pair| self.param(pair)).collect()
    }

    pub fn param<'i>(&self, pair: Pair<'i, Rule>) -> FunctionArg {
        let mut inner = pair.into_inner();

        let is_mutable = inner.peek().unwrap().as_str().trim() == "mut";

        if is_mutable {
            inner.next();
        }

        let name = inner.next().unwrap().as_str().trim().to_string();
        let type_ = inner.next().unwrap().as_str().trim().to_string();

        FunctionArg {
            name,
            type_,
            is_mutable,
        }
    }
}
