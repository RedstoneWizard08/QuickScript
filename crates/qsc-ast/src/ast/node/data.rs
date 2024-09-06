use qsc_core::{
    conv::IntoSourceSpan,
    error::{lexical::LexicalError, Result},
};

use crate::{
    ast::{
        decl::DeclarationNode, expr::ExpressionNode, literal::LiteralNode, stmt::StatementNode,
        AbstractTree,
    },
    get_enum_variant_value_impl, is_enum_variant_impl, is_enum_variant_no_field_impl,
};

use super::{block::Block, sym::SymbolNode, ty::TypeNode};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub fn get_type(&self, func: &Option<String>, tree: &AbstractTree) -> Result<String> {
        let globals = tree.globals();
        let funcs = tree.functions();

        match self.clone() {
            NodeData::Block(block) => block
                .data
                .last()
                .map(|v| v.data.get_type(func, tree))
                .unwrap_or(Err(LexicalError {
                    location: block.span.into_source_span(),
                    src: tree.src.clone().into(),
                    error: miette!("Cannot get a return type for block!"),
                }.into())),

            NodeData::EOI | NodeData::Declaration(_) => Err(LexicalError {
                location: (0..0).into(),
                src: tree.src.clone().into(),
                error: miette!("EOI and Declarations do not support types. This error shouldn't be shown. This shouldn't be an issue."),
            }.into()),

            NodeData::Expr(expr) => expr.get_type(func, tree),
            NodeData::Literal(lit) => lit.get_type(),
            NodeData::Statement(stmt) => stmt.get_type(func, tree),
            NodeData::Type(ty) => Ok(ty.as_str()),

            NodeData::Symbol(sym) => {
                if func.is_none() {
                    if let Some(var) = globals.get(&sym.value) {
                        Ok(var.type_.as_str())
                    } else {
                        Err(LexicalError {
                            location: sym.span.into_source_span(),
                            src: tree.src.clone().into(),
                            error: miette!("Cannot find a type for symbol: {}", sym.value),
                        }.into())
                    }
                } else {
                    let funct = funcs.get(&func.clone().unwrap()).unwrap();
                    let vars = funct.variables();

                    if let Some(var) = vars.get(&sym.value) {
                        debug!("{:?}", var);

                        if let Some(ty) = &var.type_ {
                            Ok(ty.as_str())
                        } else if let Some(val) = &var.value {
                            val.data.get_type(func, tree)
                        } else {
                            Err(LexicalError {
                                location: var.span.into_source_span(),
                                src: tree.src.clone().into(),
                                error: miette!("Cannot find a type for symbol: {}", sym.value),
                            }.into())
                        }
                    } else if let Some(var) = globals.get(&sym.value) {
                        Ok(var.type_.as_str())
                    } else {
                        Err(LexicalError {
                            location: sym.span.into_source_span(),
                            src: tree.src.clone().into(),
                            error: miette!("Cannot find symbol: {}", sym.value),
                        }.into())
                    }
                }
            }
        }
    }

    pub fn is_int(&self, func: &Option<String>, tree: &AbstractTree) -> Result<bool> {
        if let Ok(lit) = self.as_literal() {
            Ok(lit.is_int())
        } else {
            Ok(["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"]
                .contains(&self.get_type(func, tree)?.as_str()))
        }
    }

    pub fn is_float(&self, func: &Option<String>, tree: &AbstractTree) -> Result<bool> {
        if let Ok(lit) = self.as_literal() {
            Ok(lit.is_float())
        } else {
            Ok(["f32", "f64"].contains(&self.get_type(func, tree)?.as_str()))
        }
    }

    pub fn is_char(&self, func: &Option<String>, tree: &AbstractTree) -> Result<bool> {
        if let Ok(lit) = self.as_literal() {
            Ok(lit.is_char())
        } else {
            Ok(self.get_type(func, tree)? == "char")
        }
    }

    pub fn is_str(&self, func: &Option<String>, tree: &AbstractTree) -> Result<bool> {
        if let Ok(lit) = self.as_literal() {
            Ok(lit.is_string())
        } else {
            Ok(self.get_type(func, tree)? == "str")
        }
    }

    pub fn is_bool(&self, func: &Option<String>, tree: &AbstractTree) -> Result<bool> {
        if let Ok(lit) = self.as_literal() {
            Ok(lit.is_bool())
        } else {
            Ok(self.get_type(func, tree)? == "bool")
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
