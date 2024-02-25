use pest::iterators::Pair;
use qsc_ast::{call::Call, expr::ExprKind};

use crate::parser::{Lexer, Rule};

impl Lexer {
    pub fn call<'i>(&self, pair: &Pair<'i, Rule>) -> ExprKind {
        let mut inner = pair.clone().into_inner();
        let name = inner.next().unwrap().as_str().trim().to_string();

        let args = inner
            .next()
            .map(|pair| {
                pair.into_inner()
                    .map(|pair| self.parse_expr(pair))
                    .filter(|v| v.content != ExprKind::None)
                    .collect()
            })
            .unwrap_or_default();

        ExprKind::Call(Call { name, args })
    }
}
