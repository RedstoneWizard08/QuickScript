use pest::iterators::Pair;
use qsc_ast::expr::ExprKind;

use crate::parser::{Lexer, Rule};

impl Lexer {
    pub fn r#type<'i>(&self, pair: &Pair<'i, Rule>) -> ExprKind {
        let mut inner = pair.clone().into_inner();
        let name = inner.next().unwrap().as_str().trim().to_string();

        let generics = inner.next().map(|pair| {
            pair.into_inner()
                .map(|pair| self.parse_expr(pair))
                .filter(|v| v.content != ExprKind::None)
                .collect()
        });

        ExprKind::Type(name, generics)
    }
}
