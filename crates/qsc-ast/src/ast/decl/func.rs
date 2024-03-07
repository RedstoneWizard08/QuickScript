use pest::Span;

use crate::ast::node::{block::Block, ty::TypeNode, vis::Visibility};

use super::var::VariableNode;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionNode<'i> {
    pub span: Span<'i>,
    pub name: &'i str,
    pub args: Vec<FunctionArgument<'i>>,
    pub ret: Option<TypeNode<'i>>,
    pub content: Block<'i>,
    pub vis: Visibility,
}

impl<'i> FunctionNode<'i> {
    pub fn vars(&self) -> Vec<VariableNode<'i>> {
        let mut vars = Vec::new();

        for node in &self.content.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(var) = decl.as_variable() {
                    vars.push(var);
                }
            }
        }

        vars
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionArgument<'i> {
    pub span: Span<'i>,
    pub mutable: bool,
    pub name: &'i str,
    pub type_: TypeNode<'i>,
}

impl<'i> Into<VariableNode<'i>> for FunctionArgument<'i> {
    fn into(self) -> VariableNode<'i> {
        VariableNode {
            span: self.span,
            mutable: self.mutable,
            name: self.name.to_string(),
            type_: Some(self.type_),
            value: None,
        }
    }
}
