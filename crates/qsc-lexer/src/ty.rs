use pest::iterators::Pair;
use qsc_ast::ast::node::ty::TypeNode;

use crate::{
    lexer::{Lexer, Result},
    parser::Rule,
};

impl<'i> Lexer {
    pub fn ty(&self, pair: Pair<'i, Rule>) -> Result<TypeNode> {
        let mut inner = pair.clone().into_inner();
        let name = inner.next().unwrap().as_str().trim();
        let mut generics = Vec::new();

        if let Some(pair) = inner.next() {
            for pair in pair.into_inner() {
                let pair = self.parse(pair)?;

                if let Ok(sym) = pair.data.as_symbol() {
                    generics.push(sym);
                }
            }
        }

        Ok(TypeNode {
            span: pair.as_span().into(),
            name: name.to_string(),
            generics,
        })
    }
}
