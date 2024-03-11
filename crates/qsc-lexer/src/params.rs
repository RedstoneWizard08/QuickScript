use pest::iterators::Pair;
use qsc_ast::ast::decl::func::FunctionArgument;
use qsc_core::error::Result;

use crate::{lexer::Lexer, parser::Rule};

impl<'i> Lexer {
    pub fn params(&self, pair: Pair<'i, Rule>) -> Result<Vec<FunctionArgument>> {
        let mut args = Vec::new();

        for pair in pair.into_inner() {
            args.push(self.param(pair)?);
        }

        Ok(args)
    }

    pub fn param(&self, pair: Pair<'i, Rule>) -> Result<FunctionArgument> {
        let mut inner = pair.clone().into_inner();
        let mutable = inner.peek().unwrap().as_str().trim() == "mut";

        if mutable {
            inner.next();
        }

        let name = inner.next().unwrap().as_str().trim().to_string();
        let type_ = self.ty(inner.next().unwrap())?;

        Ok(FunctionArgument {
            span: pair.as_span().into(),
            name,
            type_,
            mutable,
        })
    }
}
