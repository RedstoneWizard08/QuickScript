use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

use self::{call::CallNode, ret::ReturnNode};

pub mod call;
pub mod ret;

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode<'i> {
    Call(CallNode<'i>),
    Return(ReturnNode<'i>),
}

is_enum_variant_impl!(is_call -> StatementNode::Call);
is_enum_variant_impl!(is_return -> StatementNode::Return);

get_enum_variant_value_impl!(as_call -> StatementNode::Call: CallNode);
get_enum_variant_value_impl!(as_return -> StatementNode::Return: ReturnNode);
