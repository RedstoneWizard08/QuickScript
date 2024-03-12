use super::func::FunctionArgument;
use crate::{
    ast::node::{ty::TypeNode, vis::Visibility},
    span::StaticSpan,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternFunctionNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub name: String,
    pub args: Vec<FunctionArgument>,
    pub ret: Option<TypeNode>,
    pub vis: Visibility,
}
