use qsc_ast::ast::{decl::func::FunctionNode, AbstractTree};

#[derive(Debug, PartialEq)]
pub struct ProcessorContext<'a> {
    pub func: Option<FunctionNode>,
    pub tree: &'a mut AbstractTree,
}

impl<'a> ProcessorContext<'a> {
    pub fn new(tree: &'a mut AbstractTree) -> Self {
        Self { tree, func: None }
    }
}
