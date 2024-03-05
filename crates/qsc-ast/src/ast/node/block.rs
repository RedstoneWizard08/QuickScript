use super::Node;
use crate::ast::decl::var::VariableNode;
use pest::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Block<'i> {
    pub span: Span<'i>,
    pub data: Vec<Node<'i>>,
}

impl<'i> Block<'i> {
    pub fn vars(&self) -> Vec<VariableNode<'i>> {
        let mut vars = Vec::new();

        for node in &self.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(var) = decl.as_variable() {
                    vars.push(var);
                }
            }
        }

        vars
    }
}
