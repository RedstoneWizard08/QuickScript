use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
}

is_enum_variant_impl!(is_string -> Literal::String);
is_enum_variant_impl!(is_int -> Literal::Int);
is_enum_variant_impl!(is_float -> Literal::Float);
is_enum_variant_impl!(is_bool -> Literal::Bool);
is_enum_variant_impl!(is_char -> Literal::Char);

get_enum_variant_value_impl!(get_string -> Literal::String: String);
get_enum_variant_value_impl!(get_int -> Literal::Int: i64);
get_enum_variant_value_impl!(get_float -> Literal::Float: f64);
get_enum_variant_value_impl!(get_bool -> Literal::Bool: bool);
get_enum_variant_value_impl!(get_char -> Literal::Char: char);
