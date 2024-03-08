use crate::{
    lexer::{Lexer, Result},
    parser::Rule,
};
use pest::iterators::Pair;
use qsc_ast::ast::{
    decl::func::FunctionNode,
    node::{block::Block, vis::Visibility},
};

impl<'i> Lexer {
    pub fn function(&self, pair: Pair<'i, Rule>) -> Result<FunctionNode> {
        let mut inner = pair.clone().into_inner();
        let name = inner.next().unwrap().as_str().trim().to_string();

        let args = if inner.peek().map(|v| v.as_rule()) == Some(Rule::params) {
            self.params(inner.next().unwrap())?
        } else {
            Vec::new()
        };

        let ret = if inner.peek().map(|v| v.as_rule()) == Some(Rule::r#type) {
            Some(self.ty(inner.next().unwrap())?)
        } else {
            None
        };

        let body_pair = inner.next().unwrap();

        let body = Block {
            span: body_pair.as_span().into(),
            data: body_pair
                .into_inner()
                .map(|pair| self.parse(pair).unwrap())
                .collect(),
        };

        Ok(FunctionNode {
            span: pair.as_span().into(),
            name,
            args,
            content: body,
            ret,
            vis: Visibility::Public,
        })
    }
}
