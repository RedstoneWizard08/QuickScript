use pest::iterators::Pair;
use qsc_ast::ast::node::{sym::SymbolNode, ty::TypeNode};

use crate::{
    lexer::{Lexer, Result},
    parser::Rule,
};

impl<'i> Lexer<'i> {
    pub fn ty(&'i self, pair: &Pair<'i, Rule>) -> Result<TypeNode> {
        let mut inner = pair.clone().into_inner();
        let name = inner.next().unwrap().as_str().trim();

        let generics = inner
            .next()
            .map(|pair| {
                pair.into_inner()
                    .map(|pair| self.parse(pair).unwrap())
                    .filter(|v| v.data.is_symbol())
                    .map(|v| v.data.as_symbol().unwrap())
                    .collect::<Vec<SymbolNode<'i>>>()
            })
            .unwrap_or(Vec::new());

        Ok(TypeNode {
            span: pair.as_span(),
            name,
            generics,
        })
    }
}
