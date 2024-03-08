use qsc_ast::ast::{decl::func::FunctionNode, AbstractTree};

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessorContext {
    pub func: Option<FunctionNode>,
    pub tree: AbstractTree,
}

impl ProcessorContext {
    pub fn new(tree: AbstractTree) -> Self {
        Self { tree, func: None }
    }
}
