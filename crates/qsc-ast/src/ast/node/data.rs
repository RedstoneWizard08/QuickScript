use crate::{
    ast::{decl::DeclarationNode, expr::ExpressionNode, literal::LiteralNode, stmt::StatementNode},
    get_enum_variant_value_impl, is_enum_variant_impl,
};

use super::{block::Block, sym::SymbolNode, ty::TypeNode};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeData<'i> {
    Expr(ExpressionNode<'i>),
    Literal(LiteralNode<'i>),
    Declaration(DeclarationNode<'i>),
    Statement(StatementNode<'i>),
    Block(Block<'i>),
    Symbol(SymbolNode<'i>),
    Type(TypeNode<'i>),
}

is_enum_variant_impl!(is_expr -> NodeData::Expr);
is_enum_variant_impl!(is_literal -> NodeData::Literal);
is_enum_variant_impl!(is_decl -> NodeData::Declaration);
is_enum_variant_impl!(is_stmt -> NodeData::Statement);
is_enum_variant_impl!(is_block -> NodeData::Block);
is_enum_variant_impl!(is_symbol -> NodeData::Symbol);
is_enum_variant_impl!(is_type -> NodeData::Type);

get_enum_variant_value_impl!(as_expr -> NodeData::Expr: ExpressionNode);
get_enum_variant_value_impl!(as_literal -> NodeData::Literal: LiteralNode);
get_enum_variant_value_impl!(as_decl -> NodeData::Declaration: DeclarationNode);
get_enum_variant_value_impl!(as_stmt -> NodeData::Statement: StatementNode);
get_enum_variant_value_impl!(as_block -> NodeData::Block: Block);
get_enum_variant_value_impl!(as_symbol -> NodeData::Symbol: SymbolNode);
get_enum_variant_value_impl!(as_type -> NodeData::Type: TypeNode);
