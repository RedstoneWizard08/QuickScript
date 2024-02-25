use crate::parser::{Lexer, Rule};
use pest::iterators::Pair;
use qsc_ast::{expr::ExprKind, func::Function};

impl Lexer {
    pub fn function<'i>(&self, pair: &Pair<'i, Rule>) -> ExprKind {
        let mut inner = pair.clone().into_inner();
        let name = inner.next().unwrap().as_str().trim().to_string();

        let args = if inner.peek().map(|v| v.as_rule()) == Some(Rule::params) {
            self.params(inner.next().unwrap())
        } else {
            Vec::new()
        };

        let return_type = if inner.peek().map(|v| v.as_rule()) == Some(Rule::r#type) {
            inner.next().unwrap().as_str().trim().to_string()
        } else {
            "void".to_string()
        };

        let body = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|pair| self.parse_expr(pair))
            .collect();

        ExprKind::Function(Function {
            name,
            args,
            body: Box::new(body),
            return_type,
        })
    }
}
