use super::Node;
use crate::{ast::decl::var::VariableNode, span::StaticSpan};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub span: StaticSpan,
    pub data: Vec<Node>,
}

impl Block {
    pub fn vars(&self) -> Vec<VariableNode> {
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
