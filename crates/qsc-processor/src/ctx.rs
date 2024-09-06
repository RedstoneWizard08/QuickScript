use qsc_ast::ast::AbstractTree;

#[derive(Debug, Clone)]
pub struct ProcessorContext {
    pub func: Option<FunctionNode>,
    pub tree: AbstractTree,
}

impl ProcessorContext {
    pub fn new(tree: AbstractTree) -> Self {
        Self { tree, func: None }
    }
}
