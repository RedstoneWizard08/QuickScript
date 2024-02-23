use pest::iterators::Pair;
use qsc_ast::{expr::ExprKind, var::Variable};

use crate::parser::{Lexer, Rule};

impl Lexer {
    pub fn var<'i>(&self, pair: &Pair<'i, Rule>) -> ExprKind {
        let mut inner = pair.clone().into_inner();

        let is_mutable = inner
            .peek()
            .map(|pair| pair.as_str().trim() == "mut")
            .unwrap_or(false);

        if is_mutable {
            inner.next();
        }

        let name = inner.next().unwrap().as_str().trim().to_string();

        let type_ = if inner
            .peek()
            .map(|pair| pair.as_str().trim() == ":")
            .unwrap_or(false)
        {
            inner.next();
            inner.next().unwrap().as_str().trim().to_string()
        } else {
            String::new()
        };

        let value = inner.next().map(|pair| Box::new(self.parse_expr(pair)));

        ExprKind::Variable(Variable {
            name,
            type_,
            value,
            is_mutable,
        })
    }
}
