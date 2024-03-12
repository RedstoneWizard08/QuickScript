use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

use self::{
    external::ExternFunctionNode, func::FunctionNode, global::GlobalVariable, var::VariableNode,
};

pub mod external;
pub mod func;
pub mod global;
pub mod var;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeclarationNode {
    Function(FunctionNode),
    Variable(VariableNode),
    Global(GlobalVariable),
    Extern(ExternFunctionNode),
}

is_enum_variant_impl!(is_function -> DeclarationNode::Function);
is_enum_variant_impl!(is_variable -> DeclarationNode::Variable);
is_enum_variant_impl!(is_global -> DeclarationNode::Global);
is_enum_variant_impl!(is_extern -> DeclarationNode::Extern);

get_enum_variant_value_impl!(as_function -> DeclarationNode::Function: FunctionNode);
get_enum_variant_value_impl!(as_variable -> DeclarationNode::Variable: VariableNode);
get_enum_variant_value_impl!(as_global -> DeclarationNode::Global: GlobalVariable);
get_enum_variant_value_impl!(as_extern -> DeclarationNode::Extern: ExternFunctionNode);
