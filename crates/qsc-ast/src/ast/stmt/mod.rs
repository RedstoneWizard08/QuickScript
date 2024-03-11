use qsc_core::{
    conv::IntoSourceSpan,
    error::{lexical::LexicalError, Result},
};

use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

use self::{call::CallNode, cond::ConditionalNode, ret::ReturnNode};

use super::AbstractTree;

pub mod call;
pub mod cond;
pub mod ret;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatementNode {
    Call(CallNode),
    Return(ReturnNode),
    Condition(ConditionalNode),
}

impl StatementNode {
    pub fn get_type(&self, _func: &Option<String>, tree: &AbstractTree) -> Result<String> {
        let funcs = tree.functions();

        match self.clone() {
            Self::Call(call) => {
                if let Some(func) = funcs.get(&call.func) {
                    func.ret.clone().map(|v| v.as_str()).ok_or(
                        LexicalError {
                            location: call.span.into_source_span(),
                            src: tree.src.clone().into(),
                            error: miette!("Cannot find a return type for call!"),
                        }
                        .into(),
                    )
                } else {
                    Err(LexicalError {
                        location: call.span.into_source_span(),
                        src: tree.src.clone().into(),
                        error: miette!("Cannot find a return type for call!"),
                    }
                    .into())
                }
            }

            Self::Condition(cond) => Err(LexicalError {
                location: cond.span.into_source_span(),
                src: tree.src.clone().into(),
                error: miette!("Conditional return values are not currently supported!"),
            }
            .into()),

            Self::Return(ret) => Err(LexicalError {
                location: ret.span.into_source_span(),
                src: tree.src.clone().into(),
                error: miette!("Return types cannot have a type!"),
            }
            .into()),
        }
    }
}

is_enum_variant_impl!(is_call -> StatementNode::Call);
is_enum_variant_impl!(is_return -> StatementNode::Return);

get_enum_variant_value_impl!(as_call -> StatementNode::Call: CallNode);
get_enum_variant_value_impl!(as_return -> StatementNode::Return: ReturnNode);
