use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

use self::{boolean::BoolNode, char::CharNode, float::FloatNode, int::IntNode, string::StringNode};

pub mod boolean;
pub mod char;
pub mod float;
pub mod int;
pub mod string;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralNode<'i> {
    Char(CharNode<'i>),
    Float(FloatNode<'i>),
    Int(IntNode<'i>),
    String(StringNode<'i>),
    Bool(BoolNode<'i>),
}

is_enum_variant_impl!(is_char -> LiteralNode::Char);
is_enum_variant_impl!(is_float -> LiteralNode::Float);
is_enum_variant_impl!(is_int -> LiteralNode::Int);
is_enum_variant_impl!(is_string -> LiteralNode::String);
is_enum_variant_impl!(is_bool -> LiteralNode::Bool);

get_enum_variant_value_impl!(as_char -> LiteralNode::Char: CharNode);
get_enum_variant_value_impl!(as_float -> LiteralNode::Float: FloatNode);
get_enum_variant_value_impl!(as_int -> LiteralNode::Int: IntNode);
get_enum_variant_value_impl!(as_string -> LiteralNode::String: StringNode);
get_enum_variant_value_impl!(as_bool -> LiteralNode::Bool: BoolNode);
