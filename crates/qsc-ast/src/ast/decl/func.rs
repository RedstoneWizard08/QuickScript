use super::var::VariableNode;
use crate::{
    ast::node::{block::Block, ty::TypeNode, vis::Visibility},
    span::StaticSpan,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub name: String,
    pub args: Vec<FunctionArgument>,
    pub ret: Option<TypeNode>,
    pub content: Block,
    pub vis: Visibility,
}

impl FunctionNode {
    pub fn variables(&self) -> HashMap<String, VariableNode> {
        let mut vars = HashMap::new();

        for node in &self.content.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(var) = decl.as_variable() {
                    vars.insert(var.name.clone(), var);
                }
            }
        }

        vars
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionArgument {
    #[serde(skip)]
    pub span: StaticSpan,
    pub mutable: bool,
    pub name: String,
    pub type_: TypeNode,
}

impl Into<VariableNode> for FunctionArgument {
    fn into(self) -> VariableNode {
        VariableNode {
            span: self.span,
            mutable: self.mutable,
            name: self.name,
            type_: Some(self.type_),
            value: None,
        }
    }
}
