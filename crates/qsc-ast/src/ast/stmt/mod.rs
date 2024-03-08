use crate::{get_enum_variant_value_impl, is_enum_variant_impl};

use self::{call::CallNode, ret::ReturnNode};

use super::AbstractTree;

pub mod call;
pub mod ret;

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    Call(CallNode),
    Return(ReturnNode),
}

impl StatementNode {
    pub fn get_type(&self, _func: &Option<String>, tree: &AbstractTree) -> Option<String> {
        let funcs = tree.functions();

        match self.clone() {
            Self::Call(call) => {
                if let Some(func) = funcs.get(&call.func) {
                    func.ret.clone().map(|v| v.as_str())
                } else {
                    None
                }
            }

            Self::Return(_) => None,
        }
    }
}

is_enum_variant_impl!(is_call -> StatementNode::Call);
is_enum_variant_impl!(is_return -> StatementNode::Return);

get_enum_variant_value_impl!(as_call -> StatementNode::Call: CallNode);
get_enum_variant_value_impl!(as_return -> StatementNode::Return: ReturnNode);
