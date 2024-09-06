use crate::{
    block::Block, call::Call, cond::Conditional, func::Function, get_enum_variant_value_impl,
    is_enum_variant_impl, is_enum_variant_no_field_impl, literal::Literal, op::Operation,
    var::Variable,
};

pub type Return = Option<Box<Expr>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Literal(Literal),
    Name(String),
    Variable(Variable),
    Function(Function),
    Conditional(Conditional),
    Operation(Operation),
    Call(Call),
    Block(Block),
    Return(Return),
    Error,
}

is_enum_variant_impl!(is_literal -> Expr::Literal);
is_enum_variant_impl!(is_name -> Expr::Name);
is_enum_variant_impl!(is_var -> Expr::Variable);
is_enum_variant_impl!(is_func -> Expr::Function);
is_enum_variant_impl!(is_cond -> Expr::Conditional);
is_enum_variant_impl!(is_op -> Expr::Operation);
is_enum_variant_impl!(is_call -> Expr::Call);
is_enum_variant_impl!(is_block -> Expr::Block);
is_enum_variant_impl!(is_return -> Expr::Return);
is_enum_variant_no_field_impl!(is_error -> Expr::Error);

get_enum_variant_value_impl!(get_literal -> Expr::Literal: Literal);
get_enum_variant_value_impl!(get_name -> Expr::Name: String);
get_enum_variant_value_impl!(get_var -> Expr::Variable: Variable);
get_enum_variant_value_impl!(get_func -> Expr::Function: Function);
get_enum_variant_value_impl!(get_cond -> Expr::Conditional: Conditional);
get_enum_variant_value_impl!(get_op -> Expr::Operation: Operation);
get_enum_variant_value_impl!(get_call -> Expr::Call: Call);
get_enum_variant_value_impl!(get_block -> Expr::Block: Block);
get_enum_variant_value_impl!(get_return -> Expr::Return: Return);
