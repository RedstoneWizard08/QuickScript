pub mod decl;
pub mod expr;
pub mod literal;
pub mod node;
pub mod stmt;

use self::{
    decl::{func::FunctionNode, global::GlobalVariable},
    node::Node,
};

use pest::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractTree<'i> {
    pub span: Span<'i>,
    pub source: &'i str,
    pub data: Vec<Node<'i>>,
}

impl<'i> AbstractTree<'i> {
    pub fn new(source: &'i str) -> Self {
        Self {
            span: Span::new(source, 0, source.len()).unwrap(),
            source,
            data: Vec::new(),
        }
    }

    pub fn functions(&self) -> Vec<FunctionNode<'i>> {
        let mut funcs = Vec::new();

        for node in &self.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(func) = decl.as_function() {
                    funcs.push(func);
                }
            }
        }

        funcs
    }

    pub fn globals(&self) -> Vec<GlobalVariable<'i>> {
        let mut globals = Vec::new();

        for node in &self.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(global) = decl.as_global() {
                    globals.push(global);
                }
            }
        }

        globals
    }
}
