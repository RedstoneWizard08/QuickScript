use qsc_core::{conv::IntoSourceSpan, error::lexical::LexicalError};

use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

use self::{binary::BinaryExpr, unary::UnaryExpr};

use super::AbstractTree;

pub mod binary;
pub mod operator;
pub mod unary;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpressionNode {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
}

impl ExpressionNode {
    pub fn get_type(&self, func: &Option<String>, tree: &AbstractTree) -> Option<String> {
        match self.clone() {
            ExpressionNode::Binary(expr) => {
                let lhs = expr.lhs.data.get_type(func, tree);
                let rhs = expr.rhs.data.get_type(func, tree);

                if lhs.is_none() && rhs.is_some() {
                    rhs
                } else if rhs.is_none() && lhs.is_some() {
                    lhs
                } else if rhs.is_some() && lhs.is_some() {
                    if rhs.unwrap() != lhs.clone().unwrap() {
                        let err = LexicalError {
                            location: expr.span.into_source_span(),
                            src: tree.src.clone().into(),
                            error: miette!("Left and right operands' types do not match!"),
                        };

                        Result::<(), _>::Err(err).unwrap();
                        unreachable!();
                    } else {
                        Some(lhs.unwrap())
                    }
                } else {
                    None
                }
            }

            ExpressionNode::Unary(expr) => expr.value.data.get_type(func, tree),
        }
    }
}

is_enum_variant_impl!(is_binary -> ExpressionNode::Binary);
is_enum_variant_impl!(is_unary -> ExpressionNode::Unary);

get_enum_variant_value_impl!(as_binary -> ExpressionNode::Binary: BinaryExpr);
get_enum_variant_value_impl!(as_unary -> ExpressionNode::Unary: UnaryExpr);
