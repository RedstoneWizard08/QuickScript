use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

use self::{binary::BinaryExpr, unary::UnaryExpr};

pub mod binary;
pub mod operator;
pub mod unary;

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode<'i> {
    Binary(BinaryExpr<'i>),
    Unary(UnaryExpr<'i>),
}

is_enum_variant_impl!(is_binary -> ExpressionNode::Binary);
is_enum_variant_impl!(is_unary -> ExpressionNode::Unary);

get_enum_variant_value_impl!(as_binary -> ExpressionNode::Binary: BinaryExpr);
get_enum_variant_value_impl!(as_unary -> ExpressionNode::Unary: UnaryExpr);
