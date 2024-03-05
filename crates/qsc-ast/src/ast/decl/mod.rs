use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

use self::{func::FunctionNode, global::GlobalVariable, var::VariableNode};

pub mod func;
pub mod global;
pub mod var;

#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationNode<'i> {
    Function(FunctionNode<'i>),
    Variable(VariableNode<'i>),
    Global(GlobalVariable<'i>),
}

is_enum_variant_impl!(is_function -> DeclarationNode::Function);
is_enum_variant_impl!(is_variable -> DeclarationNode::Variable);
is_enum_variant_impl!(is_global -> DeclarationNode::Global);

get_enum_variant_value_impl!(as_function -> DeclarationNode::Function: FunctionNode);
get_enum_variant_value_impl!(as_variable -> DeclarationNode::Variable: VariableNode);
get_enum_variant_value_impl!(as_global -> DeclarationNode::Global: GlobalVariable);
