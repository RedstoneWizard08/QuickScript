use crate::{
    ast::{
        decl::DeclarationNode, expr::ExpressionNode, literal::LiteralNode, stmt::StatementNode,
        AbstractTree,
    },
    get_enum_variant_value_impl, is_enum_variant_impl, is_enum_variant_no_field_impl,
};

use super::{block::Block, sym::SymbolNode, ty::TypeNode};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeData {
    Expr(ExpressionNode),
    Literal(LiteralNode),
    Declaration(DeclarationNode),
    Statement(StatementNode),
    Block(Block),
    Symbol(SymbolNode),
    Type(TypeNode),
    EOI,
}

impl NodeData {
    pub fn get_type(&self, func: &Option<String>, tree: &AbstractTree) -> Option<String> {
        let globals = tree.globals();
        let funcs = tree.functions();

        match self.clone() {
            NodeData::Block(block) => block
                .data
                .last()
                .map(|v| v.data.get_type(func, tree))
                .flatten(),
            NodeData::EOI | NodeData::Declaration(_) => None,
            NodeData::Expr(expr) => expr.get_type(func, tree),
            NodeData::Literal(lit) => lit.get_type(),
            NodeData::Statement(stmt) => stmt.get_type(func, tree),
            NodeData::Type(ty) => Some(ty.as_str()),

            NodeData::Symbol(sym) => {
                if func.is_none() {
                    if let Some(var) = globals.get(&sym.value) {
                        Some(var.type_.as_str())
                    } else {
                        None
                    }
                } else {
                    let func = funcs.get(&func.clone().unwrap()).unwrap();
                    let vars = func.variables();

                    if let Some(var) = vars.get(&sym.value) {
                        var.type_.clone().map(|v| v.as_str())
                    } else if let Some(var) = globals.get(&sym.value) {
                        Some(var.type_.as_str())
                    } else {
                        None
                    }
                }
            }
        }
    }
}

is_enum_variant_impl!(is_expr -> NodeData::Expr);
is_enum_variant_impl!(is_literal -> NodeData::Literal);
is_enum_variant_impl!(is_decl -> NodeData::Declaration);
is_enum_variant_impl!(is_stmt -> NodeData::Statement);
is_enum_variant_impl!(is_block -> NodeData::Block);
is_enum_variant_impl!(is_symbol -> NodeData::Symbol);
is_enum_variant_impl!(is_type -> NodeData::Type);
is_enum_variant_no_field_impl!(is_eoi -> NodeData::EOI);

get_enum_variant_value_impl!(as_expr -> NodeData::Expr: ExpressionNode);
get_enum_variant_value_impl!(as_literal -> NodeData::Literal: LiteralNode);
get_enum_variant_value_impl!(as_decl -> NodeData::Declaration: DeclarationNode);
get_enum_variant_value_impl!(as_stmt -> NodeData::Statement: StatementNode);
get_enum_variant_value_impl!(as_block -> NodeData::Block: Block);
get_enum_variant_value_impl!(as_symbol -> NodeData::Symbol: SymbolNode);
get_enum_variant_value_impl!(as_type -> NodeData::Type: TypeNode);
