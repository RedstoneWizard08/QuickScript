use qsc_core::error::Result;

use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

use self::{boolean::BoolNode, char::CharNode, float::FloatNode, int::IntNode, string::StringNode};

pub mod boolean;
pub mod char;
pub mod float;
pub mod int;
pub mod string;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralNode {
    Char(CharNode),
    Float(FloatNode),
    Int(IntNode),
    String(StringNode),
    Bool(BoolNode),
}

impl LiteralNode {
    pub fn get_type(&self) -> Result<String> {
        Ok(match self.clone() {
            LiteralNode::Bool(_) => "bool",
            LiteralNode::Char(_) => "char",
            LiteralNode::Float(_) => "f32",
            LiteralNode::Int(_) => "i32",
            LiteralNode::String(_) => "str",
        }
        .to_string())
    }
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
